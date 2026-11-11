//! Casts: int <-> int natively, anything touching a float through jit_num_cast

use umc_model::instructions::{ResizeCast, SimpleCast};
use umc_model::reg_model::{FloatRegT, Reg, RegOrConstant};

use super::int::JitInt;
use super::operand::JitReg;
use super::{Ctx, Flow, float};
use crate::vm::compiler::CompileError;
use crate::vm::compiler::bridge::{CAST_F, CAST_I, CAST_U, jit_num_cast};
use crate::vm::compiler::emit::*;

pub(super) fn cast<E: Emitter>(cx: &mut Ctx<E>, c: &SimpleCast) -> Result<Flow, CompileError> {
    match c {
        SimpleCast::Resize(ResizeCast::Unsigned(d, s)) => int_to_int(cx, d, s),
        SimpleCast::Resize(ResizeCast::Signed(d, s)) => int_to_int(cx, d, s),
        SimpleCast::IgnoreSigned(c) => int_to_int(cx, c.dst(), c.from()),
        SimpleCast::AddSign(c) => int_to_int(cx, c.dst(), c.from()),
        SimpleCast::Resize(ResizeCast::Float(d, s)) => {
            let src = float_src(cx, s)?;
            to_float(cx, d, src)
        }
        SimpleCast::USink(c) => {
            let src = float_src(cx, c.from())?;
            to_int(cx, c.dst(), src)
        }
        SimpleCast::ISink(c) => {
            let src = float_src(cx, c.from())?;
            to_int(cx, c.dst(), src)
        }
        SimpleCast::UBuoy(c) => {
            let src = int_src(cx, c.from())?;
            to_float(cx, c.dst(), src)
        }
        SimpleCast::IBuoy(c) => {
            let src = int_src(cx, c.from())?;
            to_float(cx, c.dst(), src)
        }
    }
}

fn bits(w: OpWidth) -> u64 {
    match w {
        OpWidth::W32 => 32,
        OpWidth::W64 => 64,
    }
}

fn int_kind<RT: JitInt>() -> u64 {
    match RT::SIGNEDNESS {
        Signedness::Unsigned => CAST_U,
        Signedness::Signed => CAST_I,
    }
}

// W32 slot writes leave the top half of the 8 byte slot stale, so read at the source width, constants are full 64 bit
fn int_to_int<E: Emitter, TO: JitInt, FROM: JitInt>(
    cx: &mut Ctx<E>,
    dst: &Reg<TO>,
    s: &RegOrConstant<FROM>,
) -> Result<Flow, CompileError> {
    let w = cx.width(dst.width)?;
    let sw = cx.width(s.width().unwrap_or(64))?;
    // W32 loads zero the top half, signed sources need it filled
    cx.load(s, Tmp::T0, sw);
    if sw == OpWidth::W32 && w == OpWidth::W64 && FROM::SIGNEDNESS == Signedness::Signed {
        cx.e.sign_extend(Tmp::T0, 32, OpWidth::W64);
    }
    TO::wrap(&mut cx.e, Tmp::T0, dst.width, w);
    cx.store(dst, Tmp::T0, w);
    Ok(Flow::Next)
}

/// source bits into T0, returns its jit_num_cast descriptor
fn float_src<E: Emitter>(
    cx: &mut Ctx<E>,
    s: &RegOrConstant<FloatRegT>,
) -> Result<u64, CompileError> {
    let w = float::fwidth(cx, s.width().unwrap_or(64))?;
    float::load_bits(cx, s, Tmp::T0, w)?;
    Ok(CAST_F | bits(w))
}

fn int_src<E: Emitter, RT: JitInt>(
    cx: &mut Ctx<E>,
    s: &RegOrConstant<RT>,
) -> Result<u64, CompileError> {
    let w = cx.width(s.width().unwrap_or(64))?;
    cx.load(s, Tmp::T0, w);
    Ok(int_kind::<RT>() | bits(w))
}

// shortcut: host call per float cast, upgrade to cvt / fcvtz* in the emitters if casts show up hot
fn call<E: Emitter>(cx: &mut Ctx<E>, src: u64, dst: u64) {
    let f = jit_num_cast as *const () as usize;
    cx.e.call_host(f, &[Arg::Tmp(Tmp::T0), Arg::Imm(src), Arg::Imm(dst)]);
}

fn to_float<E: Emitter>(
    cx: &mut Ctx<E>,
    dst: &Reg<FloatRegT>,
    src: u64,
) -> Result<Flow, CompileError> {
    let w = float::fwidth(cx, dst.width)?;
    call(cx, src, CAST_F | bits(w));
    let d = cx.layout.write_slot(FloatRegT::key(dst));
    cx.e.store_slot(d, Tmp::T0, w);
    Ok(Flow::Next)
}

fn to_int<E: Emitter, RT: JitInt>(
    cx: &mut Ctx<E>,
    dst: &Reg<RT>,
    src: u64,
) -> Result<Flow, CompileError> {
    let w = cx.width(dst.width)?;
    call(cx, src, int_kind::<RT>() | bits(w));
    RT::wrap(&mut cx.e, Tmp::T0, dst.width, w);
    cx.store(dst, Tmp::T0, w);
    Ok(Flow::Next)
}
