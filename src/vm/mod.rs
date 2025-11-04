mod state;
mod types;

use crate::bytecode::{Instruction, Operand, RegOperand};
use crate::model::{RegType, RegisterSet};
use crate::vm::state::{ArbStoreFor, RegState, StoreFor};
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{CastFrom, CastInto, CastSingleAny, UMCArithmetic};

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
            Instruction::Mov(dst, operand) => {
                Self::operate_arithmetic(
                    &mut self.state,
                    dst,
                    operand,
                    &Operand::UnsignedConstant(0),
                );
            }
            Instruction::Add(dst, op1, op2) => {
                Self::operate_arithmetic(&mut self.state, dst, op1, op2);
            }
            Instruction::Not(dst, op1) => {
                Self::operate_not(&mut self.state, dst, op1);
            }
            Instruction::Dbg(reg) => match reg.set {
                RegisterSet::Single(RegType::UnsignedInt, _) => {
                    let x: ArbitraryUnsignedInt =
                        Self::read_as(&self.state, &Operand::Reg(reg.clone()))
                            .expect("Should be able to read any unsigned as arbitrary");
                    println!("{} = {}", reg, x);
                }
                RegisterSet::Single(RegType::SignedInt, i32::BITS) => {
                    let v: Option<i32> = self.state.read(reg.index);
                    println!("{} = {:?}", reg, v);
                }
                _ => todo!(),
            },
        };
        self.pc += 1;
    }

    fn operate_arithmetic(state: &mut RegState, dst_op: &RegOperand, op1: &Operand, op2: &Operand) {
        match dst_op.set {
            RegisterSet::Single(RegType::UnsignedInt, u32::BITS) => {
                let mut op1_v: u32 = Self::read_as(&state, op1).unwrap();
                let op2_v: u32 = Self::read_as(&state, op2).unwrap();
                op1_v.add(&op2_v);
                state.store(dst_op.index, op1_v);
            }
            RegisterSet::Single(RegType::UnsignedInt, u64::BITS) => {
                let mut op1_v: u64 = Self::read_as(&state, op1).unwrap();
                let op2_v: u64 = Self::read_as(&state, op2).unwrap();
                op1_v.add(&op2_v);
                state.store(dst_op.index, op1_v);
            }
            RegisterSet::Single(RegType::UnsignedInt, w) => {
                let op1_v: ArbitraryUnsignedInt = Self::read_as(&state, op1).unwrap();
                let op2_v: ArbitraryUnsignedInt = Self::read_as(&state, op2).unwrap();
                // TODO: catch dst is one of the operands?
                let mut dst = ArbitraryUnsignedInt::new(w);
                dst.add(&op1_v);
                dst.add(&op2_v);
                state.store_arb(dst_op.index, w, dst);
            }
            RegisterSet::Single(RegType::SignedInt, _) => todo!(),
            RegisterSet::Single(RegType::Float, _) => todo!(),
            RegisterSet::Vector(_, _, _) => todo!(),
        }
    }

    fn operate_not(state: &mut RegState, dst_op: &RegOperand, op1: &Operand) {
        fn compute_as<T>(state: &mut RegState, dst: &RegOperand, op1: &Operand)
        where
            T: UMCArithmetic,
            T: Copy + CastSingleAny + Default,
            RegState: StoreFor<T>,
        {
            let mut v: T = VirtualMachine::read_as(&state, op1).unwrap();
            v.not();
            state.store(dst.index, v);
        }

        match dst_op.set {
            RegisterSet::Single(RegType::UnsignedInt, u32::BITS) => {
                compute_as::<u32>(state, dst_op, op1)
            }
            RegisterSet::Single(RegType::UnsignedInt, u64::BITS) => {
                compute_as::<u64>(state, dst_op, op1)
            }
            RegisterSet::Single(_, _) => todo!(),
            RegisterSet::Vector(_, _, _) => todo!(),
        }
    }

    fn read_as<'a, T>(state: &'a RegState, operand: &Operand) -> Result<T, ()>
    where
        T: CastFrom<u32> + CastFrom<u64> + CastFrom<ArbitraryUnsignedInt> + Default,
    {
        match operand {
            Operand::Reg(reg) => match reg.set {
                RegisterSet::Single(RegType::UnsignedInt, u32::BITS) => {
                    let v: u32 = state.read(reg.index).unwrap_or_default();
                    Ok(v.cast_into())
                }
                RegisterSet::Single(RegType::UnsignedInt, u64::BITS) => {
                    let v: u64 = state.read(reg.index).unwrap_or_default();
                    Ok(v.cast_into())
                }
                RegisterSet::Single(RegType::UnsignedInt, w) => {
                    let v: T = state
                        .read_arb(reg.index, w)
                        .map(|v| v.cast_into())
                        .unwrap_or_default();

                    Ok(v)
                }
                RegisterSet::Single(_, _) => todo!(),
                RegisterSet::Vector(_, _, _) => Err(()),
            },
            Operand::UnsignedConstant(c) => Ok((*c).cast_into()),
        }
    }
}
