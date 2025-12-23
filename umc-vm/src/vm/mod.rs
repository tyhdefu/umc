mod state;
mod types;

mod helper;

#[cfg(test)]
mod test;

use crate::vm::state::RegState;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{BinaryArithmeticOp, CastSingleSigned, CastSingleUnsigned};
use umc_model::instructions::{Instruction, NumReg, RegOrConstant};
use umc_model::{NumRegType, Program, RegIndex, RegType, RegWidth, RegisterSet};

pub struct VirtualMachine {
    program: Vec<Instruction>,
    pc: usize,
    state: RegState,
}

impl VirtualMachine {
    pub fn new(program: Program) -> Self {
        Self {
            program: program.instructions,
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

    pub fn inspect_uint<T>(&self, index: RegIndex, width: RegWidth) -> T
    where
        T: CastSingleUnsigned,
        T: Default,
    {
        let reg = RegOrConstant::Reg(NumReg { index, width });
        helper::read_uint::<T>(&reg, &self.state)
    }

    pub fn inspect_int<T>(&self, index: RegIndex, width: RegWidth) -> T
    where
        T: CastSingleSigned,
        T: Default,
    {
        let reg = RegOrConstant::Reg(NumReg { index, width });
        helper::read_int::<T>(&reg, &self.state)
    }

    fn execute_step(&mut self) {
        let instr: &Instruction = &self.program[self.pc];
        println!("Executing instruction {}: {}", self.pc, instr);
        match instr {
            Instruction::Mov(params) => {
                helper::execute_mov(params, &mut self.state);
            }
            Instruction::Add(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Add, &mut self.state);
            }
            Instruction::Sub(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Sub, &mut self.state);
            }
            Instruction::And(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::And, &mut self.state);
            }
            Instruction::Xor(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Xor, &mut self.state);
            }
            Instruction::Not(params) => {
                helper::execute_not(params, &mut self.state);
            }
            Instruction::Compare { cond, dst, args } => {
                helper::execute_comparison(cond, dst, args, &mut self.state);
            }
            Instruction::Jmp(p) => {
                let to = helper::read_iaddr(p, &self.state);
                println!("Jumping to {:?}", to);
                self.pc = to.pc();
                return;
            }
            Instruction::Bz(p1, p2) => {
                let to = helper::read_iaddr(p1, &self.state);
                if helper::is_zero(p2, &self.state) {
                    self.pc = to.pc();
                    return;
                }
            }
            Instruction::Bnz(p1, p2) => {
                let to = helper::read_iaddr(p1, &self.state);
                if !helper::is_zero(p2, &self.state) {
                    self.pc = to.pc();
                    return;
                }
            }
            Instruction::Dbg(reg) => match reg.set {
                RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(w))) => {
                    let reg_ref = RegOrConstant::Reg(NumReg {
                        index: reg.index,
                        width: w,
                    });
                    let x: ArbitraryUnsignedInt = helper::read_uint(&reg_ref, &self.state);
                    println!("{} = {}", reg_ref, x);
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(w))) => {
                    let reg_ref = RegOrConstant::Reg(NumReg {
                        index: reg.index,
                        width: w,
                    });
                    let x: i64 = helper::read_int(&reg_ref, &self.state);
                    println!("{} = {:X}", reg_ref, x);
                }
                _ => todo!("debug on this register not yet supported"),
            },
        };
        self.pc += 1;
    }
}
