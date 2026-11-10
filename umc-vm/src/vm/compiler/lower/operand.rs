//! How register types map onto frame slots and immediates

use umc_model::RegWidth;
use umc_model::reg_model::{
    FloatRegT, InstrRegT, MemRegT, Reg, RegOrConstant, RegTypeT, SignedRegT, UnsignedRegT,
};

use super::Ctx;
use crate::vm::compiler::CompileError;
use crate::vm::compiler::emit::{Emitter, OpWidth, Src, Tmp};
use crate::vm::compiler::frame::SlotKey;

/// register type that lives in a frame slot
/// adding a register type to the jit starts with implementing this
pub(super) trait JitReg: RegTypeT {
    fn key(r: &Reg<Self>) -> SlotKey;
    /// constant operand as raw bits for an immediate
    fn imm_bits(c: &Self::C) -> u64;
}

impl JitReg for UnsignedRegT {
    fn key(r: &Reg<Self>) -> SlotKey {
        SlotKey::Unsigned {
            index: r.index,
            width: r.width,
        }
    }
    fn imm_bits(c: &u64) -> u64 {
        *c
    }
}

impl JitReg for SignedRegT {
    fn key(r: &Reg<Self>) -> SlotKey {
        SlotKey::Signed {
            index: r.index,
            width: r.width,
        }
    }
    fn imm_bits(c: &i64) -> u64 {
        *c as u64
    }
}

impl JitReg for FloatRegT {
    fn key(r: &Reg<Self>) -> SlotKey {
        SlotKey::Float {
            index: r.index,
            width: r.width,
        }
    }
    fn imm_bits(c: &f64) -> u64 {
        c.to_bits()
    }
}

impl JitReg for InstrRegT {
    fn key(r: &Reg<Self>) -> SlotKey {
        SlotKey::Instr { index: r.index }
    }
    fn imm_bits(c: &usize) -> u64 {
        *c as u64
    }
}

impl JitReg for MemRegT {
    fn key(r: &Reg<Self>) -> SlotKey {
        SlotKey::MemBlock { index: r.index }
    }
    fn imm_bits(c: &u32) -> u64 {
        *c as u64
    }
}

impl<E: Emitter> Ctx<E> {
    /// native op width for a declared register width
    pub(super) fn width(&self, bits: RegWidth) -> Result<OpWidth, CompileError> {
        match bits {
            // relaxed semantics so that signed comparison functions ok
            1..=32 => Ok(OpWidth::W32),
            33..=64 => Ok(OpWidth::W64),
            _ => Err(self.unsupported("register width")),
        }
    }

    /// operand as alu source: register loaded into t, constant kept as immediate
    pub(super) fn src<RT: JitReg>(&mut self, op: &RegOrConstant<RT>, t: Tmp, w: OpWidth) -> Src {
        match op {
            RegOrConstant::Const(c) => Src::Imm(RT::imm_bits(c)),
            RegOrConstant::Reg(r) => {
                self.load_reg(r, t, w);
                Src::Tmp(t)
            }
        }
    }

    /// operand always materialised in t
    pub(super) fn load<RT: JitReg>(&mut self, op: &RegOrConstant<RT>, t: Tmp, w: OpWidth) {
        match op {
            RegOrConstant::Const(c) => self.e.mov_imm(t, RT::imm_bits(c), w),
            RegOrConstant::Reg(r) => self.load_reg(r, t, w),
        }
    }

    pub(super) fn load_reg<RT: JitReg>(&mut self, r: &Reg<RT>, t: Tmp, w: OpWidth) {
        let slot = self.layout.read_slot(RT::key(r));
        self.e.load_slot(t, slot, w);
    }

    pub(super) fn store<RT: JitReg>(&mut self, dst: &Reg<RT>, t: Tmp, w: OpWidth) {
        let slot = self.layout.write_slot(RT::key(dst));
        self.e.store_slot(slot, t, w);
    }
}
