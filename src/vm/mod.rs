mod state;
mod types;

mod helper;

use crate::bytecode::{Instruction, Operand, RegOperand};
use crate::model::{NumRegType, RegType, RegisterSet};
use crate::vm::state::{RegState, StoreFor};
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{BinaryArithmeticOp, CastSingleAny, UMCArithmetic};

pub struct VirtualMachine {
    program: Vec<Instruction>,
    pc: usize,
    state: RegState,
}

impl VirtualMachine {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            state: RegState::new(),
        }
    }

    pub fn execute(&mut self) {
        let program_len = self.program.len();
        while self.pc < program_len {
            self.execute_step();
        }
    }

    fn execute_step(&mut self) {
        let instr: &Instruction = &self.program[self.pc];
        println!("Executing instruction {}: {}", self.pc, instr);
        match instr {
            Instruction::Mov(dst, src) => {
                helper::compute_mov(&mut self.state, dst, src);
            }
            Instruction::Add(dst, op1, op2) => {
                helper::compute_addsub(&mut self.state, dst, op1, op2, true);
            }
            Instruction::Sub(dst, op1, op2) => {
                helper::compute_addsub(&mut self.state, dst, op1, op2, false);
            }
            Instruction::And(dst, op1, op2) => {
                Self::operate_arithmetic(&mut self.state, dst, op1, op2, BinaryArithmeticOp::And);
            }
            Instruction::Xor(dst, op1, op2) => {
                Self::operate_arithmetic(&mut self.state, dst, op1, op2, BinaryArithmeticOp::Xor);
            }
            Instruction::Not(dst, op1) => {
                Self::operate_not(&mut self.state, dst, op1);
            }
            Instruction::Jmp(op1) => {
                let to = helper::read_single_as_iaddress(&mut self.state, op1).unwrap();
                println!("Jumping to {:?}", to);
                self.pc = to.pc();
                return;
            }
            Instruction::Bz(op1, op2) => {
                let to = helper::read_single_as_iaddress(&mut self.state, op1).unwrap();
                let x: u32 = helper::read_single_as(&mut self.state, op2).unwrap();
                if x == 0 {
                    self.pc = to.pc();
                    return;
                }
            }
            Instruction::Bnz(op1, op2) => {
                let to = helper::read_single_as_iaddress(&mut self.state, op1).unwrap();
                let x: u32 = helper::read_single_as(&mut self.state, op2).unwrap();
                if x != 0 {
                    self.pc = to.pc();
                    return;
                }
            }
            Instruction::Dbg(reg) => match reg.set {
                RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(_))) => {
                    let x: ArbitraryUnsignedInt =
                        helper::read_single_as(&self.state, &Operand::Reg(reg.clone()))
                            .expect("Should be able to read any unsigned as arbitrary");
                    println!("{} = {}", reg, x);
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(i32::BITS))) => {
                    let v: i32 = self.state.read(reg.index).unwrap_or_default();
                    println!("{} = {}", reg, v);
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(i64::BITS))) => {
                    let v: i64 = self.state.read(reg.index).unwrap_or_default();
                    println!("{} = {}", reg, v);
                }
                _ => todo!(),
            },
        };
        self.pc += 1;
    }

    fn operate_arithmetic(
        state: &mut RegState,
        dst_op: &RegOperand,
        op1: &Operand,
        op2: &Operand,
        arith_op: BinaryArithmeticOp,
    ) {
        match &dst_op.set {
            RegisterSet::Single(RegType::Num(num)) => {
                helper::compute_arith(state, &num, dst_op.index, op1, op2, arith_op);
            }
            RegisterSet::Single(RegType::InstructionAddress) => todo!(),
            RegisterSet::Single(RegType::MemoryAddress) => todo!(),
            RegisterSet::Vector(_, _) => todo!(),
        }
    }

    fn operate_not(state: &mut RegState, dst_op: &RegOperand, op1: &Operand) {
        fn compute_as_prim<T>(state: &mut RegState, dst: &RegOperand, op1: &Operand)
        where
            T: UMCArithmetic,
            T: Copy + CastSingleAny + Default,
            RegState: StoreFor<T>,
        {
            let mut v: T = helper::read_single_as(&state, op1).unwrap();
            v.not();
            state.store(dst.index, v);
        }

        match dst_op.set {
            RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(u32::BITS))) => {
                compute_as_prim::<u32>(state, dst_op, op1)
            }
            RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(u64::BITS))) => {
                compute_as_prim::<u64>(state, dst_op, op1)
            }
            RegisterSet::Single(_) => todo!(),
            RegisterSet::Vector(_, _) => todo!(),
        }
    }
}
