mod state;
mod types;

use crate::bytecode::{Instruction, Operand, RegOperand};
use crate::model::{RegType, RegisterSet};
use crate::vm::state::{RegState, StoreFor};
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{CastFrom, CastInto, UMCArithmetic};

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
            Instruction::Dbg(reg) => match reg.set {
                RegisterSet::Single(RegType::UnsignedInt, _) => {
                    let x: ArbitraryUnsignedInt =
                        Self::read_as(&self.state, &Operand::Reg(reg.clone()))
                            .expect("Should be able to read any unsigned as arbitrary");
                    println!("{} = {:?}", reg, x);
                }
                /*RegisterSet::Single(RegType::UnsignedInt, u32::BITS) => {
                    let v: Option<u32> = self.state.read(reg.index).copied();
                    println!("{} = {:?}", reg, v);
                }
                RegisterSet::Single(RegType::UnsignedInt, u64::BITS) => {
                    let v: Option<u64> = self.state.read(reg.index).copied();
                }*/
                RegisterSet::Single(RegType::SignedInt, i32::BITS) => {
                    let v: Option<i32> = self.state.read(reg.index).copied();
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
            RegisterSet::Single(RegType::UnsignedInt, _) => {
                todo!()
            }
            RegisterSet::Single(RegType::SignedInt, _) => todo!(),
            RegisterSet::Single(RegType::Float, _) => todo!(),
            RegisterSet::Vector(_, _, _) => todo!(),
        }
    }

    fn read_as<T>(state: &RegState, operand: &Operand) -> Result<T, ()>
    where
        T: CastFrom<u32> + CastFrom<u64> + CastFrom<ArbitraryUnsignedInt>,
    {
        match operand {
            Operand::Reg(reg) => match reg.set {
                RegisterSet::Single(RegType::UnsignedInt, u32::BITS) => {
                    let v: u32 = state.read(reg.index).copied().unwrap_or_default();
                    Ok(v.cast_into())
                }
                RegisterSet::Single(RegType::UnsignedInt, u64::BITS) => {
                    let v: u64 = state.read(reg.index).copied().unwrap_or_default();
                    Ok(v.cast_into())
                }
                RegisterSet::Single(RegType::UnsignedInt, _) => {
                    todo!()
                }
                RegisterSet::Single(_, _) => todo!(),
                RegisterSet::Vector(_, _, _) => Err(()),
            },
            Operand::UnsignedConstant(c) => Ok((*c).cast_into()),
        }
    }
}
