//! Wrapped Win32 functions to allocate RW virtual pages and change protection to RX, flush instruction cache, and drop accordingly
use core::ffi::c_void;
use core::ptr::{NonNull, null_mut};

use super::ExecMemError;
use std::io;

#[link(name = "kernel32")]
#[allow(non_snake_case)]
unsafe extern "system" {
    fn VirtualAlloc(
        lpAddress: *mut c_void, // ptr to base address of region to allocate
        dwsize: usize,          // size of region to allocate
        flAllocationType: u32,  // type of allocation
        flProtect: u32,         // mem protection for pages - RW
    ) -> *mut c_void; // returns base addr of allocated region

    fn VirtualProtect(
        lpAddress: *mut c_void,   // ptr to base address of pages
        dwsize: usize,            // size of region to change access in bytes
        flNewProtect: u32,        // new mem protection value
        lpflOldProtect: *mut u32, // pointer to var which receives value of old first page protection
    ) -> i32; // 0 on failure

    fn VirtualFree(
        lpAddress: *mut c_void, // ptr to base address of pages to free
        dwSize: usize,          // size of region to free in bytes -- must be 0 for MEM_RELEASE
        dwFreeType: u32,        // type of free
    ) -> i32; // 0 on failure

    fn FlushInstructionCache(
        hProcess: *mut c_void,  // handle to process to flush
        lpAddress: *mut c_void, // ptr to base of region to flush
        dwSize: usize,          // size of region to flush
    ) -> i32; // 0 on failure

    fn GetCurrentProcess() -> *mut c_void; // returns handle to current process
}

// alloc
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;

// protect
const PAGE_READWRITE: u32 = 0x04;
const PAGE_EXECUTE_READ: u32 = 0x20;

// free
const MEM_RELEASE: u32 = 0x8000;

/// reserve & commit RW memory
pub(super) fn alloc_rw(len: usize) -> Result<NonNull<c_void>, ExecMemError> {
    if len == 0 {
        return Err(ExecMemError::ZeroLength);
    }

    // use null_mut for lpaddress so that OS picks unused address
    let p = unsafe { VirtualAlloc(null_mut(), len, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) };
    NonNull::new(p).ok_or_else(|| ExecMemError::Alloc(io::Error::last_os_error()))
}

pub(super) unsafe fn protect_rx(ptr: NonNull<c_void>, len: usize) -> Result<(), ExecMemError> {
    let mut old = 0u32; // passing null for lpflOldProtect makes call fail
    if unsafe { VirtualProtect(ptr.as_ptr(), len, PAGE_EXECUTE_READ, &mut old) } == 0 {
        return Err(ExecMemError::Protect(io::Error::last_os_error()));
    }
    Ok(())
}

/// flush icache
pub(super) unsafe fn flush_icache(ptr: NonNull<c_void>, _len: usize) -> Result<(), ExecMemError> {
    let h_process = unsafe { GetCurrentProcess() };

    if unsafe { FlushInstructionCache(h_process, ptr.as_ptr(), 0) } == 0 {
        return Err(ExecMemError::Flush(io::Error::last_os_error()));
    }
    Ok(())
}

/// free target memory
/// return boolean as failures inside Drop can't be returned so result type is not necessary
pub(super) unsafe fn free(ptr: NonNull<c_void>, _len: usize) -> bool {
    if unsafe { VirtualFree(ptr.as_ptr(), 0, MEM_RELEASE) } == 0 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_length_is_rejected() {
        assert!(matches!(alloc_rw(0), Err(ExecMemError::ZeroLength)));
    }

    #[test]
    fn alloc_is_page_aligned_and_writable() {
        let p = alloc_rw(4096).expect("alloc failed");
        assert_eq!(p.as_ptr() as usize % 4096, 0);
        unsafe {
            let bytes = p.as_ptr() as *mut u8;
            bytes.write(0xAB);
            assert_eq!(bytes.read(), 0xAB);
            assert!(free(p, 0));
        }
    }

    /// write machine code in, change page to RX, call
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn round_trip_executes_code() {
        // mov eax, 42 ; ret
        const CODE: [u8; 6] = [0xB8, 0x2A, 0x00, 0x00, 0x00, 0xC3];

        let p = alloc_rw(4096).expect("alloc failed");
        unsafe {
            core::ptr::copy_nonoverlapping(CODE.as_ptr(), p.as_ptr() as *mut u8, CODE.len());
            protect_rx(p, 4096).expect("protect failed");
            flush_icache(p, 4096).expect("flush failed");

            let f: extern "C" fn() -> i32 = core::mem::transmute(p.as_ptr());
            assert_eq!(f(), 42);

            assert!(free(p, 0));
        }
    }
}
