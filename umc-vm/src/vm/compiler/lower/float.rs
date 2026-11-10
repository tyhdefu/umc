//! Float alu ops and compare, values travel as raw IEEE bits in frame slots

use umc_model::RegWidth;
use umc_model::instructions::BinaryCondition;
use umc_model::reg_model::{FloatRegT, Reg, RegOrConstant, UnsignedRegT};

use super::int::{cmp_cond, store_flag};
use super::operand::JitReg;
use super::{Ctx, Flow};
use crate::vm::compiler::CompileError;
use crate::vm::compiler::FrameSlot;
use crate::vm::compiler::bridge::{jit_frem_f32, jit_frem_f64};
use crate::vm::compiler::emit::*;

// interpreter only has f32 and f64
fn fwidth<E: Emitter>(cx: &Ctx<E>, bits: RegWidth) -> Result<OpWidth, CompileError> {
    match bits {
        32 => Ok(OpWidth::W32),
        64 => Ok(OpWidth::W64),
        _ => Err(cx.unsupported("float width")),
    }
}

// constant as bits at op widht, interpreter narrows f64 constsants with as f32
fn imm_bits(c: f64, w: OpWidth) -> u64 {
    match w {
        OpWidth::W32 => (c as f32).to_bits() as u64,
        OpWidth::W64 => c.to_bits(),
    }
}

// register slot at op width, mixed widths are not lowered
fn slot<E: Emitter>(
    cx: &mut Ctx<E>,
    r: &Reg<FloatRegT>,
    w: OpWidth,
) -> Result<FrameSlot, CompileError> {
    if fwidth(cx, r.width)? != w {
        return Err(cx.unsupported("mixed float widths"));
    }
    Ok(cx.layout.read_slot(FloatRegT::key(r)))
}

fn load<E: Emitter>(
    cx: &mut Ctx<E>,
    op: &RegOrConstant<FloatRegT>,
    t: FTmp,
    w: OpWidth,
) -> Result<(), CompileError> {
    match op {
        RegOrConstant::Const(c) => cx.e.fmov_imm(t, imm_bits(*c, w), w),
        RegOrConstant::Reg(r) => {
            let s = slot(cx, r, w)?;
            cx.e.fload_slot(t, s, w);
        }
    }
    Ok(())
}

// raw bits into integere tmp for host calls
fn load_bits<E: Emitter>(
    cx: &mut Ctx<E>,
    op: &RegOrConstant<FloatRegT>,
    t: Tmp,
    w: OpWidth,
) -> Result<(), CompileError> {
    match op {
        RegOrConstant::Const(c) => cx.e.mov_imm(t, imm_bits(*c, w), w),
        RegOrConstant::Reg(r) => {
            let s = slot(cx, r, w)?;
            cx.e.load_slot(t, s, w);
        }
    }
    Ok(())
}

pub(super) fn binop<E: Emitter>(
    cx: &mut Ctx<E>,
    op: AluOp,
    dst: &Reg<FloatRegT>,
    lhs: &RegOrConstant<FloatRegT>,
    rhs: &RegOrConstant<FloatRegT>,
) -> Result<Flow, CompileError> {
    let w = fwidth(cx, dst.width)?;
    let fop = match op {
        AluOp::Add => FAluOp::Add,
        AluOp::Sub => FAluOp::Sub,
        AluOp::Mul => FAluOp::Mul,
        AluOp::Div => FAluOp::Div, // IEEE no zero guard
        AluOp::Mod => return rem(cx, dst, lhs, rhs, w),
        // interpreter panics on these
        AluOp::And | AluOp::Or | AluOp::Xor => return Err(cx.unsupported("float bitwise op")),
    };
    load(cx, lhs, FTmp::F0, w)?;
    load(cx, rhs, FTmp::F1, w)?;
    cx.e.falu(fop, FTmp::F0, FTmp::F1, w);
    let d = cx.layout.write_slot(FloatRegT::key(dst));
    cx.e.fstore_slot(d, FTmp::F0, w);
    Ok(Flow::Next)
}

// interpreter mov is dst = src + 0.0, which turns -0.0 into +0.0, so do exactly that
pub(super) fn mov<E: Emitter>(
    cx: &mut Ctx<E>,
    dst: &Reg<FloatRegT>,
    s: &RegOrConstant<FloatRegT>,
) -> Result<Flow, CompileError> {
    binop(cx, AluOp::Add, dst, s, &RegOrConstant::Const(0.0))
}

// rem_euclid has no instruction, call into the host with raw bits
fn rem<E: Emitter>(
    cx: &mut Ctx<E>,
    dst: &Reg<FloatRegT>,
    lhs: &RegOrConstant<FloatRegT>,
    rhs: &RegOrConstant<FloatRegT>,
    w: OpWidth,
) -> Result<Flow, CompileError> {
    load_bits(cx, lhs, Tmp::T0, w)?;
    load_bits(cx, rhs, Tmp::T1, w)?;
    let f = match w {
        OpWidth::W32 => jit_frem_f32 as *const () as usize,
        OpWidth::W64 => jit_frem_f64 as *const () as usize,
    };
    cx.e.call_host(f, &[Arg::Tmp(Tmp::T0), Arg::Tmp(Tmp::T1)]);
    let d = cx.layout.write_slot(FloatRegT::key(dst));
    cx.e.store_slot(d, Tmp::T0, w);
    Ok(Flow::Next)
}

// dst = 1 if lhs cond rhs else 0, NaN makes every condition false
pub(super) fn compare<E: Emitter>(
    cx: &mut Ctx<E>,
    cond: &BinaryCondition,
    dst: &Reg<UnsignedRegT>,
    lhs: &RegOrConstant<FloatRegT>,
    rhs: &RegOrConstant<FloatRegT>,
) -> Result<Flow, CompileError> {
    // interpreter compares as f32 up to 32 bits, f64 otherwise
    let w = match lhs.width().or(rhs.width()).unwrap_or(64) {
        b if b <= 32 => OpWidth::W32,
        _ => OpWidth::W64,
    };
    load(cx, lhs, FTmp::F0, w)?;
    load(cx, rhs, FTmp::F1, w)?;
    cx.e.fset_cmp(Tmp::T0, cmp_cond(cond), FTmp::F0, FTmp::F1, w);
    store_flag(cx, dst)
}

/// float square root in place
pub(super) fn sqrt<E: Emitter>(
    cx: &mut Ctx<E>,
    dst: &Reg<FloatRegT>,
) -> Result<Flow, CompileError> {
    let w = fwidth(cx, dst.width)?;
    load(cx, &RegOrConstant::Reg(*dst), FTmp::F0, w)?;
    cx.e.fsqrt(FTmp::F0, w);
    let d = cx.layout.write_slot(FloatRegT::key(dst));
    cx.e.fstore_slot(d, FTmp::F0, w);
    Ok(Flow::Next)
}
