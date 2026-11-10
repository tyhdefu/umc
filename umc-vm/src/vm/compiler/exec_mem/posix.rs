//! Wrapped Posix functions to allocate RW virtual pages and change protection to RX, flush instruction cache, and drop accordingly
use core::ffi::{c_char, c_void};
use core::ptr::{NonNull, null_mut};

use super::ExecMemError;
use std::io;

/// reserve & commit anonymous private memory
pub(super) fn alloc_rw(len: usize) -> Result<NonNull<c_void>, ExecMemError> {
    if len == 0 {
        return Err(ExecMemError::ZeroLength);
    }

    // fd = -1 and offset = 0 as not file-backed
    let p = unsafe {
        libc::mmap(
            null_mut(),
            len,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        )
    };

    // mmap signals failure with MAP_FAILED not null
    if p == libc::MAP_FAILED {
        return Err(ExecMemError::Alloc(io::Error::last_os_error()));
    }

    NonNull::new(p).ok_or_else(|| ExecMemError::Alloc(io::Error::other("mmap returned null")))
}

pub(super) unsafe fn protect_rx(ptr: NonNull<c_void>, len: usize) -> Result<(), ExecMemError> {
    if unsafe { libc::mprotect(ptr.as_ptr(), len, libc::PROT_READ | libc::PROT_EXEC) } != 0 {
        return Err(ExecMemError::Protect(io::Error::last_os_error()));
    }
    Ok(())
}

/// flush icache - needed for aarch
#[cfg(target_arch = "x86_64")]
pub(super) unsafe fn flush_icache(_ptr: NonNull<c_void>, _len: usize) -> Result<(), ExecMemError> {
    Ok(())
}
#[cfg(target_arch = "aarch64")]
pub(super) unsafe fn flush_icache(ptr: NonNull<c_void>, len: usize) -> Result<(), ExecMemError> {
    unsafe extern "C" {
        fn __clear_cache(start: *mut c_char, end: *mut c_char);
    }
    let p = ptr.as_ptr() as *mut c_char;
    unsafe { __clear_cache(p, p.add(len)) };
    Ok(())
}

pub(super) unsafe fn free(ptr: NonNull<c_void>, len: usize) -> bool {
    unsafe { libc::munmap(ptr.as_ptr(), len) == 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page_size() -> usize {
        unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
    }

    #[test]
    fn zero_length_is_rejected() {
        assert!(matches!(alloc_rw(0), Err(ExecMemError::ZeroLength)));
    }

    #[test]
    fn alloc_is_page_aligned_zeroed_and_writable() {
        let len = page_size();
        let p = alloc_rw(len).expect("alloc failed");
        assert_eq!(p.as_ptr() as usize % len, 0);
        unsafe {
            let bytes = p.as_ptr() as *mut u8;
            assert_eq!(bytes.read(), 0); // anonymous maps are zero-filled
            bytes.add(len - 1).write(0xAB); // last byte of the page is ours too
            assert_eq!(bytes.add(len - 1).read(), 0xAB);
            assert!(free(p, len));
        }
    }

    #[test]
    fn multi_page_alloc() {
        let len = page_size() * 4;
        let p = alloc_rw(len).expect("alloc failed");
        unsafe {
            (p.as_ptr() as *mut u8).add(len - 1).write(1);
            assert!(free(p, len));
        }
    }

    /// write machine code in, change page to RX, call
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn round_trip_executes_code() {
        // mov eax, 42 ; ret
        const CODE: [u8; 6] = [0xB8, 0x2A, 0x00, 0x00, 0x00, 0xC3];
        let len = page_size();

        let p = alloc_rw(len).expect("alloc failed");
        unsafe {
            core::ptr::copy_nonoverlapping(CODE.as_ptr(), p.as_ptr() as *mut u8, CODE.len());
            protect_rx(p, len).expect("protect failed");
            flush_icache(p, len).expect("flush failed");

            let f: extern "C" fn() -> i32 = core::mem::transmute(p.as_ptr());
            assert_eq!(f(), 42);

            assert!(free(p, len));
        }
    }
}
