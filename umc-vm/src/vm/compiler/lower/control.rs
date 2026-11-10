//! Block-ending control flow: jumps and branches

use umc_model::reg_model::{InstrRegT, Reg, RegOrConstant};

use super::int::JitInt;
use super::{Ctx, Flow};
use crate::vm::compiler::emit::*;
use crate::vm::compiler::lower::operand::JitReg;
use crate::vm::compiler::{BlockExit, CompileError};

fn exit_to<E: Emitter>(cx: &mut Ctx<E>, target: &RegOrConstant<InstrRegT>) {
    match target {
        RegOrConstant::Const(pc) => cx.exit(BlockExit::Goto(*pc)),
        RegOrConstant::Reg(r) => {
            let slot = cx.layout.read_slot(InstrRegT::key(r));
            cx.exit_slot(slot);
        }
    }
}

pub(super) fn jmp<E: Emitter>(
    cx: &mut Ctx<E>,
    target: &RegOrConstant<InstrRegT>,
) -> Result<Flow, CompileError> {
    exit_to(cx, target);
    Ok(Flow::End)
}

/// link register gets the address of the next instruction, then jump
pub(super) fn jal<E: Emitter>(
    cx: &mut Ctx<E>,
    target: &RegOrConstant<InstrRegT>,
    link: &Reg<InstrRegT>,
) -> Result<Flow, CompileError> {
    cx.e.mov_imm(Tmp::T0, (cx.pc + 1) as u64, OpWidth::W64);
    // link written before target is read so jal n:0 n:0 returns to pc+1
    cx.store(link, Tmp::T0, OpWidth::W64);
    exit_to(cx, target);
    Ok(Flow::End)
}

/// bz (cond Eq) / bnz (cond Ne): go to target if op cond 0, else the next instruction
pub(super) fn branch_zero<E: Emitter, RT: JitInt>(
    cx: &mut Ctx<E>,
    cond: CmpCond,
    target: &RegOrConstant<InstrRegT>,
    op: &RegOrConstant<RT>,
) -> Result<Flow, CompileError> {
    let next = cx.pc + 1;
    match op {
        // constant operand is just plain jump as outcome is known at compile time
        RegOrConstant::Const(c) => {
            let taken = (RT::imm_bits(c) == 0) == (cond == CmpCond::Eq);
            if taken {
                exit_to(cx, target);
            } else {
                cx.exit(BlockExit::Goto(next));
            }
        }
        RegOrConstant::Reg(r) => {
            let w = cx.width(r.width)?;
            cx.load_reg(r, Tmp::T0, w);
            let taken = cx.e.new_label();
            cx.e.branch_cmp(cond, Tmp::T0, Src::Imm(0), w, RT::SIGNEDNESS, taken);
            cx.exit(BlockExit::Goto(next)); // not taken
            cx.e.bind(taken);
            exit_to(cx, target);
        }
    }
    Ok(Flow::End)
}
