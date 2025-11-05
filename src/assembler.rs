use std::collections::HashMap;

use crate::ast;
use crate::bytecode as bc;
use crate::bytecode::{Instruction, RegOperand};
use crate::model::{RegType, RegisterSet};

#[derive(Debug)]
pub enum AssembleError {
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
    InvalidInstruction(AssembleError), // TODO: add line information
}

pub fn compile_prog(
    ast_prog: Vec<ast::Statement>,
) -> Result<Vec<bc::Instruction>, AssembleProgError> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut prog = Vec::new();

    for (i, statement) in ast_prog.iter().enumerate() {
        if let Some(label) = &statement.label {
            if labels.insert(label.clone(), i).is_some() {
                return Err(AssembleProgError::DuplicateLabel(label.to_string()));
            }
        }
    }

    for statement in ast_prog.into_iter() {
        let bc = ast_to_bytecode(statement.instr, &labels)
            .map_err(|e| AssembleProgError::InvalidInstruction(e))?;
        prog.push(bc);
    }
    Ok(prog)
}

pub fn ast_to_bytecode(
    instr: ast::Instruction,
    labels: &HashMap<String, usize>,
) -> Result<bc::Instruction, AssembleError> {
    match instr.opcode.as_str() {
        "mov" => {
            if instr.operands.len() != 2 {
                return Err(AssembleError::InvalidOperandCount(2, instr.operands.len()));
            }
            let dst = parse_dst_reg(&instr.operands[0])?;
            let operand = parse_reg_or_constant(&instr.operands[1], &dst.set, labels)?;

            // TODO: that the type of these can be converted to ints

            Ok(Instruction::Mov(dst, operand))
        }
        "add" => {
            if instr.operands.len() != 3 {
                return Err(AssembleError::InvalidOperandCount(3, instr.operands.len()));
            }
            let dst = parse_dst_reg(&instr.operands[0])?;
            let operand1 = parse_reg_or_constant(&instr.operands[1], &dst.set, labels)?;
            let operand2 = parse_reg_or_constant(&instr.operands[2], &dst.set, labels)?;

            Ok(Instruction::Add(dst, operand1, operand2))
        }
        "not" => {
            if instr.operands.len() != 2 {
                return Err(AssembleError::InvalidOperandCount(2, instr.operands.len()));
            }
            let dst = parse_dst_reg(&instr.operands[0])?;
            let operand1 = parse_reg_or_constant(&instr.operands[1], &dst.set, labels)?;

            Ok(Instruction::Not(dst, operand1))
        }
        "jmp" => {
            if instr.operands.len() != 1 {
                return Err(AssembleError::InvalidOperandCount(1, instr.operands.len()));
            }
            let dst = parse_address_operand(&instr.operands[0], labels)?;
            Ok(Instruction::Jmp(dst))
        }
        "dbg" => {
            // acts like a dst reg, as it cannot be inferred
            let operand = parse_dst_reg(&instr.operands[0])?;
            Ok(Instruction::Dbg(operand))
        }
        _ => Err(AssembleError::UnknownOpCode),
    }
}

fn parse_dst_reg(operand: &ast::Operand) -> Result<bc::RegOperand, AssembleError> {
    match operand {
        ast::Operand::Reg(reg) => match &reg.set {
            None => return Err(AssembleError::CannotInferReg),
            Some(reg_set) => Ok(RegOperand {
                set: reg_set.clone(),
                index: reg.index,
            }),
        },
        _ => return Err(AssembleError::ExpectedDstReg),
    }
}

fn parse_reg_or_constant(
    operand: &ast::Operand,
    infer_as: &RegisterSet,
    labels: &HashMap<String, usize>,
) -> Result<bc::Operand, AssembleError> {
    match operand {
        ast::Operand::Reg(reg) => Ok(bc::Operand::Reg(infer_reg(reg.clone(), infer_as))),
        ast::Operand::Constant(x) => Ok(bc::Operand::UnsignedConstant(*x)),
        // Labels are only allowed if we are inferring as the address type (destination is an address register)
        ast::Operand::Label(label) => match infer_as {
            RegisterSet::Single(RegType::Address) => {
                let pc = labels
                    .get(label)
                    .ok_or_else(|| AssembleError::MissingLabel(label.to_owned()))?;
                Ok(bc::Operand::LabelConstant(*pc))
            }
            _ => Err(AssembleError::InvalidOperand),
        },
    }
}

fn parse_address_operand(
    operand: &ast::Operand,
    labels: &HashMap<String, usize>,
) -> Result<bc::Operand, AssembleError> {
    match operand {
        ast::Operand::Reg(reg) => match &reg.set {
            Some(RegisterSet::Single(RegType::Address)) => Ok(bc::Operand::Reg(RegOperand {
                set: RegisterSet::Single(RegType::Address),
                index: reg.index,
            })),
            Some(_) => Err(AssembleError::InvalidOperand),
            None => Err(AssembleError::CannotInferReg), // TODO: Could infer as address register?
        },
        ast::Operand::Constant(_) => Err(AssembleError::InvalidOperand),
        ast::Operand::Label(label) => {
            let pc = labels
                .get(label)
                .ok_or_else(|| AssembleError::MissingLabel(label.to_owned()))?;
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
