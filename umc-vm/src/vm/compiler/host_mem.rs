//! Memory operations compiled code calls back into the host for
//! Frame slots are passed by index, nonzero return hands instruction back to interpreter

use core::ffi::c_void;

use crate::vm::memory::safe::{SafeAddress, SafeMemoryManager};
use crate::vm::memory::{MemoryManager, Serializable};
use crate::vm::types::address::InstructionAddress;

// fixed size value moved through one frame slot
pub(crate) trait SlotPrim: Serializable + Copy {
    fn to_slot(self) -> u64;
    fn from_slot(bits: u64) -> Self;
}

impl SlotPrim for u8 {
    fn to_slot(self) -> u64 {
        self as u64
    }
    fn from_slot(bits: u64) -> Self {
        bits as u8
    }
}

impl SlotPrim for u16 {
    fn to_slot(self) -> u64 {
        self as u64
    }
    fn from_slot(bits: u64) -> Self {
        bits as u16
    }
}

impl SlotPrim for u32 {
    fn to_slot(self) -> u64 {
        self as u64
    }
    fn from_slot(bits: u64) -> Self {
        bits as u32
    }
}

impl SlotPrim for u64 {
    fn to_slot(self) -> u64 {
        self
    }
    fn from_slot(bits: u64) -> Self {
        bits
    }
}

impl SlotPrim for i8 {
    fn to_slot(self) -> u64 {
        self as i64 as u64
    }
    fn from_slot(bits: u64) -> Self {
        bits as i8
    }
}

impl SlotPrim for i16 {
    fn to_slot(self) -> u64 {
        self as i64 as u64
    }
    fn from_slot(bits: u64) -> Self {
        bits as i16
    }
}

impl SlotPrim for i32 {
    fn to_slot(self) -> u64 {
        self as i64 as u64
    }
    fn from_slot(bits: u64) -> Self {
        bits as i32
    }
}

impl SlotPrim for i64 {
    fn to_slot(self) -> u64 {
        self as u64
    }
    fn from_slot(bits: u64) -> Self {
        bits as i64
    }
}

impl SlotPrim for f32 {
    fn to_slot(self) -> u64 {
        self.to_bits() as u64
    }
    fn from_slot(bits: u64) -> Self {
        f32::from_bits(bits as u32)
    }
}

impl SlotPrim for f64 {
    fn to_slot(self) -> u64 {
        self.to_bits()
    }
    fn from_slot(bits: u64) -> Self {
        f64::from_bits(bits)
    }
}

impl SlotPrim for InstructionAddress {
    fn to_slot(self) -> u64 {
        self.pc() as u64
    }
    fn from_slot(bits: u64) -> Self {
        InstructionAddress::new(bits as usize)
    }
}

unsafe fn slot(frame: *mut u64, i: u64) -> *mut u64 {
    unsafe { frame.add(i as usize) }
}

unsafe fn addr_at(frame: *mut u64, i: u64) -> SafeAddress {
    unsafe { SafeAddress::from_parts(*slot(frame, i) as usize, *slot(frame, i + 1) as usize) }
}

unsafe fn write_addr(frame: *mut u64, i: u64, a: &SafeAddress) {
    unsafe {
        *slot(frame, i) = a.block_id() as u64;
        *slot(frame, i + 1) = a.byte_offset() as u64;
    }
}

// load primitive val from target address
pub(crate) unsafe extern "C" fn jit_load<T: SlotPrim>(
    mem: *mut c_void,
    frame: *mut u64,
    addr: u64,
    dst: u64,
) -> u32 {
    unsafe {
        let mem = &*(mem as *const SafeMemoryManager);
        match mem.load_prim::<T>(&addr_at(frame, addr)) {
            Ok(v) => {
                *slot(frame, dst) = v.to_slot();
                0
            }
            Err(_) => 1,
        }
    }
}

// store prim into target slot location in memory
pub(crate) unsafe extern "C" fn jit_store<T: SlotPrim>(
    mem: *mut c_void,
    frame: *mut u64,
    addr: u64,
    src: u64,
) -> u32 {
    unsafe {
        let mem = &mut *(mem as *mut SafeMemoryManager);
        mem.store_prim(T::from_slot(*slot(frame, src)), &addr_at(frame, addr))
            .is_err() as u32
    }
}

// dst is the first slot of a (block_id, byte_offset) pair
pub(crate) unsafe extern "C" fn jit_load_mem(
    mem: *mut c_void,
    frame: *mut u64,
    addr: u64,
    dst: u64,
) -> u32 {
    unsafe {
        let mem = &*(mem as *const SafeMemoryManager);
        match mem.load_prim::<SafeAddress>(&addr_at(frame, addr)) {
            Ok(a) => {
                write_addr(frame, dst, &a);
                0
            }
            Err(_) => 1,
        }
    }
}

// src is the first slot of a (block_id, byte_offset) pair
pub(crate) unsafe extern "C" fn jit_store_mem(
    mem: *mut c_void,
    frame: *mut u64,
    addr: u64,
    src: u64,
) -> u32 {
    unsafe {
        let mem = &mut *(mem as *mut SafeMemoryManager);
        mem.store_prim(addr_at(frame, src), &addr_at(frame, addr))
            .is_err() as u32
    }
}

pub(crate) unsafe extern "C" fn jit_alloc(
    mem: *mut c_void,
    frame: *mut u64,
    dst: u64,
    bytes: u64,
) -> u32 {
    unsafe {
        let mem = &mut *(mem as *mut SafeMemoryManager);
        // aborts rather than panics if alloc fails
        let a = mem.allocate(bytes as usize).expect("alloc failed");
        write_addr(frame, dst, &a);
        0
    }
}

pub(crate) unsafe extern "C" fn jit_free(mem: *mut c_void, frame: *mut u64, addr: u64) -> u32 {
    unsafe {
        let mem = &mut *(mem as *mut SafeMemoryManager);
        mem.free(&addr_at(frame, addr));
        0
    }
}
