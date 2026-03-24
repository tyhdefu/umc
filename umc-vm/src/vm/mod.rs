mod memory;
mod state;
mod types;

mod compiler;
mod environment;
mod helper;
mod widths;

#[cfg(test)]
mod test;

use crate::vm::environment::AnyEnvironment;
use crate::vm::memory::safe::{SafeAddress, SafeMemoryManager};
use crate::vm::memory::{AllocateError, MemoryManager};
use crate::vm::state::{RegState, StoreFor};
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{
    BinaryArithmeticOp, BinaryBitwiseOp, CastSingleFloat, CastSingleSigned, CastSingleUnsigned,
};
use crate::vm::widths::uint::UIntWidth;
use umc_model::instructions::Instruction;
use umc_model::reg_model::{Reg, RegOrConstant, UnsignedRegT};
use umc_model::{NumRegType, Program, RegIndex, RegType, RegWidth, RegisterSet};

pub struct VirtualMachine {
    program: Vec<Instruction>,
    pc: usize,
    state: RegState<SafeAddress>,
    memory: SafeMemoryManager,
    // Memory Label id -> Allocated Memory Address
    memory_constants: Vec<SafeAddress>,
    environment: AnyEnvironment,
    verbose: bool,
}

pub struct VMOptions {
    /// Whether to print extra debugging information about which instructions are being executed
    pub verbose: bool,
}

impl VMOptions {
    /// Recommended configuration for debugging the VM
    pub fn vm_debug() -> Self {
        Self { verbose: true }
    }
}

#[derive(Debug)]
pub enum CreateVMError {
    /// Pre-initialised memory could not be allocated
    AllocateError(AllocateError),
}

impl VirtualMachine {
    /// Initialise a new VM with the given program
    pub fn create(program: Program, options: VMOptions) -> Result<Self, CreateVMError> {
        let mut memory = SafeMemoryManager::new();

        // Allocate pre-initialised memory
        let mut memory_constants = Vec::with_capacity(program.pre_init_mem.len());
        for x in program.pre_init_mem {
            let address = memory
                .allocate_initalised(x)
                .map_err(|e| CreateVMError::AllocateError(e))?;
            memory_constants.push(address);
        }

        Ok(Self {
            program: program.instructions,
            pc: 0,
            state: RegState::new(),
            memory,
            memory_constants: memory_constants,
            verbose: options.verbose,
            environment: AnyEnvironment::new(),
        })
    }

    /// Begin execution of the program until it completes
    pub fn execute(&mut self) {
        let program_len = self.program.len();
        while self.pc < program_len {
            self.execute_step();
        }
    }

    pub fn inspect_bool(&self, index: RegIndex) -> bool {
        self.inspect_uint(index, 1)
    }

    pub fn inspect_uint<T>(&self, index: RegIndex, width: RegWidth) -> T
    where
        T: CastSingleUnsigned,
        T: Default,
    {
        let reg = RegOrConstant::from_reg(Reg { index, width });
        helper::read_uint::<T, _>(&reg, &self.state)
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
        helper::read_int::<T, _>(&reg, &self.state)
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
        if self.verbose {
            println!("Executing instruction {}: {}", self.pc, instr);
        }
        match instr {
            Instruction::Nop => {}
            Instruction::Mov(params) => {
                helper::execute_mov(params, &mut self.state, &self.memory_constants);
            }
            Instruction::Add(add_params) => {
                helper::execute_add(add_params, &mut self.state, &self.memory_constants);
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
                helper::execute_comparison(cond, params, &mut self.state, &self.memory_constants);
            }
            Instruction::Jmp(p) => {
                let to = helper::read_iaddr(p, &self.state);
                if self.verbose {
                    println!("Jumping to {:?}", to);
                }
                self.pc = to.pc();
                return;
            }
            Instruction::Jal(p, r) => {
                let link = InstructionAddress::new(self.pc + 1);
                self.state.store(*r, link);
                let to = helper::read_iaddr(p, &self.state);
                if self.verbose {
                    println!("Jumping to {:?} (linking {:?})", to, link);
                }
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
                helper::execute_load(
                    dst_reg,
                    mem_reg,
                    &mut self.state,
                    &self.memory,
                    &self.memory_constants,
                )
                .unwrap_or_else(|err| {
                    panic!("Failed to load {} from {}: {:?}", dst_reg, mem_reg, err)
                });
            }
            Instruction::Store(mem_reg, from_reg) => {
                helper::execute_store(
                    from_reg,
                    mem_reg,
                    &self.state,
                    &mut self.memory,
                    &self.memory_constants,
                )
                .unwrap_or_else(|err| {
                    panic!("Failed to store {} into {}: {:?}", from_reg, mem_reg, err)
                });
            }
            Instruction::SizeOf(reg, reg_type) => {
                let rt = match reg_type {
                    umc_model::RegisterSet::Single(rt) => rt,
                    umc_model::RegisterSet::Vector(rt, _) => rt,
                };
                let mut size_bytes: u32 = match rt {
                    RegType::Num(NumRegType::UnsignedInt(w)) => w.div_ceil(u8::BITS),
                    RegType::Num(NumRegType::SignedInt(w)) => w.div_ceil(u8::BITS),
                    RegType::Num(NumRegType::Float(w)) => w.div_ceil(u8::BITS),
                    RegType::InstructionAddress => InstructionAddress::SIZE_BYTES,
                    RegType::MemoryAddress => SafeAddress::SIZE_BYTES,
                };
                if let RegisterSet::Vector(_, length) = reg_type {
                    size_bytes = size_bytes * length;
                }
                UIntWidth::store_u64(*reg, &mut self.state, size_bytes as u64);
            }
            Instruction::Cast(simple_cast) => {
                helper::execute_simple_cast(simple_cast, &mut self.state);
            }
            Instruction::ECall(ecall) => helper::execute_ecall(
                ecall,
                &mut self.state,
                &mut self.memory,
                &self.memory_constants,
                &mut self.environment,
            )
            .unwrap_or_else(|err| panic!("Environment Call Failed: {:?} ({})", err, instr)),
            Instruction::Dbg(reg) => helper::execute_debug(reg, &self.state),
        };
        self.pc += 1;
    }
}
