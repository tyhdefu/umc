use std::collections::HashMap;
use std::ops::RangeInclusive;

use crate::ast::{self, OperandWithLoc};
use crate::bytecode as bc;
use crate::bytecode::{Instruction, RegOperand};
use crate::model::{RegType, RegisterSet};

type Loc = RangeInclusive<usize>;

pub enum AssembleError {
    DuplicateLabel(String, Loc),
    InvalidInstruction(AssembleInstructionError, Loc),
}

#[derive(Debug)]
pub enum AssembleInstructionError {
    UnknownOpCode(Loc),
    InvalidOperandCount(usize, usize),
    InvalidOperand(InvalidOperandError, usize, Loc),
}

impl AssembleInstructionError {
    pub fn invalid_op(error: InvalidOperandError, operand: &OperandWithLoc) -> Self {
        Self::InvalidOperand(error, operand.1, operand.2.clone())
    }

    pub fn unknown_opcode(instr: &ast::Instruction) -> Self {
        let opcode_loc = *instr.loc.start()..=(instr.loc.start() + instr.opcode.len() - 1); // TODO?
        Self::UnknownOpCode(opcode_loc)
    }
}

#[derive(Debug)]
pub enum InvalidOperandError {
    ExpectedDstReg,
    CannotInferReg,
    UnknownLabel(String),
    /// The type of the operand does not agree with the destination / instruction
    InvalidType,
}

impl AssembleError {}

impl AssembleError {
    pub fn bad_instruction(error: AssembleInstructionError, instr: &ast::Instruction) -> Self {
        Self::InvalidInstruction(error, instr.loc.clone())
    }
}

pub fn compile_prog(
    ast_prog: Vec<ast::Statement>,
) -> Result<Vec<bc::Instruction>, Vec<AssembleError>> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut prog = Vec::new();

    let mut errors = vec![];

    for (i, statement) in ast_prog.iter().enumerate() {
        if let Some(label) = &statement.label {
            if labels.insert(label.0.clone(), i).is_some() {
                errors.push(AssembleError::DuplicateLabel(
                    label.0.to_owned(),
                    label.1.clone(),
                ));
            }
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    for statement in ast_prog.into_iter() {
        match ast_to_bytecode(statement.instr.clone(), &labels) {
            Ok(bc) => prog.push(bc),
            Err(e) => errors.push(AssembleError::bad_instruction(e, &statement.instr)),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(prog)
}

fn ops<const N: usize>(
    instr: &ast::Instruction,
) -> Result<&[OperandWithLoc; N], AssembleInstructionError> {
    let slice: &[OperandWithLoc] = &instr.operands;
    slice
        .try_into()
        .map_err(|_| AssembleInstructionError::InvalidOperandCount(N, slice.len()))
}

pub fn ast_to_bytecode(
    instr: ast::Instruction,
    labels: &HashMap<String, usize>,
) -> Result<bc::Instruction, AssembleInstructionError> {
    match instr.opcode.as_str() {
        "mov" => {
            let [op1, op2] = ops(&instr)?;
            let dst = parse_dst_reg(&op1)?;
            let operand = parse_reg_or_constant(&op2, &dst.set, labels)?;

            // TODO: that the type of these can be converted to ints

            Ok(Instruction::Mov(dst, operand))
        }
        "add" => {
            let [op1, op2, op3] = ops(&instr)?;
            let dst = parse_dst_reg(&op1)?;
            let operand1 = parse_reg_or_constant(&op2, &dst.set, labels)?;
            let operand2 = parse_reg_or_constant(&op3, &dst.set, labels)?;

            Ok(Instruction::Add(dst, operand1, operand2))
        }
        "and" => {
            let [op1, op2, op3] = ops(&instr)?;
            let dst = parse_dst_reg(&op1)?;
            let operand1 = parse_reg_or_constant(&op2, &dst.set, labels)?;
            let operand2 = parse_reg_or_constant(&op3, &dst.set, labels)?;
            Ok(Instruction::And(dst, operand1, operand2))
        }
        "xor" => {
            let [op1, op2, op3] = ops(&instr)?;
            let dst = parse_dst_reg(&op1)?;
            let operand1 = parse_reg_or_constant(&op2, &dst.set, labels)?;
            let operand2 = parse_reg_or_constant(&op3, &dst.set, labels)?;
            Ok(Instruction::Xor(dst, operand1, operand2))
        }
        "not" => {
            let [op1, op2] = ops(&instr)?;
            let dst = parse_dst_reg(&op1)?;
            let operand1 = parse_reg_or_constant(&op2, &dst.set, labels)?;

            Ok(Instruction::Not(dst, operand1))
        }
        "jmp" => {
            let [op1] = ops(&instr)?;
            let dst = parse_address_operand(&op1, labels)?;
            Ok(Instruction::Jmp(dst))
        }
        "bz" => {
            let [op1, op2] = ops(&instr)?;
            let dst = parse_address_operand(&op1, labels)?;
            let operand1 = parse_int_reg(&op2)?;
            Ok(Instruction::Bz(dst, operand1))
        }
        "bnz" => {
            let [op1, op2] = ops(&instr)?;
            let dst = parse_address_operand(&op1, labels)?;
            let operand1 = parse_int_reg(&op2)?;
            Ok(Instruction::Bnz(dst, operand1))
        }
        "dbg" => {
            let [op1] = ops(&instr)?;
            // acts like a dst reg, as it cannot be inferred
            let operand = parse_dst_reg(&op1)?;
            Ok(Instruction::Dbg(operand))
        }
        _ => Err(AssembleInstructionError::unknown_opcode(&instr)),
    }
}

fn parse_dst_reg(operand: &OperandWithLoc) -> Result<bc::RegOperand, AssembleInstructionError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => match &reg.set {
            None => {
                return Err(AssembleInstructionError::invalid_op(
                    InvalidOperandError::CannotInferReg,
                    &operand,
                ));
            }
            Some(reg_set) => Ok(RegOperand {
                set: reg_set.clone(),
                index: reg.index,
            }),
        },
        _ => {
            return Err(AssembleInstructionError::invalid_op(
                InvalidOperandError::ExpectedDstReg,
                &operand,
            ));
        }
    }
}

fn parse_reg_or_constant(
    operand: &OperandWithLoc,
    infer_as: &RegisterSet,
    labels: &HashMap<String, usize>,
) -> Result<bc::Operand, AssembleInstructionError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => Ok(bc::Operand::Reg(infer_reg(reg.clone(), infer_as))),
        ast::Operand::Constant(x) => Ok(bc::Operand::UnsignedConstant(*x)),
        // Labels are only allowed if we are inferring as the i-address type (destination is an i-address register)
        ast::Operand::Label(label) => match infer_as {
            RegisterSet::Single(RegType::InstructionAddress) => {
                let pc = labels.get(label).ok_or_else(|| {
                    AssembleInstructionError::invalid_op(
                        InvalidOperandError::UnknownLabel(label.to_owned()),
                        operand,
                    )
                })?;
                Ok(bc::Operand::LabelConstant(*pc))
            }
            _ => Err(AssembleInstructionError::invalid_op(
                InvalidOperandError::CannotInferReg,
                operand,
            )),
        },
    }
}

fn parse_int_reg(operand: &OperandWithLoc) -> Result<bc::Operand, AssembleInstructionError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => Ok(bc::Operand::Reg(RegOperand {
            set: reg
                .set
                .as_ref()
                .ok_or_else(|| {
                    AssembleInstructionError::invalid_op(
                        InvalidOperandError::CannotInferReg,
                        operand,
                    )
                })?
                .clone(),
            index: reg.index,
        })),
        ast::Operand::Constant(c) => Ok(bc::Operand::UnsignedConstant(*c)),
        ast::Operand::Label(_) => Err(AssembleInstructionError::invalid_op(
            InvalidOperandError::InvalidType,
            operand,
        )),
    }
}

fn parse_address_operand(
    operand: &OperandWithLoc,
    labels: &HashMap<String, usize>,
) -> Result<bc::Operand, AssembleInstructionError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => match &reg.set {
            Some(RegisterSet::Single(RegType::InstructionAddress)) => {
                Ok(bc::Operand::Reg(RegOperand {
                    set: RegisterSet::Single(RegType::InstructionAddress),
                    index: reg.index,
                }))
            }
            Some(_) => Err(AssembleInstructionError::invalid_op(
                InvalidOperandError::InvalidType,
                operand,
            )),
            None => Err(AssembleInstructionError::invalid_op(
                InvalidOperandError::CannotInferReg,
                operand,
            )), // TODO: Could infer as address register?
        },
        ast::Operand::Constant(_) => Err(AssembleInstructionError::invalid_op(
            InvalidOperandError::InvalidType,
            operand,
        )),
        ast::Operand::Label(label) => {
            let pc = labels.get(label).ok_or_else(|| {
                AssembleInstructionError::invalid_op(
                    InvalidOperandError::UnknownLabel(label.to_owned()),
                    operand,
                )
            })?;
            Ok(bc::Operand::LabelConstant(*pc))
        }
    }
}

fn infer_reg(reg: ast::ASTRegisterOperand, default: &RegisterSet) -> bc::RegOperand {
    bc::RegOperand {
        set: reg.set.unwrap_or_else(|| default.clone()),
        index: reg.index,
    }
}
