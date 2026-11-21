use core::ffi::c_void;
use core::ptr::NonNull;

#[cfg(unix)]
mod posix;
#[cfg(unix)]
use posix as sys;

#[cfg(windows)]
mod win;
#[cfg(windows)]
use win as sys;

use crate::vm::compiler::bridge::CompiledFn;

/// hardcoded page size just for assurance
const PAGE_SIZE: usize = 4096;

#[derive(Debug)]
pub enum ExecMemError {
    Alloc(std::io::Error),
    Protect(std::io::Error),
    Flush(std::io::Error),
    ZeroLength,
}

/// owns a W^X region: RW when being filled, RX after commit
pub struct ExecPage {
    ptr: NonNull<c_void>,
    len: usize,
}

impl ExecPage {
    /// allocate RW -> copy -> protect RX -> flush
    pub fn commit(code: &[u8]) -> Result<Self, ExecMemError> {
        if code.is_empty() {
            return Err(ExecMemError::ZeroLength);
        }
        let len = code.len().next_multiple_of(PAGE_SIZE);
        let ptr = sys::alloc_rw(len)?;

        // take ownership immediately so Drop frees region if later step fails
        let page = ExecPage { ptr, len };
        unsafe {
            core::ptr::copy_nonoverlapping(code.as_ptr(), ptr.as_ptr().cast::<u8>(), code.len());
            sys::protect_rx(ptr, len)?;
            sys::flush_icache(ptr, len)?;
        }
        Ok(page)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn as_fn(&self) -> CompiledFn {
        unsafe { core::mem::transmute::<*mut c_void, CompiledFn>(self.ptr.as_ptr()) }
    }
}

impl Drop for ExecPage {
    fn drop(&mut self) {
        let freed = unsafe { sys::free(self.ptr, self.len) };
        debug_assert!(freed, "failed to free exec page");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_code_is_rejected() {
        assert!(matches!(
            ExecPage::commit(&[]),
            Err(ExecMemError::ZeroLength)
        ));
    }

    // commit tiny block and call through with CompiledFn struct
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn commit_and_call_returns_value() {
        // mov eax, 7 ; ret
        const CODE: [u8; 6] = [0xB8, 0x07, 0x00, 0x00, 0x00, 0xC3];

        let page = ExecPage::commit(&CODE).expect("commit failed");
        assert_eq!(page.len(), PAGE_SIZE);

        let mut frame = 0u64; // dummy frame
        let result = unsafe { page.as_fn()(&mut frame, std::ptr::null_mut()) };
        assert_eq!(result, 7);
    }

    // commit and drop pages in loop -- mem in task manager should stay flat if Drop works correclty
    // run with: cargo test commit_drop_does_not_leak -- --ignored --nocapture
    #[test]
    #[ignore]
    fn commit_drop_does_not_leak() {
        const CODE: [u8; 6] = [0xB8, 0x07, 0x00, 0x00, 0x00, 0xC3];

        for i in 0..100_000 {
            let page = ExecPage::commit(&CODE).expect("commit failed");
            drop(page);
            if i % 1_000 == 0 {
                println!("iteration {i}");
            }
        }
    }
}
