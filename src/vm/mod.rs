mod state;
mod types;

use crate::bytecode::{Instruction, Operand, RegOperand};
use crate::model::{RegType, RegisterSet};
use crate::vm::state::{ArbStoreFor, RegState, StoreFor};
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{BinaryArithmeticOp, CastInto, CastSingleAny, UMCArithmetic};

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
                    BinaryArithmeticOp::Add,
                );
            }
            Instruction::Add(dst, op1, op2) => {
                Self::operate_arithmetic(&mut self.state, dst, op1, op2, BinaryArithmeticOp::Add);
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
                let to = read_single_as_iaddress(&mut self.state, op1).unwrap();
                self.pc = to.pc();
                return;
            }
            Instruction::Bz(op1, op2) => {
                let to = read_single_as_iaddress(&mut self.state, op1).unwrap();
                let x: u32 = read_single_as(&mut self.state, op2).unwrap();
                if x == 0 {
                    self.pc = to.pc();
                    return;
                }
            }
            Instruction::Bnz(op1, op2) => {
                let to = read_single_as_iaddress(&mut self.state, op1).unwrap();
                let x: u32 = read_single_as(&mut self.state, op2).unwrap();
                if x != 0 {
                    self.pc = to.pc();
                    return;
                }
            }
            Instruction::Dbg(reg) => match reg.set {
                RegisterSet::Single(RegType::UnsignedInt(_)) => {
                    let x: ArbitraryUnsignedInt =
                        read_single_as(&self.state, &Operand::Reg(reg.clone()))
                            .expect("Should be able to read any unsigned as arbitrary");
                    println!("{} = {}", reg, x);
                }
                RegisterSet::Single(RegType::SignedInt(i32::BITS)) => {
                    let v: Option<i32> = self.state.read(reg.index);
                    println!("{} = {:?}", reg, v);
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
        fn compute_as_prim<T>(
            state: &mut RegState,
            dst_op: &RegOperand,
            op1: &Operand,
            op2: &Operand,
            arith_op: BinaryArithmeticOp,
        ) -> Result<(), ()>
        where
            T: UMCArithmetic,
            T: Copy + CastSingleAny + Default,
            RegState: StoreFor<T>,
        {
            let mut op1_v: T = read_single_as(&state, op1)?;
            let op2_v: T = read_single_as(&state, op2)?;

            arith_op.operate(&mut op1_v, &op2_v);
            state.store(dst_op.index, op1_v);
            Ok(())
        }

        match dst_op.set {
            RegisterSet::Single(RegType::UnsignedInt(u32::BITS)) => {
                compute_as_prim::<u32>(state, dst_op, op1, op2, arith_op).unwrap();
            }
            RegisterSet::Single(RegType::UnsignedInt(u64::BITS)) => {
                compute_as_prim::<u64>(state, dst_op, op1, op2, arith_op).unwrap();
            }
            RegisterSet::Single(RegType::UnsignedInt(w)) => {
                let op1_v: ArbitraryUnsignedInt = read_single_as(&state, op1).unwrap();
                let op2_v: ArbitraryUnsignedInt = read_single_as(&state, op2).unwrap();
                // TODO: catch dst is one of the operands?
                let mut dst = ArbitraryUnsignedInt::new(w);
                dst.add(&op1_v); // TODO: Verify this is chill?
                arith_op.operate(&mut dst, &op2_v);
                state.store_arb(dst_op.index, w, dst);
            }
            RegisterSet::Single(RegType::InstructionAddress) => {
                let mut op1_v: InstructionAddress = read_single_as_iaddress(&state, op1).unwrap();
                let op2_v: InstructionAddress = read_single_as_iaddress(&state, op2).unwrap();

                arith_op.operate(&mut op1_v, &op2_v);
                state.store(dst_op.index, op1_v);
            }
            RegisterSet::Single(RegType::MemoryAddress) => {
                let mut op1_v: InstructionAddress = read_single_as_iaddress(&state, op1).unwrap();
                let op2_v: InstructionAddress = read_single_as_iaddress(&state, op2).unwrap();

                arith_op.operate(&mut op1_v, &op2_v);
                state.store(dst_op.index, op1_v);
            }
            RegisterSet::Single(RegType::SignedInt(_)) => todo!(),
            RegisterSet::Single(RegType::Float(_)) => todo!(),
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
            let mut v: T = read_single_as(&state, op1).unwrap();
            v.not();
            state.store(dst.index, v);
        }

        match dst_op.set {
            RegisterSet::Single(RegType::UnsignedInt(u32::BITS)) => {
                compute_as_prim::<u32>(state, dst_op, op1)
            }
            RegisterSet::Single(RegType::UnsignedInt(u64::BITS)) => {
                compute_as_prim::<u64>(state, dst_op, op1)
            }
            RegisterSet::Single(_) => todo!(),
            RegisterSet::Vector(_, _) => todo!(),
        }
    }
}

/// Read a single value and cast it to the specified type if required.
fn read_single_as<'a, T>(state: &'a RegState, operand: &Operand) -> Result<T, ()>
where
    T: CastSingleAny + Default,
{
    match operand {
        Operand::Reg(reg) => match reg.set {
            RegisterSet::Single(RegType::UnsignedInt(u32::BITS)) => {
                let v: u32 = state.read(reg.index).unwrap_or_default();
                Ok(v.cast_into())
            }
            RegisterSet::Single(RegType::UnsignedInt(u64::BITS)) => {
                let v: u64 = state.read(reg.index).unwrap_or_default();
                Ok(v.cast_into())
            }
            RegisterSet::Single(RegType::UnsignedInt(w)) => {
                let v: ArbitraryUnsignedInt = state
                    .read_arb(reg.index, w)
                    .map(|v| v.cast_into())
                    .unwrap_or_default();

                Ok(v.cast_into())
            }
            RegisterSet::Single(RegType::SignedInt(_)) => todo!(),
            RegisterSet::Single(RegType::Float(_)) => todo!(),
            RegisterSet::Single(RegType::InstructionAddress) => Err(()),
            RegisterSet::Single(RegType::MemoryAddress) => Err(()),
            RegisterSet::Vector(_, _) => Err(()),
        },
        Operand::UnsignedConstant(c) => Ok((*c).cast_into()),
        Operand::LabelConstant(_) => Err(()),
    }
}

fn read_single_as_iaddress(state: &RegState, operand: &Operand) -> Result<InstructionAddress, ()> {
    match operand {
        Operand::Reg(reg) => match reg.set {
            RegisterSet::Single(RegType::InstructionAddress) => {
                let v: InstructionAddress = state.read(reg.index).unwrap_or_default();
                Ok(v.cast_into())
            }
            _ => read_single_as(state, operand),
        },
        Operand::UnsignedConstant(c) => Ok(InstructionAddress::new(*c as usize)),
        Operand::LabelConstant(c) => Ok(InstructionAddress::new(*c)),
    }
}

fn read_vector_as<'a, T>(state: &'a RegState, operand: &Operand) -> Option<Vec<T>>
where
    T: CastSingleAny + Default,
{
    fn cast_vec<T, F>(slice: &[F]) -> Vec<T>
    where
        F: CastInto<T>,
    {
        slice.iter().map(|v| v.cast_into()).collect()
    }
    match operand {
        Operand::Reg(reg) => match reg.set {
            RegisterSet::Single(_) => None,
            RegisterSet::Vector(RegType::UnsignedInt(u32::BITS), l) => {
                let slice: &[u32] = state.read_multi(reg.index, l as usize)?;
                Some(cast_vec(slice))
            }
            RegisterSet::Vector(_, _) => todo!(),
        },
        Operand::UnsignedConstant(_) => None,
        Operand::LabelConstant(_) => None,
    }
}
