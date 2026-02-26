use std::cmp::Ordering;
use std::iter::repeat_n;

use crate::vm::environment::{ECallCode, Environment};
use crate::vm::memory::safe::{SafeAddress, SafeMemoryManager};
use crate::vm::memory::{MemoryAccessError, MemoryManager, Serializable};
use crate::vm::state::{RegState as RegStateRaw, StoreFor, StorePrim};
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::int::ArbitraryInt;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::vector::VecValue;
use crate::vm::types::{
    BinaryArithmeticOp, BinaryBitwiseOp, BinaryOp, CastInto, CastSingleFloat, CastSingleSigned,
    CastSingleUnsigned, MovOp, NotOp, UMCOffset,
};
use crate::vm::widths::int::IntWidth;
use crate::vm::widths::uint::UIntWidth;
use crate::vm::widths::{WidthBinaryOp, WidthOptions, WidthUnaryOp};
use umc_model::RegWidth;
use umc_model::instructions::{
    AddParams, AnyConsistentNumOp, AnyReg, AnySingleReg, AnySingleRegOrConstant, BinaryCondition,
    CompareParams, CompareToZero, ConsistentComparison, ConsistentOp, ECallParams, MovParams,
    NotParams, OffsetOp, ResizeCast, SimpleCast,
};
use umc_model::reg_model::{
    FloatRegT, InstrRegT, MemRegT, Reg, RegOrConstant, RegTypeT, SignedRegT, UnsignedRegT,
};

// TODO: Make these helpers and the VM have a switchable Memory implementation
type RegState = RegStateRaw<SafeAddress>;

pub fn execute_mov(params: &MovParams, state: &mut RegState, memory_constants: &Vec<SafeAddress>) {
    match params {
        MovParams::UnsignedInt(r, reg_or_constant) => {
            let domain = UIntWidth::from_width(r.width);
            domain.operate_unary_in_domain(*r, reg_or_constant, state, &MovOp);
        }
        MovParams::SignedInt(r, reg_or_constant) => {
            let domain = IntWidth::from_width(r.width);
            domain.operate_unary_in_domain(*r, reg_or_constant, state, &MovOp);
        }
        MovParams::Float(r, reg_or_constant) => {
            let num_op = AnyConsistentNumOp::Float(ConsistentOp::Single(
                r.clone(),
                reg_or_constant.clone(),
                RegOrConstant::Const(0.0),
            ));
            execute_arithmetic(&num_op, BinaryArithmeticOp::Add, state);
        }
        MovParams::MemAddress(dst, p) => match read_mem_addr(p, state, memory_constants) {
            Some(v) => state.store(*dst, v.clone()),
            None => {}
        },
        MovParams::InstrAddress(dst, p) => {
            let addr = read_iaddr(p, state);
            state.store(*dst, addr);
        }
    }
}

fn compute_binary<T, F, R, O>(op: O, read: F, p1: &R, p2: &R) -> T
where
    O: BinaryOp<T>,
    F: Fn(&R) -> T,
{
    let mut p1: T = read(p1);
    let p2: T = read(p2);
    op.operate(&mut p1, &p2);
    p1
}

fn compute_float<O, T>(
    op: O,
    dst: &Reg<FloatRegT>,
    p1: &RegOrConstant<FloatRegT>,
    p2: &RegOrConstant<FloatRegT>,
    state: &mut RegState,
) where
    O: BinaryOp<T>,
    T: CastSingleFloat + Copy,
    RegState: StorePrim<T, FloatRegT>,
{
    let result: T = compute_binary(op, |r| read_float(r, state), p1, p2);
    state.store_prim(*dst, result);
}

pub fn execute_add(params: &AddParams, state: &mut RegState, memory_constants: &Vec<SafeAddress>) {
    match params {
        AddParams::UnsignedInt(consistent_op) => execute_arithmetic(
            &AnyConsistentNumOp::UnsignedInt(consistent_op.clone()),
            BinaryArithmeticOp::Add,
            state,
        ),
        AddParams::SignedInt(consistent_op) => execute_arithmetic(
            &AnyConsistentNumOp::SignedInt(consistent_op.clone()),
            BinaryArithmeticOp::Add,
            state,
        ),
        AddParams::Float(consistent_op) => execute_arithmetic(
            &AnyConsistentNumOp::Float(consistent_op.clone()),
            BinaryArithmeticOp::Add,
            state,
        ),
        AddParams::MemAddress(dst, reg, offset) => {
            let mut address = read_mem_addr(reg, state, memory_constants)
                .expect("Tried to add to an unset memory register")
                .clone();
            // TODO: Arbitrary Unsigned or specialisation
            let offset_bytes: isize = read_offset(offset, state);
            address.offset(offset_bytes);
            state.store(*dst, address);
        }
        AddParams::InstrAddress(dst, reg_or_constant, offset) => {
            let mut iaddr = read_iaddr(reg_or_constant, state);
            let offset_bytes: isize = read_offset(offset, state);
            iaddr.offset(offset_bytes);
            state.store(*dst, iaddr);
        }
    }
}

pub fn execute_arithmetic(
    params: &AnyConsistentNumOp,
    op: BinaryArithmeticOp,
    state: &mut RegState,
) {
    match params {
        AnyConsistentNumOp::UnsignedInt(param_kind) => match param_kind {
            ConsistentOp::Single(dst, p1, p2) => {
                let domain = UIntWidth::from_width(dst.width);
                domain.operate_binary_in_domain(*dst, p1, p2, state, &op);
            }
            ConsistentOp::VectorBroadcast(params) => {
                let domain = UIntWidth::from_width(params.dst().width);
                domain.operate_binary_broadcast_in_domain(params, state, &op);
            }
            ConsistentOp::VectorVector(params) => {
                let domain = UIntWidth::from_width(params.dst().width);
                domain.operate_binary_vector_in_domain(params, &op, state);
            }
        },
        AnyConsistentNumOp::SignedInt(param_kind) => match param_kind {
            ConsistentOp::Single(dst, p1, p2) => {
                let domain = IntWidth::from_width(dst.width);
                domain.operate_binary_in_domain(*dst, p1, p2, state, &op);
            }
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
        AnyConsistentNumOp::Float(param_kind) => match param_kind {
            ConsistentOp::Single(dst, p1, p2) => match dst.width {
                32 => compute_float::<_, f32>(op, dst, p1, p2, state),
                64 => compute_float::<_, f64>(op, dst, p1, p2, state),
                _ => panic!("Floats must be 32 or 64-bit"),
            },
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
    }
}

pub fn execute_bitwise(params: &AnyConsistentNumOp, op: BinaryBitwiseOp, state: &mut RegState) {
    match params {
        AnyConsistentNumOp::UnsignedInt(num_op) => match num_op {
            ConsistentOp::Single(dst, p1, p2) => {
                let domain = UIntWidth::from_width(dst.width);
                domain.operate_binary_in_domain(*dst, p1, p2, state, &op);
            }
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
        AnyConsistentNumOp::SignedInt(num_op) => match num_op {
            ConsistentOp::Single(dst, p1, p2) => {
                let domain = IntWidth::from_width(dst.width);
                domain.operate_binary_in_domain(*dst, p1, p2, state, &op);
            }
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
        AnyConsistentNumOp::Float(_) => panic!("TODO: Make new num op for bitwise"),
    }
}

pub fn execute_comparison(
    cond: &BinaryCondition,
    params: &CompareParams,
    state: &mut RegState,
    memory_constants: &Vec<SafeAddress>,
) {
    let result = compare(&params.args, state, memory_constants)
        .map(|r| match cond {
            BinaryCondition::Equal => r.is_eq(),
            BinaryCondition::GreaterThan => r.is_gt(),
            BinaryCondition::GreaterThanOrEqualTo => r.is_ge(),
            BinaryCondition::LessThan => r.is_lt(),
            BinaryCondition::LessThanOrEqualTo => r.is_le(),
        })
        .unwrap_or(false);
    let dst = &params.dst;
    UIntWidth::store_u64(*dst, state, result as u64);
}

pub fn compare(
    comparison: &ConsistentComparison,
    state: &RegState,
    memory_constants: &Vec<SafeAddress>,
) -> Option<Ordering> {
    match comparison {
        ConsistentComparison::UnsignedCompare(op1, op2) => UIntWidth::compare(op1, op2, state),
        ConsistentComparison::SignedCompare(op1, op2) => IntWidth::compare(op1, op2, state),
        ConsistentComparison::FloatCompare(op1, op2) => {
            let width = op1.width().or(op2.width()).unwrap_or(64);
            match width {
                w if w <= 32 => {
                    let v1: f32 = read_float(op1, state);
                    let v2: f32 = read_float(op2, state);
                    v1.partial_cmp(&v2)
                }
                w if w <= 64 => {
                    let v1: f64 = read_float(op1, state);
                    let v2: f64 = read_float(op2, state);
                    v1.partial_cmp(&v2)
                }
                _ => panic!("Only 32-bit or 64-bit floats supported"),
            }
        }
        ConsistentComparison::MemAddressCompare(op1, op2) => {
            match (
                read_mem_addr(op1, state, memory_constants),
                read_mem_addr(op2, state, memory_constants),
            ) {
                (Some(m1), Some(m2)) => m1.partial_cmp(m2),
                _ => None,
            }
        }
        ConsistentComparison::InstrAddressCompare(op1, op2) => {
            let x = read_iaddr(op1, state);
            let y = read_iaddr(op2, state);
            x.partial_cmp(&y)
        }
    }
}

pub fn execute_not(params: &NotParams, state: &mut RegState) {
    match params {
        NotParams::UnsignedInt(d, p) => {
            UIntWidth::from_width(d.width).operate_unary_in_domain(*d, p, state, &NotOp);
        }
        NotParams::SignedInt(..) => todo!(),
    }
}

pub fn execute_load(
    reg: &AnySingleReg,
    mem_addr: &RegOrConstant<MemRegT>,
    state: &mut RegState,
    memory: &SafeMemoryManager,
    memory_constants: &Vec<SafeAddress>,
) -> Result<(), MemoryAccessError<SafeAddress>> {
    fn load_prim<RT: RegTypeT, T>(
        reg: Reg<RT>,
        address: &SafeAddress,
        state: &mut RegState,
        memory: &SafeMemoryManager,
    ) -> Result<(), MemoryAccessError<SafeAddress>>
    where
        T: Serializable + Copy,
        RegState: StorePrim<T, RT>,
    {
        let val: T = memory.load_prim(address)?;
        state.store_prim(reg, val);
        Ok(())
    }

    // If the memory register was never set, it is an invalid address
    let address: SafeAddress = read_mem_addr(mem_addr, state, memory_constants)
        .ok_or(MemoryAccessError::InvalidAddress(SafeAddress::NULL))?
        .clone();

    match reg {
        AnySingleReg::Unsigned(reg) => match reg.width {
            u32::BITS => load_prim::<_, u32>(*reg, &address, state, memory),
            u64::BITS => load_prim::<_, u64>(*reg, &address, state, memory),
            w => {
                let val: ArbitraryUnsignedInt = memory.load(w as usize, &address).unwrap();
                state.store(*reg, val);
                Ok(())
            }
        },
        AnySingleReg::Signed(reg) => match reg.width {
            i32::BITS => load_prim::<_, i32>(*reg, &address, state, memory),
            i64::BITS => load_prim::<_, i64>(*reg, &address, state, memory),
            _ => todo!(),
        },
        AnySingleReg::Float(reg) => match reg.width {
            32 => load_prim::<_, f32>(*reg, &address, state, memory),
            64 => load_prim::<_, f64>(*reg, &address, state, memory),
            _ => panic!("Only 32-bit and 64-bit floats supported"),
        },
        AnySingleReg::Instr(_) => todo!(),
        AnySingleReg::Mem(_) => todo!(),
    }
}

pub fn execute_store(
    reg: &AnySingleReg,
    mem_addr: &RegOrConstant<MemRegT>,
    state: &RegState,
    memory: &mut SafeMemoryManager,
    memory_constants: &Vec<SafeAddress>,
) -> Result<(), MemoryAccessError<SafeAddress>> {
    fn store_prim<RT: RegTypeT, T>(
        reg: Reg<RT>,
        address: &SafeAddress,
        state: &RegState,
        memory: &mut SafeMemoryManager,
    ) -> Result<(), MemoryAccessError<SafeAddress>>
    where
        T: Serializable + Copy + Default,
        RegState: StorePrim<T, RT>,
    {
        let val: T = state.read_prim(reg.clone()).unwrap_or_default();
        memory.store_prim(val, address)
    }

    // If the memory register was never set, it is an invalid address
    let address: SafeAddress = read_mem_addr(mem_addr, state, memory_constants)
        .ok_or(MemoryAccessError::InvalidAddress(SafeAddress::NULL))?
        .clone();

    match reg {
        AnySingleReg::Unsigned(reg) => UIntWidth::store_into_memory(*reg, state, memory, &address),
        AnySingleReg::Signed(reg) => IntWidth::store_into_memory(*reg, state, memory, &address),
        AnySingleReg::Float(reg) => match reg.width {
            32 => store_prim::<_, f32>(*reg, &address, state, memory),
            64 => store_prim::<_, f64>(*reg, &address, state, memory),
            _ => panic!("Only 32-bit and 64-bit floats supported"),
        },
        AnySingleReg::Instr(_) => todo!(),
        AnySingleReg::Mem(_) => todo!(),
    }
}

pub fn execute_simple_cast(cast: &SimpleCast, state: &mut RegState) {
    // Note that most of these simple casts are performed by read_uint itself
    match cast {
        SimpleCast::Resize(ResizeCast::Unsigned(dst, p)) => {
            let domain = UIntWidth::from_width(dst.width);
            domain.operate_unary_in_domain(*dst, p, state, &MovOp);
        }
        SimpleCast::Resize(ResizeCast::Signed(dst, p)) => {
            let domain = IntWidth::from_width(dst.width);
            domain.operate_unary_in_domain(*dst, p, state, &MovOp);
        }
        SimpleCast::Resize(ResizeCast::Float(dst, p)) => match dst.width {
            32 => {
                let v: f32 = read_float(&p, state);
                state.store_prim(*dst, v);
            }
            64 => {
                let v: f64 = read_float(&p, state);
                state.store_prim(*dst, v);
            }
            _ => panic!("Only 32-bit or 64-bit floats supported"),
        },
        SimpleCast::IgnoreSigned(c) => match c.width() {
            32 => {
                let v: i32 = read_int(c.from(), state);
                state.store_prim(*c.dst(), v as u32);
            }
            64 => {
                let v: i64 = read_int(c.from(), state);
                state.store_prim(*c.dst(), v as u64);
            }
            _ => {
                todo!("Arbitrary unsigned integers unsupported");
            }
        },
        SimpleCast::AddSign(c) => match c.width() {
            32 => {
                let v: u32 = read_uint(c.from(), state);
                state.store_prim(*c.dst(), v as i32);
            }
            64 => {
                let v: u64 = read_uint(c.from(), state);
                state.store_prim(*c.dst(), v as i64);
            }
            _ => {
                let v: ArbitraryUnsignedInt = read_uint(c.from(), state);
                todo!("Arbitrary signed integers unsupported");
            }
        },
    }
}

#[derive(Debug)]
pub enum ECallError {
    InvalidECallCode(u32),
    /// List of arguments did not match the expected argument format
    InvalidArguments,
    /// A particular argument had an invalid value
    InvalidArgValue(usize),
    InvalidDestination,
}

pub fn execute_ecall<E: Environment>(
    ecall: &ECallParams,
    state: &mut RegStateRaw<SafeAddress>,
    memory_state: &mut SafeMemoryManager,
    memory_constants: &Vec<SafeAddress>,
    environment: &mut E,
) -> Result<(), ECallError> {
    let ecall_code: u32 = read_uint(&ecall.code, state);
    let ecall_code: ECallCode = ecall_code
        .try_into()
        .map_err(|x| ECallError::InvalidECallCode(x))?;

    fn args<const N: usize>(
        slice: &[AnySingleRegOrConstant],
    ) -> Result<&[AnySingleRegOrConstant; N], ECallError> {
        slice.try_into().map_err(|_| ECallError::InvalidArguments)
    }

    match ecall_code {
        ECallCode::EXIT => todo!(),
        ECallCode::OPEN => {
            let dst_reg = match ecall.dst {
                AnyReg::Single(AnySingleReg::Unsigned(r)) => r,
                _ => return Err(ECallError::InvalidDestination),
            };

            let [filename] = args(&ecall.args)?;
            let mem_addr = match filename {
                AnySingleRegOrConstant::Mem(x) => x,
                _ => return Err(ECallError::InvalidArguments),
            };
            // TODO: Handle these errors better
            let filename_ptr = read_mem_addr(mem_addr, state, memory_constants)
                .ok_or(ECallError::InvalidArgValue(0))?;
            let filename = memory_state
                .get_null_terminated(filename_ptr)
                .map_err(|_| ECallError::InvalidArgValue(0))?;
            let filename_str =
                str::from_utf8(filename).map_err(|_| ECallError::InvalidArgValue(0))?;

            // TODO: Open file failed
            let file_handle = environment.open(filename_str).unwrap();

            UIntWidth::store_u64(dst_reg, state, file_handle as u64);
        }
        ECallCode::CLOSE => {
            let dst_reg = match ecall.dst {
                AnyReg::Single(AnySingleReg::Unsigned(r)) => r,
                _ => return Err(ECallError::InvalidDestination),
            };

            let [file_handle] = args(&ecall.args)?;
            let file_handle: u32 = match file_handle {
                AnySingleRegOrConstant::Unsigned(x) => read_uint(x, state),
                _ => return Err(ECallError::InvalidArguments),
            };
            let suc = environment.close(file_handle).is_err();
            UIntWidth::store_u64(dst_reg, state, suc as u64);
            return Ok(());
        }
        ECallCode::READ => {
            let dst_reg = match ecall.dst {
                AnyReg::Single(AnySingleReg::Unsigned(r)) => r,
                _ => return Err(ECallError::InvalidDestination),
            };

            let [file_handle, buf_addr, size_reg] = args(&ecall.args)?;
            let file_handle: u32 = match file_handle {
                AnySingleRegOrConstant::Unsigned(x) => read_uint(x, state),
                _ => return Err(ECallError::InvalidArguments),
            };
            let mem_addr = match buf_addr {
                AnySingleRegOrConstant::Mem(x) => {
                    read_mem_addr(x, state, memory_constants).unwrap()
                }
                _ => return Err(ECallError::InvalidArguments),
            };
            let size: u64 = match size_reg {
                AnySingleRegOrConstant::Unsigned(x) => read_uint(x, state),
                _ => return Err(ECallError::InvalidArguments),
            };
            let buf = memory_state
                .get_mut_length(mem_addr, size as usize)
                .unwrap();
            let read_bytes = environment.read(file_handle, buf).unwrap();
            UIntWidth::store_u64(dst_reg, state, read_bytes as u64);
        }
        ECallCode::WRITE => {
            let dst_reg = match ecall.dst {
                AnyReg::Single(AnySingleReg::Unsigned(r)) => r,
                _ => return Err(ECallError::InvalidDestination),
            };

            let [file_handle, buf_addr, size_reg] = args(&ecall.args)?;
            let file_handle: u32 = match file_handle {
                AnySingleRegOrConstant::Unsigned(x) => read_uint(x, state),
                _ => return Err(ECallError::InvalidArguments),
            };
            let mem_addr = match buf_addr {
                AnySingleRegOrConstant::Mem(x) => {
                    read_mem_addr(x, state, memory_constants).unwrap()
                }
                _ => return Err(ECallError::InvalidArguments),
            };
            let size: u64 = match size_reg {
                AnySingleRegOrConstant::Unsigned(x) => read_uint(x, state),
                _ => return Err(ECallError::InvalidArguments),
            };
            let buf = memory_state
                .get_mut_length(mem_addr, size as usize)
                .unwrap();
            let wrote_bytes = environment.write(file_handle, &buf).unwrap();
            UIntWidth::store_u64(dst_reg, state, wrote_bytes as u64);
        }
    }
    Ok(())
}

pub fn execute_debug(reg: &AnyReg, state: &RegState) {
    match reg {
        AnyReg::Single(AnySingleReg::Unsigned(reg)) => {
            let x: ArbitraryUnsignedInt = read_uint(&RegOrConstant::Reg(reg.clone()), state);
            if reg.width == u8::BITS {
                let v: u32 = x.cast_into();
                println!(
                    "{} = {} ('{}')",
                    reg,
                    x,
                    std::ascii::escape_default(v as u8)
                );
                return;
            }
            println!("{} = {}", reg, x);
        }
        AnyReg::Single(AnySingleReg::Signed(reg)) => {
            let reg_ref = RegOrConstant::Reg(reg.clone());
            let x: i64 = read_int(&reg_ref, state);
            println!("{} = {:X}", reg_ref, x);
        }
        AnyReg::Single(AnySingleReg::Float(reg)) => {
            let reg_ref = RegOrConstant::Reg(reg.clone());
            let x: f64 = read_float(&reg_ref, state);
            println!("{} = {}", reg_ref, x);
        }
        AnyReg::Single(AnySingleReg::Instr(reg)) => {
            let reg_ref = RegOrConstant::Reg(reg.clone());
            let x: InstructionAddress = read_iaddr(&reg_ref, state);
            println!("{} = {:?}", reg_ref, x);
        }
        AnyReg::Single(AnySingleReg::Mem(m)) => println!("{} = {:?}", m, state.read(*m)),
        AnyReg::Vector(AnySingleReg::Unsigned(reg), l) => {
            let x: Vec<ArbitraryUnsignedInt> =
                read_uint_vec(&reg, *l, state).unwrap_or_else(|| {
                    repeat_n(ArbitraryUnsignedInt::ZERO.clone(), *l as usize).collect()
                });
            println!("{}", VecValue::from_vec(x));
        }
        _ => todo!("debug on this register not yet supported"),
    }
}

pub fn read_uint<T, S>(op: &RegOrConstant<UnsignedRegT>, state: &S) -> T
where
    T: CastSingleUnsigned,
    S: StorePrim<u32, UnsignedRegT>
        + StorePrim<u64, UnsignedRegT>
        + StoreFor<ArbitraryUnsignedInt, UnsignedRegT>,
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            u32::BITS => {
                let v: u32 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            u64::BITS => {
                let v: u64 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            _ => {
                let v: &ArbitraryUnsignedInt = state
                    .read(*num_reg)
                    .unwrap_or(ArbitraryUnsignedInt::ZERO_REF);
                v.cast_into()
            }
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_uint_vec<T>(
    reg: &Reg<UnsignedRegT>,
    length: RegWidth,
    state: &RegState,
) -> Option<Vec<T>>
where
    T: CastSingleUnsigned,
{
    Some(match reg.width {
        u32::BITS => {
            let v: &VecValue<u32> = state.read_multi_prim(*reg, length as usize)?;
            v.as_slice().iter().map(|x| x.cast_into()).collect()
        }
        u64::BITS => {
            let v: &VecValue<u64> = state.read_multi_prim(*reg, length as usize)?;
            v.as_slice().iter().map(|x| x.cast_into()).collect()
        }
        _ => {
            let v: &VecValue<ArbitraryUnsignedInt> = state.read_multi(*reg, length as usize)?;
            v.as_slice().iter().map(|x| x.cast_into()).collect()
        }
    })
}

pub fn read_int<T, S>(op: &RegOrConstant<SignedRegT>, state: &S) -> T
where
    T: CastSingleSigned,
    S: StorePrim<i32, SignedRegT> + StorePrim<i64, SignedRegT>, /*+ StoreFor<ArbitraryInt, SignedRegT>*/
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            i32::BITS => {
                let v: i32 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            i64::BITS => {
                let v: i64 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            _ => {
                todo!();
            }
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_float<T>(op: &RegOrConstant<FloatRegT>, state: &RegState) -> T
where
    T: CastSingleFloat,
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            32 => {
                let v: f32 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            64 => {
                let v: f64 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            _ => panic!("Floats can only be 32-bit or 64-bit"),
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_iaddr(p: &RegOrConstant<InstrRegT>, state: &RegState) -> InstructionAddress {
    match p {
        RegOrConstant::Reg(r) => state.read(*r).copied().unwrap_or_default(),
        RegOrConstant::Const(c) => InstructionAddress::new(*c),
    }
}

pub fn read_mem_addr<'a, 'b>(
    p: &'b RegOrConstant<MemRegT>,
    state: &'a RegState,
    memory_constants: &'a Vec<SafeAddress>,
) -> Option<&'a SafeAddress> {
    match p {
        RegOrConstant::Reg(reg) => state.read(*reg),
        RegOrConstant::Const(c) => memory_constants.get(*c as usize),
    }
}

pub fn is_zero(p: &CompareToZero, state: &RegState) -> bool {
    // TODO: This isn't right
    match p {
        CompareToZero::Unsigned(r) => UIntWidth::is_zero(r, state),
        CompareToZero::Signed(r) => IntWidth::is_zero(r, state),
    }
}

pub fn read_offset(p: &OffsetOp, state: &RegState) -> isize {
    match p {
        OffsetOp::Unsigned(op) => read_uint::<u64, _>(op, state) as isize,
        OffsetOp::Signed(op) => read_int::<i64, _>(op, state) as isize,
    }
}
