mod memory;
mod state;
mod types;

mod helper;

#[cfg(test)]
mod test;

use crate::vm::memory::MemoryManager;
use crate::vm::memory::safe::{SafeAddress, SafeMemoryManager};
use crate::vm::state::{RegState, StoreFor};
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{
    BinaryArithmeticOp, BinaryBitwiseOp, CastSingleFloat, CastSingleSigned, CastSingleUnsigned,
};
use umc_model::instructions::Instruction;
use umc_model::reg_model::{Reg, RegOrConstant, UnsignedRegT};
use umc_model::{Program, RegIndex, RegWidth};

pub struct VirtualMachine {
    program: Vec<Instruction>,
    pc: usize,
    state: RegState<SafeAddress>,
    memory: SafeMemoryManager,
}

impl VirtualMachine {
    /// Initialise a new VM with the given program
    pub fn new(program: Program) -> Self {
        Self {
            program: program.instructions,
            pc: 0,
            state: RegState::new(),
            memory: SafeMemoryManager::new(),
        }
    }

    /// Begin execution of the program until it completes
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
        let reg = RegOrConstant::from_reg(Reg { index, width });
        helper::read_uint::<T>(&reg, &self.state)
    }

    pub fn inspect_uint_vec<T>(
        &self,
        index: RegIndex,
        width: RegWidth,
        length: RegWidth,
    ) -> Option<Vec<T>>
    where
        T: CastSingleUnsigned,
        T: Default,
    {
        let reg: Reg<UnsignedRegT> = Reg { index, width };
        helper::read_uint_vec(&reg, length, &self.state)
    }

    pub fn inspect_int<T>(&self, index: RegIndex, width: RegWidth) -> T
    where
        T: CastSingleSigned,
        T: Default,
    {
        let reg = RegOrConstant::from_reg(Reg { index, width });
        helper::read_int::<T>(&reg, &self.state)
    }

    pub fn inspect_float<T>(&self, index: RegIndex, width: RegWidth) -> T
    where
        T: CastSingleFloat,
    {
        let reg = RegOrConstant::from_reg(Reg { index, width });
        helper::read_float(&reg, &self.state)
    }

    fn execute_step(&mut self) {
        let instr: &Instruction = &self.program[self.pc];
        println!("Executing instruction {}: {}", self.pc, instr);
        match instr {
            Instruction::Nop => {}
            Instruction::Mov(params) => {
                helper::execute_mov(params, &mut self.state);
            }
            Instruction::Add(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Add, &mut self.state);
            }
            Instruction::Sub(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Sub, &mut self.state);
            }
            Instruction::Mul(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Mul, &mut self.state);
            }
            Instruction::Div(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Div, &mut self.state);
            }
            Instruction::Mod(num_op) => {
                helper::execute_arithmetic(num_op, BinaryArithmeticOp::Modulo, &mut self.state);
            }
            Instruction::And(num_op) => {
                helper::execute_bitwise(num_op, BinaryBitwiseOp::And, &mut self.state);
            }
            Instruction::Or(num_op) => {
                helper::execute_bitwise(num_op, BinaryBitwiseOp::Or, &mut self.state);
            }
            Instruction::Xor(num_op) => {
                helper::execute_bitwise(num_op, BinaryBitwiseOp::Xor, &mut self.state);
            }
            Instruction::Not(params) => {
                helper::execute_not(params, &mut self.state);
            }
            Instruction::Compare { cond, params } => {
                helper::execute_comparison(cond, params, &mut self.state);
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
            Instruction::Alloc(mem_reg, size_reg) => {
                let arb_bytes: ArbitraryUnsignedInt = helper::read_uint(size_reg, &self.state);
                let bytes = arb_bytes.as_usize();
                let address: SafeAddress = self.memory.allocate(bytes).expect("alloc failed");
                self.state.store(*mem_reg, address);
            }
            Instruction::Free(mem_reg) => {
                let address: &SafeAddress = self
                    .state
                    .read(*mem_reg)
                    .expect("Tried to free unset memory");
                self.memory.free(address);
            }
            Instruction::Load(dst_reg, mem_reg) => {
                helper::execute_load(dst_reg, mem_reg, &mut self.state, &self.memory)
                    .unwrap_or_else(|err| {
                        panic!("Failed to load {} from {}: {:?}", dst_reg, mem_reg, err)
                    });
            }
            Instruction::Store(mem_reg, from_reg) => {
                helper::execute_store(from_reg, mem_reg, &self.state, &mut self.memory)
                    .unwrap_or_else(|err| {
                        panic!("Failed to store {} into {}: {:?}", from_reg, mem_reg, err)
                    });
            }
            Instruction::Dbg(reg) => helper::execute_debug(reg, &self.state),
        };
        self.pc += 1;
    }
}
