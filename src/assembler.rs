use std::collections::HashMap;
use std::ops::RangeInclusive;

use crate::ast::{self, OperandWithLoc};
use crate::bytecode as bc;
use crate::bytecode::{Instruction, RegOperand};
use crate::model::{RegType, RegisterSet};

#[derive(Debug)]
pub enum AssembleInstructionError {
    /// Expected size, got size
    InvalidOperandCount(usize, usize),
    ExpectedDstReg,
    CannotInferReg,
    InvalidOperand,
    UnknownOpCode,
    MissingLabel(String),
}

#[derive(Debug)]
pub enum AssembleProgError {
    DuplicateLabel(String),
    InvalidInstruction(AssembleInstructionError), // TODO: add line information
}

#[derive(Debug)]
pub enum ErrorLocation {
    /// The error was caused by a bad instruction
    Instruction(RangeInclusive<usize>),
    /// The error was caused by an invalid operand to an instruction
    Operand(usize, RangeInclusive<usize>),
}

#[derive(Debug)]
pub struct AssembleError {
    pub error: AssembleProgError,
    pub loc: ErrorLocation,
}

impl AssembleError {
    pub fn bad_instruction(error: AssembleInstructionError, instr: &ast::Instruction) -> Self {
        Self {
            error: AssembleProgError::InvalidInstruction(error),
            loc: ErrorLocation::Instruction(instr.loc.clone()),
        }
    }

    pub fn bad_op(error: AssembleInstructionError, operand: &OperandWithLoc) -> Self {
        Self {
            error: AssembleProgError::InvalidInstruction(error),
            loc: ErrorLocation::Operand(operand.1, operand.2.clone()),
        }
    }

    pub fn invalid_op(operand: &OperandWithLoc) -> Self {
        Self::bad_op(AssembleInstructionError::InvalidOperand, operand)
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
            if labels.insert(label.clone(), i).is_some() {
                //errors.push(AssembleProgError::DuplicateLabel(label.to_owned()));
            }
        }
    }

    for statement in ast_prog.into_iter() {
        match ast_to_bytecode(statement.instr, &labels) {
            Ok(bc) => prog.push(bc),
            Err(e) => errors.push(e),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(prog)
}

fn ops<const N: usize>(instr: &ast::Instruction) -> Result<&[OperandWithLoc; N], AssembleError> {
    let slice: &[OperandWithLoc] = &instr.operands;
    slice.try_into().map_err(|_| AssembleError {
        error: AssembleProgError::InvalidInstruction(
            AssembleInstructionError::InvalidOperandCount(N, instr.operands.len()),
        ),
        loc: ErrorLocation::Instruction(instr.loc.clone()),
    })
}

pub fn ast_to_bytecode(
    instr: ast::Instruction,
    labels: &HashMap<String, usize>,
) -> Result<bc::Instruction, AssembleError> {
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
        _ => Err(AssembleError {
            error: AssembleProgError::InvalidInstruction(AssembleInstructionError::UnknownOpCode),
            loc: ErrorLocation::Instruction(instr.loc.clone()),
        }),
    }
}

fn parse_dst_reg(operand: &OperandWithLoc) -> Result<bc::RegOperand, AssembleError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => match &reg.set {
            None => {
                return Err(AssembleError::bad_op(
                    AssembleInstructionError::CannotInferReg,
                    &operand,
                ));
            }
            Some(reg_set) => Ok(RegOperand {
                set: reg_set.clone(),
                index: reg.index,
            }),
        },
        _ => {
            return Err(AssembleError::bad_op(
                AssembleInstructionError::ExpectedDstReg,
                &operand,
            ));
        }
    }
}

fn parse_reg_or_constant(
    operand: &OperandWithLoc,
    infer_as: &RegisterSet,
    labels: &HashMap<String, usize>,
) -> Result<bc::Operand, AssembleError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => Ok(bc::Operand::Reg(infer_reg(reg.clone(), infer_as))),
        ast::Operand::Constant(x) => Ok(bc::Operand::UnsignedConstant(*x)),
        // Labels are only allowed if we are inferring as the address type (destination is an address register)
        ast::Operand::Label(label) => match infer_as {
            RegisterSet::Single(RegType::Address) => {
                let pc = labels.get(label).ok_or_else(|| {
                    AssembleError::bad_op(
                        AssembleInstructionError::MissingLabel(label.to_owned()),
                        operand,
                    )
                })?;
                Ok(bc::Operand::LabelConstant(*pc))
            }
            _ => Err(AssembleError::bad_op(
                AssembleInstructionError::InvalidOperand,
                operand,
            )),
        },
    }
}

fn parse_int_reg(operand: &OperandWithLoc) -> Result<bc::Operand, AssembleError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => Ok(bc::Operand::Reg(RegOperand {
            set: reg
                .set
                .as_ref()
                .ok_or_else(|| {
                    AssembleError::bad_op(AssembleInstructionError::CannotInferReg, operand)
                })?
                .clone(),
            index: reg.index,
        })),
        ast::Operand::Constant(c) => Ok(bc::Operand::UnsignedConstant(*c)),
        ast::Operand::Label(_) => Err(AssembleError::invalid_op(operand)),
    }
}

fn parse_address_operand(
    operand: &OperandWithLoc,
    labels: &HashMap<String, usize>,
) -> Result<bc::Operand, AssembleError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => match &reg.set {
            Some(RegisterSet::Single(RegType::Address)) => Ok(bc::Operand::Reg(RegOperand {
                set: RegisterSet::Single(RegType::Address),
                index: reg.index,
            })),
            Some(_) => Err(AssembleError::invalid_op(operand)),
            None => Err(AssembleError::bad_op(
                AssembleInstructionError::CannotInferReg,
                operand,
            )), // TODO: Could infer as address register?
        },
        ast::Operand::Constant(_) => Err(AssembleError::invalid_op(operand)),
        ast::Operand::Label(label) => {
            let pc = labels.get(label).ok_or_else(|| {
                AssembleError::bad_op(
                    AssembleInstructionError::MissingLabel(label.to_owned()),
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
