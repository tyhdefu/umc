//! Translate unvalidated Instruction-Operand form to a form
//! that is guaranteed to execute

use crate::instructions::{
    AddParams, AnyCoherentNumOp, CompareParams, CompareToZero, ConsistentComparison,
    ConsistentNumOp, MovParams, NotParams,
};
use crate::operand::{Operand, RegOperand};
use crate::reg_model::{
    FloatRegT, InstrRegT, NumReg, Reg, RegOrConstant, SignedRegT, UnsignedRegT,
};
use crate::{NumRegType, RegType, RegisterSet};

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
                match AnyCoherentNumOp::try_from(value)? {
                    AnyCoherentNumOp::UnsignedInt(num_op) => Self::UnsignedInt(num_op),
                    AnyCoherentNumOp::SignedInt(num_op) => Self::SignedInt(num_op),
                    AnyCoherentNumOp::Float(num_op) => Self::Float(num_op),
                }
            }
            RegisterSet::Single(RegType::MemoryAddress) => {
                let p1 = Reg::from_mem_reg(ops[1])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;
                let p2 = RegOrConstant::from_signed(ops[2])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;
                Self::MemAddress(Reg(reg_op.index), p1, p2)
            }
            RegisterSet::Single(RegType::InstructionAddress) => {
                let p1 = RegOrConstant::from_instr_addr(ops[1])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;
                let p2 = RegOrConstant::from_signed(ops[2])
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 2 })?;
                Self::InstrAddress(Reg(reg_op.index), p1, p2)
            }
            RegisterSet::Vector(_, _) => todo!(),
        })
    }
}

impl TryFrom<&[&Operand]> for AnyCoherentNumOp {
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
                        Ok(Self::UnsignedInt(ConsistentNumOp::Single(
                            Reg(NumReg {
                                index: reg_op.index,
                                width: *w,
                            }),
                            p1,
                            p2,
                        )))
                    }
                    NumRegType::SignedInt(w) => {
                        let p1 = RegOrConstant::from_signed(ops[1])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 1 })?;
                        let p2 = RegOrConstant::from_signed(ops[2])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 2 })?;
                        Ok(Self::SignedInt(ConsistentNumOp::Single(
                            Reg(NumReg {
                                index: reg_op.index,
                                width: *w,
                            }),
                            p1,
                            p2,
                        )))
                    }
                    NumRegType::Float(w) => {
                        let p1 = RegOrConstant::from_float(ops[1])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 1 })?;
                        let p2 = RegOrConstant::from_float(ops[2])
                            .map_err(|_| Self::Error::InconsistentOperand { op_index: 2 })?;
                        Ok(Self::Float(ConsistentNumOp::Single(
                            Reg(NumReg {
                                index: reg_op.index,
                                width: *w,
                            }),
                            p1,
                            p2,
                        )))
                    }
                },
                _ => Err(Self::Error::InvalidRegType { op_index: 0 }),
            },
            RegisterSet::Vector(reg_type, _) => match reg_type {
                RegType::Num(num_reg) => match num_reg {
                    NumRegType::UnsignedInt(_) => todo!(),
                    NumRegType::SignedInt(_) => todo!(),
                    NumRegType::Float(_) => todo!(),
                },
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
                    Reg(NumReg {
                        index: dst.index,
                        width: *w,
                    }),
                    p_unsigned,
                ))
            }
            NumRegType::SignedInt(_) => todo!("signed operands not implemented yet"),
            NumRegType::Float(w) => {
                let p_float = RegOrConstant::from_float(p)
                    .map_err(|_| InstructionValidateError::InconsistentOperand { op_index: 1 })?;

                // Don't allow narrowing
                if p_float.width().is_some_and(|w2| w2 > *w) {
                    return Err(InstructionValidateError::CannotNarrowWidth { op_index: 1 });
                }
                Ok(MovParams::Float(
                    Reg(NumReg {
                        index: dst.index,
                        width: *w,
                    }),
                    p_float,
                ))
            }
        },
        RegisterSet::Single(RegType::InstructionAddress) => match p {
            Operand::Reg(r) => Ok(MovParams::InstrAddress(
                Reg(dst.index),
                RegOrConstant::reg(r.index),
            )),
            Operand::LabelConstant(l) => Ok(MovParams::InstrAddress(
                Reg(dst.index),
                RegOrConstant::Const(*l),
            )),
            _ => Err(InstructionValidateError::InconsistentOperand { op_index: 1 }),
        },
        RegisterSet::Single(RegType::MemoryAddress) => todo!(),
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
                }) => RegOrConstant::reg(NumReg {
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
                    unsigned(RegOrConstant::reg(NumReg {
                        index: reg.index,
                        width: *width,
                    }))
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))) => {
                    signed(RegOrConstant::reg(NumReg {
                        index: reg.index,
                        width: *width,
                    }))
                }
                RegisterSet::Single(RegType::Num(NumRegType::Float(width))) => {
                    float(RegOrConstant::reg(NumReg {
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
                        return Ok(Self::MemAddressCompare(Reg(reg.index), Reg(*index)));
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

impl TryFrom<&Operand> for CompareToZero {
    type Error = ();

    fn try_from(value: &Operand) -> Result<Self, Self::Error> {
        Ok(match value {
            Operand::Reg(reg) => match reg.set {
                RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(width))) => {
                    Self::Unsigned(RegOrConstant::reg(NumReg {
                        index: reg.index,
                        width,
                    }))
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))) => {
                    Self::Signed(RegOrConstant::reg(NumReg {
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
