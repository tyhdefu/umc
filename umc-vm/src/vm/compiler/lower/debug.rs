//! dbg lowered as a call back into the host

use umc_model::reg_model::{FloatRegT, InstrRegT, MemRegT, Reg, SignedRegT, UnsignedRegT};

use super::{Ctx, Flow};
use crate::vm::compiler::CompileError;
use crate::vm::compiler::bridge::{
    jit_dbg_float, jit_dbg_instr, jit_dbg_mem, jit_dbg_signed, jit_dbg_unsigned,
};
use crate::vm::compiler::emit::*;

pub(super) fn dbg_unsigned<E: Emitter>(
    cx: &mut Ctx<E>,
    r: &Reg<UnsignedRegT>,
) -> Result<Flow, CompileError> {
    let w = cx.width(r.width)?;
    cx.load_reg(r, Tmp::T0, w);
    cx.e.call_host(
        jit_dbg_unsigned as *const () as usize,
        &[
            Arg::Tmp(Tmp::T0),
            Arg::Imm(r.index as u64),
            Arg::Imm(r.width as u64),
        ],
    );
    Ok(Flow::Next)
}

pub(super) fn dbg_signed<E: Emitter>(
    cx: &mut Ctx<E>,
    r: &Reg<SignedRegT>,
) -> Result<Flow, CompileError> {
    let w = cx.width(r.width)?;
    cx.load_reg(r, Tmp::T0, w);
    cx.e.call_host(
        jit_dbg_signed as *const () as usize,
        &[
            Arg::Tmp(Tmp::T0),
            Arg::Imm(r.index as u64),
            Arg::Imm(r.width as u64),
        ],
    );
    Ok(Flow::Next)
}

pub(super) fn dbg_float<E: Emitter>(
    cx: &mut Ctx<E>,
    r: &Reg<FloatRegT>,
) -> Result<Flow, CompileError> {
    let w = cx.width(r.width)?;
    cx.load_reg(r, Tmp::T0, w);
    cx.e.call_host(
        jit_dbg_float as *const () as usize,
        &[
            Arg::Tmp(Tmp::T0),
            Arg::Imm(r.index as u64),
            Arg::Imm(r.width as u64),
        ],
    );
    Ok(Flow::Next)
}

pub(super) fn dbg_instr<E: Emitter>(
    cx: &mut Ctx<E>,
    r: &Reg<InstrRegT>,
) -> Result<Flow, CompileError> {
    cx.load_reg(r, Tmp::T0, OpWidth::W64);
    cx.e.call_host(
        jit_dbg_instr as *const () as usize,
        &[Arg::Tmp(Tmp::T0), Arg::Imm(r.index as u64)],
    );
    Ok(Flow::Next)
}

pub(super) fn dbg_mem<E: Emitter>(cx: &mut Ctx<E>, r: &Reg<MemRegT>) -> Result<Flow, CompileError> {
    cx.load_reg(r, Tmp::T0, OpWidth::W64);
    cx.e.call_host(
        jit_dbg_mem as *const () as usize,
        &[Arg::Tmp(Tmp::T0), Arg::Imm(r.index as u64)],
    );
    Ok(Flow::Next)
}
