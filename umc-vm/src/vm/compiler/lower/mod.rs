//! Lowers a basic block of UMC instructions to native code through an Emitter
//! Each instruction family has its own file, this one drives the block and dispatches

mod address;
mod cast;
mod control;
mod debug;
mod float;
mod int;
mod mem;
mod operand;

use std::ops::Range;

use umc_model::instructions::*;
use umc_model::reg_model::RegOrConstant;

use crate::vm::compiler::frame::FrameSlot;
use crate::vm::compiler::{BlockExit, CompileError, frame::FrameLayout};
use crate::vm::compiler::{ExitTo, emit::*};
use crate::vm::helper;

pub(crate) struct Lowered {
    pub code: Vec<u8>,
    pub layout: FrameLayout,
    pub exits: Vec<ExitTo>,
}

/// whether the block carries on after an instruction
enum Flow {
    Next,
    End,
}

/// state shared by every instruction lowered in a block
struct Ctx<E: Emitter> {
    e: E,
    layout: FrameLayout,
    exits: Vec<ExitTo>,
    // out-of-line BlockExit::Interpret stubs, emitted after main code
    bails: Vec<(Label, usize)>,
    // instruction currently being lowered
    pc: usize,
    entry: usize,
}

impl<E: Emitter> Ctx<E> {
    fn new(pc: usize) -> Self {
        Self {
            e: E::new(),
            layout: FrameLayout::new(),
            exits: Vec::new(),
            bails: Vec::new(),
            pc,
            entry: pc,
        }
    }

    /// return from compiled code, continuing at `to`
    fn exit(&mut self, to: BlockExit) {
        self.ret(ExitTo::Static(to));
    }

    /// return from compiled code, continuing at pc in slot
    fn exit_slot(&mut self, slot: FrameSlot) {
        self.ret(ExitTo::Slot(slot));
    }

    fn ret(&mut self, to: ExitTo) {
        self.exits.push(to);
        self.e.ret_exit((self.exits.len() - 1) as u32);
    }

    /// hand current instruction to the interpreter when lhs cond rhs holds at runtime
    fn bail_if(&mut self, cond: CmpCond, lhs: Tmp, rhs: Src, w: OpWidth, s: Signedness) {
        let bail = self.e.new_label();
        self.e.branch_cmp(cond, lhs, rhs, w, s, bail);
        self.bails.push((bail, self.pc));
    }

    fn unsupported(&self, reason: &'static str) -> CompileError {
        CompileError::Unsupported {
            pc: self.pc,
            reason,
        }
    }

    fn finish(mut self) -> Result<Lowered, CompileError> {
        for (label, pc) in std::mem::take(&mut self.bails) {
            self.e.bind(label);
            self.exit(BlockExit::Interpret(pc));
        }
        Ok(Lowered {
            code: self.e.finish().map_err(CompileError::Emit)?,
            layout: self.layout,
            exits: self.exits,
        })
    }
}

pub(crate) fn lower_block<E: Emitter>(
    program: &[Instruction],
    range: Range<usize>,
) -> Result<Lowered, CompileError> {
    if range.is_empty() {
        return Err(CompileError::EmptyBlock { pc: range.start });
    }
    let mut cx = Ctx::<E>::new(range.start);
    cx.e.prologue(&cx.layout);

    for pc in range.clone() {
        cx.pc = pc;
        if let Flow::End = lower_instr(&mut cx, &program[pc])? {
            return cx.finish();
        }
    }

    // fallthrough exit at end of block
    cx.exit(BlockExit::Goto(range.end));
    cx.finish()
}

fn lower_instr<E: Emitter>(cx: &mut Ctx<E>, instr: &Instruction) -> Result<Flow, CompileError> {
    match instr {
        Instruction::Nop => Ok(Flow::Next),
        Instruction::Mov(MovParams::UnsignedInt(dst, s)) => int::mov(cx, dst, s),
        Instruction::Mov(MovParams::SignedInt(dst, s)) => int::mov(cx, dst, s),
        Instruction::Mov(MovParams::Float(dst, s)) => float::mov(cx, dst, s),
        Instruction::Mov(MovParams::InstrAddress(dst, s)) => address::mov_iaddr(cx, dst, s),
        Instruction::Mov(MovParams::MemAddress(dst, s)) => mem::mov(cx, dst, s),
        Instruction::Add(AddParams::MemAddress(dst, base, off)) => mem::add(cx, dst, base, off),
        Instruction::Add(AddParams::UnsignedInt(ConsistentOp::Single(dst, lhs, rhs))) => {
            int::binop(cx, AluOp::Add, dst, lhs, rhs)
        }
        Instruction::Add(AddParams::SignedInt(ConsistentOp::Single(dst, lhs, rhs))) => {
            int::binop(cx, AluOp::Add, dst, lhs, rhs)
        }
        Instruction::Add(AddParams::Float(ConsistentOp::Single(dst, lhs, rhs))) => {
            float::binop(cx, AluOp::Add, dst, lhs, rhs)
        }
        Instruction::Add(AddParams::InstrAddress(dst, base, off)) => {
            address::add_iaddr(cx, dst, base, off)
        }
        Instruction::Sub(p) => num_op(cx, AluOp::Sub, p),
        Instruction::Mul(p) => num_op(cx, AluOp::Mul, p),
        Instruction::Div(p) => num_op(cx, AluOp::Div, p),
        Instruction::Mod(p) => num_op(cx, AluOp::Mod, p),
        Instruction::And(p) => num_op(cx, AluOp::And, p),
        Instruction::Or(p) => num_op(cx, AluOp::Or, p),
        Instruction::Xor(p) => num_op(cx, AluOp::Xor, p),
        Instruction::Not(NotParams::UnsignedInt(dst, s)) => int::not(cx, dst, s),
        Instruction::Compare {
            cond,
            params:
                CompareParams {
                    dst,
                    args: ConsistentComparison::UnsignedCompare(lhs, rhs),
                },
        } => int::compare(cx, cond, dst, lhs, rhs),
        Instruction::Compare {
            cond,
            params:
                CompareParams {
                    dst,
                    args: ConsistentComparison::SignedCompare(lhs, rhs),
                },
        } => int::compare(cx, cond, dst, lhs, rhs),
        Instruction::Compare {
            cond,
            params:
                CompareParams {
                    dst,
                    args: ConsistentComparison::FloatCompare(lhs, rhs),
                },
        } => float::compare(cx, cond, dst, lhs, rhs),
        Instruction::Compare {
            cond,
            params:
                CompareParams {
                    dst,
                    args: ConsistentComparison::InstrAddressCompare(lhs, rhs),
                },
        } => address::compare_iaddr(cx, cond, dst, lhs, rhs),
        Instruction::Compare {
            cond,
            params:
                CompareParams {
                    dst,
                    args: ConsistentComparison::MemAddressCompare(lhs, rhs),
                },
        } => mem::compare(cx, cond, dst, lhs, rhs),
        Instruction::Alloc(dst, size) => mem::alloc(cx, dst, size),
        Instruction::Free(r) => mem::free(cx, r),
        Instruction::Load(dst, src) => mem::load(cx, dst, src),
        Instruction::Store(dst, src) => mem::store(cx, dst, src),
        Instruction::Jmp(target) => control::jmp(cx, target),
        Instruction::Jal(target, link) => control::jal(cx, target, link),
        Instruction::Bz(target, CompareToZero::Unsigned(op)) => {
            control::branch_zero(cx, CmpCond::Eq, target, op)
        }
        Instruction::Bz(target, CompareToZero::Signed(op)) => {
            control::branch_zero(cx, CmpCond::Eq, target, op)
        }
        Instruction::Bnz(target, CompareToZero::Unsigned(op)) => {
            control::branch_zero(cx, CmpCond::Ne, target, op)
        }
        Instruction::Bnz(target, CompareToZero::Signed(op)) => {
            control::branch_zero(cx, CmpCond::Ne, target, op)
        }
        Instruction::SizeOf(dst, set) => {
            int::mov(cx, dst, &RegOrConstant::Const(helper::size_of(set) as u64))
        }
        Instruction::Cast(c) => cast::cast(cx, c),
        Instruction::Shift(params, ShiftType::LeftShift) => int::left_shift(cx, params),
        Instruction::Shift(params, ShiftType::RightLogicalShift) => int::logical_rshift(cx, params),
        Instruction::Shift(params, ShiftType::RightArithmeticShift) => {
            int::arithmetic_rshift(cx, params)
        }
        Instruction::Sqrt(AnySingleReg::Unsigned(r)) => int::sqrt(cx, r),
        Instruction::Sqrt(AnySingleReg::Signed(r)) => int::sqrt(cx, r),
        Instruction::Sqrt(AnySingleReg::Float(r)) => float::sqrt(cx, r),
        Instruction::ECall(_) => hand_over(cx),
        Instruction::Dbg(AnyReg::Single(AnySingleReg::Unsigned(r))) => debug::dbg_unsigned(cx, r),
        Instruction::Dbg(AnyReg::Single(AnySingleReg::Signed(r))) => debug::dbg_signed(cx, r),
        Instruction::Dbg(AnyReg::Single(AnySingleReg::Float(r))) => debug::dbg_float(cx, r),
        Instruction::Dbg(AnyReg::Single(AnySingleReg::Instr(r))) => debug::dbg_instr(cx, r),
        Instruction::Dbg(AnyReg::Single(AnySingleReg::Mem(r))) => debug::dbg_mem(cx, r),
        _ => {
            println!("{}", instr);
            not_lowered(cx)
        }
    }
}

/// leave instruction to interpreter but don't blacklist blockt that would be wasteful
fn hand_over<E: Emitter>(cx: &mut Ctx<E>) -> Result<Flow, CompileError> {
    if cx.pc == cx.entry {
        return Err(cx.unsupported("instruction always interpreted"));
    }
    cx.exit(BlockExit::Interpret(cx.pc));
    Ok(Flow::End)
}

/// Sub / Mul / Div / Mod / And / Or / Xor share operand shapes
fn num_op<E: Emitter>(
    cx: &mut Ctx<E>,
    op: AluOp,
    p: &AnyConsistentNumOp,
) -> Result<Flow, CompileError> {
    match p {
        AnyConsistentNumOp::UnsignedInt(ConsistentOp::Single(dst, lhs, rhs)) => {
            int::binop(cx, op, dst, lhs, rhs)
        }
        AnyConsistentNumOp::SignedInt(ConsistentOp::Single(dst, lhs, rhs)) => {
            int::binop(cx, op, dst, lhs, rhs)
        }
        AnyConsistentNumOp::Float(ConsistentOp::Single(dst, lhs, rhs)) => {
            float::binop(cx, op, dst, lhs, rhs)
        }
        _ => not_lowered(cx),
    }
}

fn not_lowered<E: Emitter>(cx: &Ctx<E>) -> Result<Flow, CompileError> {
    println!("Unsupported instruction for lowering as of now");
    Err(cx.unsupported("instruction not lowered"))
}
