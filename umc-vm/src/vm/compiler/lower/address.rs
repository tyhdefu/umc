//! Instruction address mov / add offset / compare, plus offset operand shared with mem address

use umc_model::instructions::{BinaryCondition, OffsetOp};
use umc_model::reg_model::{InstrRegT, Reg, RegOrConstant, UnsignedRegT};

use super::Ctx;
use crate::vm::compiler::CompileError;
use crate::vm::compiler::emit::{AluOp, Emitter, OpWidth, Signedness, Src, Tmp};
use crate::vm::compiler::lower::Flow;
use crate::vm::compiler::lower::int::{cmp_cond, store_flag};

/// mov n:a, n:b | .LABEL
pub(super) fn mov_iaddr<E: Emitter>(
    cx: &mut Ctx<E>,
    dst: &Reg<InstrRegT>,
    s: &RegOrConstant<InstrRegT>,
) -> Result<Flow, CompileError> {
    cx.load(s, Tmp::T0, OpWidth::W64);
    cx.store(dst, Tmp::T0, OpWidth::W64);
    Ok(Flow::Next)
}

/// uszie::wrapping_add_signed is plain 64-bit add
pub(super) fn add_iaddr<E: Emitter>(
    cx: &mut Ctx<E>,
    dst: &Reg<InstrRegT>,
    base: &RegOrConstant<InstrRegT>,
    offset: &OffsetOp,
) -> Result<Flow, CompileError> {
    cx.load(base, Tmp::T0, OpWidth::W64);
    load_offset(cx, offset, Tmp::T1)?;
    cx.e.alu(
        AluOp::Add,
        Tmp::T0,
        Tmp::T0,
        Src::Tmp(Tmp::T1),
        OpWidth::W64,
        Signedness::Unsigned,
    );
    cx.store(dst, Tmp::T0, OpWidth::W64);
    Ok(Flow::Next)
}

/// offset as 64bit signed delta in t, same as helper::read_offset
pub(super) fn load_offset<E: Emitter>(
    cx: &mut Ctx<E>,
    offset: &OffsetOp,
    t: Tmp,
) -> Result<(), CompileError> {
    match offset {
        // u32 slot loads zero-extend, u64 is reinterpreted as isize by interpreter
        OffsetOp::Unsigned(op) => {
            let w = op.width().map_or(Ok(OpWidth::W64), |b| cx.width(b))?;
            cx.load(op, t, w);
        }
        OffsetOp::Signed(op) => {
            let w = op.width().map_or(Ok(OpWidth::W64), |b| cx.width(b))?;
            cx.load(op, t, w);
            if w == OpWidth::W32 {
                cx.e.sign_extend(t, 32, OpWidth::W64);
            }
        }
    }
    Ok(())
}

pub(super) fn compare_iaddr<E: Emitter>(
    cx: &mut Ctx<E>,
    cond: &BinaryCondition,
    dst: &Reg<UnsignedRegT>,
    lhs: &RegOrConstant<InstrRegT>,
    rhs: &RegOrConstant<InstrRegT>,
) -> Result<Flow, CompileError> {
    cx.load(lhs, Tmp::T1, OpWidth::W64);
    let rhs = cx.src(rhs, Tmp::T2, OpWidth::W64);
    cx.e.set_cmp(
        Tmp::T0,
        cmp_cond(cond),
        Tmp::T1,
        rhs,
        OpWidth::W64,
        Signedness::Unsigned,
    );
    store_flag(cx, dst)
}
