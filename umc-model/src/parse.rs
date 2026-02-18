//! Translate unvalidated Instruction-Operand form to a form
//! that is guaranteed to execute

use crate::instructions::{
    AddParams, AnyConsistentNumOp, AnyReg, AnySingleReg, AnySingleRegOrConstant, CompareParams,
    CompareToZero, ConsistentComparison, ConsistentOp, IntegerCast, MovParams, NotParams, OffsetOp,
    ResizeCast, SimpleCast, VectorBroadcastParams, VectorVectorParams,
};
use crate::operand::{Operand, RegOperand};
use crate::reg_model::{
    FloatRegT, InstrRegT, MemRegT, Reg, RegOrConstant, RegTypeT, SignedRegT, UnsignedRegT,
};
use crate::{NumRegType, RegType, RegWidth, RegisterSet};

#[derive(Debug)]
pub enum InstructionValidateError {
    InvalidOpCount {
        expected: usize,
        got: usize,
    },
    ExpectedDstReg,
    CannotInferReg {
        op_index: usize,
    },
    InvalidRegType {
        op_index: usize,
    },
    InconsistentOperand {
        op_index: usize,
    },
    /// Operand inconsistent because width narrowing is not allowed implicitly
    CannotNarrowWidth {
        op_index: usize,
    },
}

impl InstructionValidateError {
    pub fn shift_op_index(&mut self, by: usize) {
        match self {
            InstructionValidateError::InvalidOpCount { expected, got } => {
                *expected += by;
                *got += by;
            }
            InstructionValidateError::ExpectedDstReg => {}
            InstructionValidateError::CannotInferReg { op_index } => *op_index += by,
            InstructionValidateError::InvalidRegType { op_index } => *op_index += by,
            InstructionValidateError::InconsistentOperand { op_index } => *op_index += by,
            InstructionValidateError::CannotNarrowWidth { op_index } => *op_index += by,
        }
    }
}

impl TryFrom<&[&Operand]> for AddParams {
    type Error = InstructionValidateError;

    fn try_from(value: &[&Operand]) -> Result<Self, Self::Error> {
        let ops: &[&Operand; 3] = ops(value)?;
        let reg_op = match ops[0] {
            Operand::Reg(reg_op) => reg_op,
            _ => return Err(InstructionValidateError::ExpectedDstReg),
        };
        Ok(match &reg_op.set {
            RegisterSet::Single(RegType::Num(_)) | RegisterSet::Vector(RegType::Num(_), _) => {
                match AnyConsistentNumOp::try_from(value)? {
                    AnyConsistentNumOp::UnsignedInt(num_op) => Self::UnsignedInt(num_op),
                    AnyConsistentNumOp::SignedInt(num_op) => Self::SignedInt(num_op),
                    AnyConsistentNumOp::Float(num_op) => Self::Float(num_op),
                }
            }
            RegisterSet::Single(RegType::MemoryAddress) => {
                let p1 = Reg::from_mem_reg(ops[1])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;
                let p2 = parse_offset(ops[2])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;
                Self::MemAddress(Reg::from_index(reg_op.index), p1, p2)
            }
            RegisterSet::Single(RegType::InstructionAddress) => {
                let p1 = RegOrConstant::from_instr_addr(ops[1])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;
                let p2 = parse_offset(ops[2])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;
                Self::InstrAddress(Reg::from_index(reg_op.index), p1, p2)
            }
            RegisterSet::Vector(RegType::MemoryAddress, _) => todo!(),
            RegisterSet::Vector(RegType::InstructionAddress, _) => todo!(),
        })
    }
}

fn parse_vector_op<RT: RegTypeT>(
    dst_reg: Reg<RT>,
    dst_length: RegWidth,
    p1: &Operand,
    p2: &Operand,
) -> Result<ConsistentOp<RT>, InstructionValidateError>
where
    VecOperand<RT>: for<'x> TryFrom<&'x Operand>,
{
    let p1: VecOperand<RT> = p1
        .try_into()
        .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;

    let p2: VecOperand<RT> = p2
        .try_into()
        .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;

    let num_op = match (p1, p2) {
        (VecOperand::Single(_), VecOperand::Single(_)) => {
            return Err(InstructionValidateError::InconsistentOperand { op_index: 1 });
        }
        (VecOperand::Vector(r1, l1), VecOperand::Single(single)) => {
            if l1 != dst_length || !dst_reg.eq_ignoring_index(&r1.clone()) {
                return Err(InstructionValidateError::InconsistentOperand { op_index: 1 });
            }
            ConsistentOp::VectorBroadcast(VectorBroadcastParams::new(
                dst_reg, dst_length, r1.index, single, false,
            ))
        }
        (VecOperand::Single(single), VecOperand::Vector(r2, l2)) => {
            if l2 != dst_length || !dst_reg.eq_ignoring_index(&r2.clone()) {
                return Err(InstructionValidateError::InconsistentOperand { op_index: 2 });
            }
            ConsistentOp::VectorBroadcast(VectorBroadcastParams::new(
                dst_reg, dst_length, r2.index, single, true,
            ))
        }
        (VecOperand::Vector(r1, l1), VecOperand::Vector(r2, l2)) => {
            if l1 != dst_length || !dst_reg.eq_ignoring_index(&r1.clone()) {
                return Err(InstructionValidateError::InconsistentOperand { op_index: 1 });
            }
            if l2 != dst_length || !dst_reg.eq_ignoring_index(&r2.clone()) {
                return Err(InstructionValidateError::InconsistentOperand { op_index: 2 });
            }
            ConsistentOp::VectorVector(VectorVectorParams::new(
                dst_reg, dst_length, r1.index, r2.index,
            ))
        }
    };
    Ok(num_op)
}

impl TryFrom<&[&Operand]> for AnyConsistentNumOp {
    type Error = InstructionValidateError;

    fn try_from(value: &[&Operand]) -> Result<Self, Self::Error> {
        let ops: &[&Operand; 3] = ops(value)?;
        let reg_op = match ops[0] {
            Operand::Reg(reg_op) => reg_op,
            _ => return Err(InstructionValidateError::ExpectedDstReg),
        };

        match &reg_op.set {
            RegisterSet::Single(reg_type) => match reg_type {
                RegType::Num(num_reg) => match num_reg {
                    NumRegType::UnsignedInt(w) => {
                        let p1 = RegOrConstant::from_unsigned(ops[1])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 1 })?;
                        let p2 = RegOrConstant::from_unsigned(ops[2])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 2 })?;
                        Ok(Self::UnsignedInt(ConsistentOp::Single(
                            Reg {
                                index: reg_op.index,
                                width: *w,
                            },
                            p1,
                            p2,
                        )))
                    }
                    NumRegType::SignedInt(w) => {
                        let p1 = RegOrConstant::from_signed(ops[1])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 1 })?;
                        let p2 = RegOrConstant::from_signed(ops[2])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 2 })?;
                        Ok(Self::SignedInt(ConsistentOp::Single(
                            Reg {
                                index: reg_op.index,
                                width: *w,
                            },
                            p1,
                            p2,
                        )))
                    }
                    NumRegType::Float(w) => {
                        let p1 = RegOrConstant::from_float(ops[1])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 1 })?;
                        let p2 = RegOrConstant::from_float(ops[2])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 2 })?;
                        Ok(Self::Float(ConsistentOp::Single(
                            Reg {
                                index: reg_op.index,
                                width: *w,
                            },
                            p1,
                            p2,
                        )))
                    }
                },
                _ => Err(Self::Error::InvalidRegType { op_index: 0 }),
            },
            RegisterSet::Vector(reg_type, l) => match reg_type {
                RegType::Num(num_reg) => Ok(match num_reg {
                    NumRegType::UnsignedInt(w) => {
                        AnyConsistentNumOp::UnsignedInt(parse_vector_op::<UnsignedRegT>(
                            Reg {
                                index: reg_op.index,
                                width: *w,
                            },
                            *l,
                            ops[1],
                            ops[2],
                        )?)
                    }
                    NumRegType::SignedInt(w) => {
                        AnyConsistentNumOp::SignedInt(parse_vector_op::<SignedRegT>(
                            Reg {
                                index: reg_op.index,
                                width: *w,
                            },
                            *l,
                            ops[1],
                            ops[2],
                        )?)
                    }
                    NumRegType::Float(w) => {
                        AnyConsistentNumOp::Float(parse_vector_op::<FloatRegT>(
                            Reg {
                                index: reg_op.index,
                                width: *w,
                            },
                            *l,
                            ops[1],
                            ops[2],
                        )?)
                    }
                }),
                _ => Err(Self::Error::InvalidRegType { op_index: 0 }),
            },
        }
    }
}

impl TryFrom<&[&Operand]> for MovParams {
    type Error = InstructionValidateError;

    fn try_from(value: &[&Operand]) -> Result<Self, Self::Error> {
        let [p1, p2] = ops(value)?;

        match p1 {
            Operand::Reg(reg) => consistent_operand(reg, p2),
            _ => Err(InstructionValidateError::ExpectedDstReg),
        }
    }
}

fn consistent_operand(
    dst: &RegOperand,
    p: &Operand,
) -> Result<MovParams, InstructionValidateError> {
    match &dst.set {
        RegisterSet::Single(RegType::Num(num_type)) => match num_type {
            NumRegType::UnsignedInt(w) => {
                let p_unsigned = RegOrConstant::from_unsigned(p)
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;

                // Don't allow narrowing
                if p_unsigned.width().is_some_and(|w2| w2 > *w) {
                    return Err(InstructionValidateError::CannotNarrowWidth { op_index: 1 });
                }
                Ok(MovParams::UnsignedInt(
                    Reg {
                        index: dst.index,
                        width: *w,
                    },
                    p_unsigned,
                ))
            }
            NumRegType::SignedInt(w) => {
                let p_signed = RegOrConstant::from_signed(p)
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;
                // Don't allow narrowing
                if p_signed.width().is_some_and(|w2| w2 > *w) {
                    return Err(InstructionValidateError::CannotNarrowWidth { op_index: 1 });
                }
                Ok(MovParams::SignedInt(
                    Reg {
                        index: dst.index,
                        width: *w,
                    },
                    p_signed,
                ))
            }
            NumRegType::Float(w) => {
                let p_float = RegOrConstant::from_float(p)
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;

                // Don't allow narrowing
                if p_float.width().is_some_and(|w2| w2 > *w) {
                    return Err(InstructionValidateError::CannotNarrowWidth { op_index: 1 });
                }
                Ok(MovParams::Float(
                    Reg {
                        index: dst.index,
                        width: *w,
                    },
                    p_float,
                ))
            }
        },
        RegisterSet::Single(RegType::InstructionAddress) => match p {
            Operand::Reg(r) => Ok(MovParams::InstrAddress(
                Reg::from_index(dst.index),
                RegOrConstant::reg(r.index),
            )),
            Operand::LabelConstant(l) => Ok(MovParams::InstrAddress(
                Reg::from_index(dst.index),
                RegOrConstant::Const(*l),
            )),
            _ => Err(InstructionValidateError::InconsistentOperand { op_index: 1 }),
        },
        RegisterSet::Single(RegType::MemoryAddress) => {
            let dst = Reg::from_index(dst.index);
            let p: RegOrConstant<MemRegT> = RegOrConstant::from_mem_addr(p)
                .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;
            Ok(MovParams::MemAddress(dst, p))
        }
        RegisterSet::Vector(_, _) => todo!("vector operands not implemented yet!"),
    }
}

impl TryFrom<&[&Operand]> for NotParams {
    type Error = InstructionValidateError;

    fn try_from(value: &[&Operand]) -> Result<Self, Self::Error> {
        let [dst, p] = ops(value)?;
        if let Operand::Reg(r) = dst {
            let params = consistent_operand(r, p)?;
            match params {
                MovParams::UnsignedInt(d, p) => Ok(NotParams::UnsignedInt(d, p)),
                MovParams::SignedInt(d, p) => Ok(NotParams::SignedInt(d, p)),
                _ => Err(InstructionValidateError::InvalidRegType { op_index: 0 }),
            }
        } else {
            Err(InstructionValidateError::ExpectedDstReg)
        }
    }
}

impl TryFrom<&[&Operand]> for CompareParams {
    type Error = InstructionValidateError;

    fn try_from(value: &[&Operand]) -> Result<Self, Self::Error> {
        let [dst, p1, p2] = ops(value)?;

        let dst_reg = Reg::from_unsigned(dst)
            .map_err(|_| InstructionValidateError::InvalidRegType { op_index: 0 })?;

        let args: ConsistentComparison =
            [*p1, *p2]
                .as_slice()
                .try_into()
                .map_err(|mut e: InstructionValidateError| {
                    e.shift_op_index(1);
                    e
                })?;

        Ok(CompareParams { dst: dst_reg, args })
    }
}

impl TryFrom<&[&Operand]> for ConsistentComparison {
    type Error = InstructionValidateError;

    fn try_from(value: &[&Operand]) -> Result<Self, Self::Error> {
        let [p1, p2] = ops(value)?;

        let unsigned = |a: RegOrConstant<UnsignedRegT>| {
            let b = RegOrConstant::from_unsigned(p2)
                .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;
            Ok(ConsistentComparison::UnsignedCompare(a, b))
        };

        let signed = |a: RegOrConstant<SignedRegT>| {
            let b = match p2 {
                Operand::Reg(RegOperand {
                    set: RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))),
                    index,
                }) => RegOrConstant::from_reg(Reg {
                    index: *index,
                    width: *width,
                }),
                Operand::SignedConstant(c) => RegOrConstant::Const(*c),
                Operand::UnsignedConstant(c) => {
                    RegOrConstant::Const((*c).try_into().map_err(|_| {
                        InstructionValidateError::InconsistentOperand { op_index: 2 }
                    })?)
                }
                _ => return Err(InstructionValidateError::InconsistentOperand { op_index: 2 }),
            };
            Ok(ConsistentComparison::SignedCompare(a, b))
        };

        let float = |a: RegOrConstant<FloatRegT>| {
            let b = RegOrConstant::from_float(p2)
                .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;
            Ok(ConsistentComparison::FloatCompare(a, b))
        };

        let iaddr = |a: RegOrConstant<InstrRegT>| {
            let b = RegOrConstant::from_instr_addr(p2)
                .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;
            Ok(ConsistentComparison::InstrAddressCompare(a, b))
        };

        match p1 {
            Operand::Reg(reg) => match &reg.set {
                RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(width))) => {
                    unsigned(RegOrConstant::from_reg(Reg {
                        index: reg.index,
                        width: *width,
                    }))
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))) => {
                    signed(RegOrConstant::from_reg(Reg {
                        index: reg.index,
                        width: *width,
                    }))
                }
                RegisterSet::Single(RegType::Num(NumRegType::Float(width))) => {
                    float(RegOrConstant::from_reg(Reg {
                        index: reg.index,
                        width: *width,
                    }))
                }
                RegisterSet::Single(RegType::InstructionAddress) => {
                    iaddr(RegOrConstant::reg(reg.index))
                }
                RegisterSet::Single(RegType::MemoryAddress) => {
                    if let Operand::Reg(RegOperand {
                        set: RegisterSet::Single(RegType::MemoryAddress),
                        index,
                    }) = p2
                    {
                        return Ok(Self::MemAddressCompare(
                            RegOrConstant::Reg(Reg::from_index(reg.index)),
                            RegOrConstant::Reg(Reg::from_index(*index)),
                        ));
                    } else {
                        return Err(InstructionValidateError::InconsistentOperand { op_index: 2 });
                    }
                }
                RegisterSet::Vector(_, _) => todo!("vector comparison not supported yet"),
            },
            Operand::UnsignedConstant(c) => unsigned(RegOrConstant::Const(*c)),
            Operand::SignedConstant(c) => signed(RegOrConstant::Const(*c)),
            Operand::FloatConstant(c) => float(RegOrConstant::Const(*c)),
            Operand::LabelConstant(c) => iaddr(RegOrConstant::Const(*c)),
        }
    }
}

impl TryFrom<&[&Operand]> for SimpleCast {
    type Error = InstructionValidateError;

    fn try_from(value: &[&Operand]) -> Result<Self, Self::Error> {
        let [p1, p2] = ops(value)?;
        let dst_reg = match p1 {
            Operand::Reg(reg) => parse_any_single_reg(reg)
                .map_err(|_| InstructionValidateError::InvalidRegType { op_index: 0 })?,
            _ => return Err(InstructionValidateError::ExpectedDstReg),
        };

        let p = parse_any_reg_or_constant(p2)
            .map_err(|_| InstructionValidateError::InvalidRegType { op_index: 1 })?;

        Ok(match dst_reg {
            AnySingleReg::Unsigned(reg) => match p {
                AnySingleRegOrConstant::Unsigned(p) => {
                    SimpleCast::Resize(ResizeCast::Unsigned(reg, p))
                }
                AnySingleRegOrConstant::Signed(p) => {
                    let cast = IntegerCast::try_create(reg, p).map_err(|_| {
                        InstructionValidateError::InconsistentOperand { op_index: 1 }
                    })?;
                    SimpleCast::IgnoreSigned(cast)
                }
                _ => return Err(InstructionValidateError::InvalidRegType { op_index: 1 }),
            },
            AnySingleReg::Signed(reg) => match p {
                AnySingleRegOrConstant::Signed(p) => SimpleCast::Resize(ResizeCast::Signed(reg, p)),
                AnySingleRegOrConstant::Unsigned(p) => {
                    let cast = IntegerCast::try_create(reg, p).map_err(|_| {
                        InstructionValidateError::InconsistentOperand { op_index: 1 }
                    })?;
                    SimpleCast::AddSign(cast)
                }
                _ => return Err(InstructionValidateError::InvalidRegType { op_index: 1 }),
            },
            AnySingleReg::Float(reg) => match p {
                AnySingleRegOrConstant::Float(p) => SimpleCast::Resize(ResizeCast::Float(reg, p)),
                _ => return Err(InstructionValidateError::InvalidRegType { op_index: 1 }),
            },
            _ => Err(InstructionValidateError::InvalidRegType { op_index: 0 })?,
        })
    }
}

impl TryFrom<&Operand> for CompareToZero {
    type Error = ();

    fn try_from(value: &Operand) -> Result<Self, Self::Error> {
        Ok(match value {
            Operand::Reg(reg) => match reg.set {
                RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(width))) => {
                    Self::Unsigned(RegOrConstant::from_reg(Reg {
                        index: reg.index,
                        width,
                    }))
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))) => {
                    Self::Signed(RegOrConstant::from_reg(Reg {
                        index: reg.index,
                        width,
                    }))
                }
                _ => return Err(()),
            },
            Operand::UnsignedConstant(c) => Self::Unsigned(RegOrConstant::Const(*c)),
            Operand::SignedConstant(c) => Self::Signed(RegOrConstant::Const(*c)),
            _ => return Err(()),
        })
    }
}

fn ops<'a, const N: usize>(
    slice: &'a [&'a Operand],
) -> Result<&'a [&'a Operand; N], InstructionValidateError> {
    slice
        .try_into()
        .map_err(|_| InstructionValidateError::InvalidOpCount {
            expected: N,
            got: slice.len(),
        })
}

enum VecOperand<RT: RegTypeT> {
    Single(RegOrConstant<RT>),
    Vector(Reg<RT>, RegWidth),
}

impl TryFrom<&Operand> for VecOperand<UnsignedRegT> {
    type Error = ();

    fn try_from(value: &Operand) -> Result<Self, Self::Error> {
        Ok(match value {
            Operand::Reg(reg_op) => match reg_op.set {
                RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(w))) => {
                    Self::Single(RegOrConstant::from_reg(Reg {
                        index: reg_op.index,
                        width: w,
                    }))
                }
                RegisterSet::Vector(RegType::Num(NumRegType::UnsignedInt(w)), l) => Self::Vector(
                    Reg {
                        index: reg_op.index,
                        width: w,
                    },
                    l,
                ),
                _ => return Err(()),
            },
            Operand::UnsignedConstant(c) => VecOperand::Single(RegOrConstant::Const(*c)),
            _ => return Err(()),
        })
    }
}

impl TryFrom<&Operand> for VecOperand<SignedRegT> {
    type Error = ();

    fn try_from(value: &Operand) -> Result<Self, Self::Error> {
        Ok(match value {
            Operand::Reg(reg_op) => match reg_op.set {
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(w))) => {
                    Self::Single(RegOrConstant::from_reg(Reg {
                        index: reg_op.index,
                        width: w,
                    }))
                }
                RegisterSet::Vector(RegType::Num(NumRegType::SignedInt(w)), l) => Self::Vector(
                    Reg {
                        index: reg_op.index,
                        width: w,
                    },
                    l,
                ),
                _ => return Err(()),
            },
            Operand::SignedConstant(c) => VecOperand::Single(RegOrConstant::Const(*c)),
            _ => return Err(()),
        })
    }
}

impl TryFrom<&Operand> for VecOperand<FloatRegT> {
    type Error = ();

    fn try_from(value: &Operand) -> Result<Self, Self::Error> {
        Ok(match value {
            Operand::Reg(reg_op) => match reg_op.set {
                RegisterSet::Single(RegType::Num(NumRegType::Float(w))) => {
                    Self::Single(RegOrConstant::from_reg(Reg {
                        index: reg_op.index,
                        width: w,
                    }))
                }
                RegisterSet::Vector(RegType::Num(NumRegType::Float(w)), l) => Self::Vector(
                    Reg {
                        index: reg_op.index,
                        width: w,
                    },
                    l,
                ),
                _ => return Err(()),
            },
            Operand::FloatConstant(c) => VecOperand::Single(RegOrConstant::Const(*c)),
            _ => return Err(()),
        })
    }
}

impl TryFrom<&Operand> for VecOperand<MemRegT> {
    type Error = ();

    fn try_from(value: &Operand) -> Result<Self, Self::Error> {
        Ok(match value {
            Operand::Reg(reg_op) => match reg_op.set {
                RegisterSet::Single(RegType::MemoryAddress) => {
                    Self::Single(RegOrConstant::reg(reg_op.index))
                }
                RegisterSet::Vector(RegType::MemoryAddress, l) => {
                    Self::Vector(Reg::from_index(reg_op.index), l)
                }
                _ => return Err(()),
            },
            _ => return Err(()),
        })
    }
}

impl TryFrom<&Operand> for VecOperand<InstrRegT> {
    type Error = ();

    fn try_from(value: &Operand) -> Result<Self, Self::Error> {
        Ok(match value {
            Operand::Reg(reg_op) => match reg_op.set {
                RegisterSet::Single(RegType::MemoryAddress) => {
                    Self::Single(RegOrConstant::reg(reg_op.index))
                }
                RegisterSet::Vector(RegType::MemoryAddress, l) => {
                    Self::Vector(Reg::from_index(reg_op.index), l)
                }
                _ => return Err(()),
            },
            Operand::LabelConstant(c) => Self::Single(RegOrConstant::Const(*c)),
            _ => return Err(()),
        })
    }
}

pub struct UnexpectedVecReg {}

pub fn parse_any_reg(reg: &RegOperand) -> AnyReg {
    let reg_type = match &reg.set {
        RegisterSet::Single(reg_type) => reg_type,
        RegisterSet::Vector(reg_type, _) => reg_type,
    };

    let any_single_reg = match reg_type {
        RegType::Num(NumRegType::UnsignedInt(w)) => AnySingleReg::Unsigned(Reg {
            index: reg.index,
            width: *w,
        }),
        RegType::Num(NumRegType::SignedInt(w)) => AnySingleReg::Signed(Reg {
            index: reg.index,
            width: *w,
        }),
        RegType::Num(NumRegType::Float(w)) => AnySingleReg::Float(Reg {
            index: reg.index,
            width: *w,
        }),
        RegType::InstructionAddress => AnySingleReg::Instr(Reg::from_index(reg.index)),
        RegType::MemoryAddress => AnySingleReg::Mem(Reg::from_index(reg.index)),
    };

    match reg.set {
        RegisterSet::Single(_) => AnyReg::Single(any_single_reg),
        RegisterSet::Vector(_, length) => AnyReg::Vector(any_single_reg, length),
    }
}

pub fn parse_any_single_reg(reg: &RegOperand) -> Result<AnySingleReg, UnexpectedVecReg> {
    match parse_any_reg(reg) {
        AnyReg::Single(reg) => Ok(reg),
        AnyReg::Vector(_, _) => Err(UnexpectedVecReg {}),
    }
}

pub fn parse_any_reg_or_constant(
    operand: &Operand,
) -> Result<AnySingleRegOrConstant, UnexpectedVecReg> {
    Ok(match operand {
        Operand::Reg(reg) => AnySingleRegOrConstant::from_any_reg(parse_any_single_reg(reg)?),
        Operand::UnsignedConstant(c) => AnySingleRegOrConstant::Unsigned(RegOrConstant::Const(*c)),
        Operand::SignedConstant(c) => AnySingleRegOrConstant::Signed(RegOrConstant::Const(*c)),
        Operand::FloatConstant(c) => AnySingleRegOrConstant::Float(RegOrConstant::Const(*c)),
        Operand::LabelConstant(c) => AnySingleRegOrConstant::Instr(RegOrConstant::Const(*c)),
    })
}

pub fn parse_offset(operand: &Operand) -> Result<OffsetOp, ()> {
    RegOrConstant::from_unsigned(operand)
        .map(|x| OffsetOp::Unsigned(x))
        .or_else(|_| RegOrConstant::from_signed(operand).map(|x| OffsetOp::Signed(x)))
}
