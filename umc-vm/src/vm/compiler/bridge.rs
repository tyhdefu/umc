use std::ffi::c_void;

use umc_model::reg_model::*;

use crate::vm::compiler::frame::{FrameLayout, FrameSlot, SlotKey};
use crate::vm::state::{StoreFor, StorePrim};
use crate::vm::types::address::InstructionAddress;
use crate::vm::widths::int::IntWidth;
use crate::vm::widths::uint::UIntWidth;
use crate::vm::{RegState, SafeAddress, helper};

pub type CompiledFn = unsafe extern "C" fn(frame: *mut u64, mem: *mut c_void) -> u32;

pub(crate) const NULL_BLOCK: u64 = u64::MAX; // SafeAddress::NULL block id

pub struct FrameBuffer {
    slots: Vec<u64>,
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    /// read VM register state into frame buffer
    /// i.e. u64:0 = 3, u64:1 = 0, u64:2 = 1 turns into [3, 0, 1] in framebuffer.slots
    /// every slot is read, not just live-in ones: commit writes back every slot, and a block that
    /// exits early (BlockExit::Interpret) must leave registers it didn't reach yet unchanged
    pub fn prepare(
        &mut self,
        layout: &FrameLayout,
        state: &RegState<SafeAddress>,
        constants: &[SafeAddress],
    ) {
        self.slots.clear();
        self.slots.extend(layout.slots.iter().map(|info| {
            match info.key {
                SlotKey::Unsigned { index, width } => {
                    let op = RegOrConstant::Reg(Reg::<UnsignedRegT> { index, width });
                    // helper::read_uint64 means UMC widening on read happens once here precompilation rather than every time line is interpreted
                    // unset regs read as 0
                    helper::read_uint::<u64, _>(&op, state).unwrap_or_default()
                }
                SlotKey::Signed { index, width } => {
                    let op = RegOrConstant::Reg(Reg::<SignedRegT> { index, width });
                    helper::read_int::<i64, _>(&op, state) as u64
                }
                SlotKey::Float { index, width } => {
                    let op = RegOrConstant::Reg(Reg::<FloatRegT> { index, width });
                    match width {
                        32 => helper::read_float::<f32>(&op, state).to_bits() as u64,
                        _ => helper::read_float::<f64>(&op, state).to_bits(),
                    }
                }
                SlotKey::Instr { index } => {
                    let op = RegOrConstant::Reg(Reg::<InstrRegT> {
                        index,
                        width: NoWidth {},
                    });
                    helper::read_iaddr(&op, state).pc() as u64
                }
                SlotKey::MemBlock { index } => state
                    .read(Reg::<MemRegT>::from_index(index))
                    .map_or(NULL_BLOCK, |a| a.block_id() as u64),
                SlotKey::MemOffset { index } => state
                    .read(Reg::<MemRegT>::from_index(index))
                    .map_or(0, |a| a.byte_offset() as u64),
                SlotKey::ConstBlock { label } => constants
                    .get(label as usize)
                    .map_or(NULL_BLOCK, |a| a.block_id() as u64),
                SlotKey::ConstOffset { label } => constants
                    .get(label as usize)
                    .map_or(0, |a| a.byte_offset() as u64),
            }
        }));
    }

    /// commit results of compiled block back into VM registers
    pub fn commit(&self, layout: &FrameLayout, state: &mut RegState<SafeAddress>) {
        for i in 0..self.slots.len() {
            match layout.slots[i].key {
                SlotKey::Unsigned { index, width } => {
                    let reg = Reg::<UnsignedRegT> { index, width };
                    UIntWidth::store_u64(reg, state, self.slots[i]);
                }
                SlotKey::Signed { index, width } => {
                    let reg = Reg::<SignedRegT> { index, width };
                    IntWidth::store_i64(reg, state, self.slots[i] as i64);
                }
                SlotKey::Float { index, width } => {
                    let reg = Reg::<FloatRegT> { index, width };
                    match width {
                        32 => state.store_prim(reg, f32::from_bits(self.slots[i] as u32)),
                        _ => state.store_prim(reg, f64::from_bits(self.slots[i])),
                    }
                }
                SlotKey::Instr { index } => {
                    let reg = Reg::<InstrRegT> {
                        index,
                        width: NoWidth {},
                    };
                    state.store(reg, InstructionAddress::new(self.slots[i] as usize));
                }
                SlotKey::MemBlock { index } => {
                    // unset stays unset, interpreter never stores null adddres
                    let block = self.slots[i];
                    if block != NULL_BLOCK {
                        let addr =
                            SafeAddress::from_parts(block as usize, self.slots[i + 1] as usize);
                        state.store(Reg::<MemRegT>::from_index(index), addr);
                    }
                }
                // written together with memblock, constants are read only
                SlotKey::MemOffset { .. }
                | SlotKey::ConstBlock { .. }
                | SlotKey::ConstOffset { .. } => {}
            }
        }
    }

    /// Convert frame buffer into mut ptr for passing into CompiledFn
    pub fn as_mut_ptr(&mut self) -> *mut u64 {
        self.slots.as_mut_ptr()
    }

    /// get target frameslot
    pub fn get(&self, slot: FrameSlot) -> u64 {
        self.slots[slot.0 as usize]
    }
}

pub extern "C" fn jit_dbg_unsigned(value: u64, index: u64, width: u64) {
    let reg = Reg::<UnsignedRegT> {
        index: index as _,
        width: width as _,
    };
    println!("{} = 0x{:X} = {}", reg, value, value);
}

pub extern "C" fn jit_dbg_signed(value: u64, index: u64, width: u64) {
    let reg = Reg::<SignedRegT> {
        index: index as _,
        width: width as _,
    };
    match width {
        32 => println!("{} = {}", reg, value as i32),
        64 => println!("{} = {}", reg, value as i64),
        _ => unreachable!("arbitrary length signed integers not allowed"),
    }
}

pub extern "C" fn jit_dbg_float(value: u64, index: u64, width: u64) {
    let reg = Reg::<FloatRegT> {
        index: index as _,
        width: width as _,
    };
    // `as f32` would numerically convert the bit pattern; from_bits reinterprets it.
    // A panic here would abort across the extern "C" boundary, so never panic.
    match width {
        32 => println!("{} = {}", reg, f32::from_bits(value as u32)),
        64 => println!("{} = {}", reg, f64::from_bits(value)),
        _ => println!("{} = 0x{:X} (bad float width)", reg, value),
    }
}

pub extern "C" fn jit_dbg_instr(value: u64, index: u64) {
    let reg = Reg::<InstrRegT> {
        index: index as _,
        width: NoWidth {},
    };
    println!("{} = 0x{:X}", reg, value);
}

pub extern "C" fn jit_dbg_mem(value: u64, index: u64) {
    let reg = Reg::<MemRegT> {
        index: index as _,
        width: NoWidth {},
    };
    println!("{} = 0x{:X}", reg, value);
}

pub extern "C" fn jit_frem_f64(a: u64, b: u64) -> u64 {
    f64::from_bits(a).rem_euclid(f64::from_bits(b)).to_bits()
}

pub extern "C" fn jit_frem_f32(a: u64, b: u64) -> u64 {
    f32::from_bits(a as u32)
        .rem_euclid(f32::from_bits(b as u32))
        .to_bits() as u64
}
