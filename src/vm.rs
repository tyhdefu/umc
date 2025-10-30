use std::collections::HashMap;

use crate::bytecode::{Instruction, Operand, RegOperand, RegisterSet};
use crate::model::{RegIndex, RegType};

pub struct VirtualMachine {
    program: Vec<Instruction>,
    pc: usize,
    state: State,
}

impl VirtualMachine {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            state: State::new(),
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
        println!("Executing instruction {}: {:?}", self.pc, instr);
        match instr {
            Instruction::Mov(dst, operand) => {
                let opvalue = match operand {
                    Operand::Reg(reg) => self.state.read_uint(reg.index),
                    Operand::UnsignedConstant(x) => *x,
                };
                self.state.store_uint(dst.index, opvalue);
            }
            Instruction::Add(dst, op1, op2) => {
                let opvalue1 = Self::read_as_uint(op1, &mut self.state);
                let opvalue2 = Self::read_as_uint(op2, &mut self.state);
                let result = opvalue1 + opvalue2;
                self.state.store_uint(dst.index, result);
            }
            Instruction::Dbg(reg) => {
                assert_eq!(reg.set, RegisterSet::Single(RegType::UnsignedInt, 64));
                println!("{reg:?} = {}", self.state.read_uint(reg.index))
            }
        };
        self.pc += 1;
    }

    fn read_as_uint(operand: &Operand, state: &mut State) -> u64 {
        match operand {
            Operand::Reg(RegOperand {
                set: RegisterSet::Single(RegType::UnsignedInt, _),
                index: i,
            }) => state.read_uint(*i),
            Operand::Reg(_) => panic!("Invalid register type"),
            Operand::UnsignedConstant(c) => *c,
        }
    }
}

struct State {
    uints: HashMap<RegIndex, u64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            uints: HashMap::new(),
        }
    }

    pub fn read_uint(&self, i: RegIndex) -> u64 {
        self.uints.get(&i).copied().unwrap_or(0)
    }

    pub fn store_uint(&mut self, i: RegIndex, v: u64) {
        self.uints.insert(i, v);
    }
}
