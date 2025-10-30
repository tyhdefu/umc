use crate::ast;
use crate::bytecode::{self, Instruction, RegOperand};
use crate::model::RegisterSet;

#[derive(Debug)]
pub enum AssembleError {
    InvalidOperandCount(usize, usize),
    ExpectedDstReg,
    CannotInferReg,
    InvalidOperand,
    UnknownOpCode,
}

pub fn ast_to_bytecode(instr: ast::Instruction) -> Result<bytecode::Instruction, AssembleError> {
    match instr.opcode.as_str() {
        "mov" => {
            if instr.operands.len() != 2 {
                return Err(AssembleError::InvalidOperandCount(2, instr.operands.len()));
            }
            let dst = parse_dst_reg(&instr.operands[0])?;
            let operand = parse_reg_or_constant(&instr.operands[1], &dst.set)?;

            // TODO: that the type of these can be converted to ints

            Ok(Instruction::Mov(dst, operand))
        }
        "add" => {
            if instr.operands.len() != 3 {
                return Err(AssembleError::InvalidOperandCount(3, instr.operands.len()));
            }
            let dst = parse_dst_reg(&instr.operands[0])?;
            let operand1 = parse_reg_or_constant(&instr.operands[1], &dst.set)?;
            let operand2 = parse_reg_or_constant(&instr.operands[2], &dst.set)?;

            Ok(Instruction::Add(dst, operand1, operand2))
        }
        "dbg" => {
            // acts like a dst reg, as it cannot be inferred
            let operand = parse_dst_reg(&instr.operands[0])?;
            Ok(Instruction::Dbg(operand))
        }
        _ => Err(AssembleError::UnknownOpCode),
    }
}

fn parse_dst_reg(operand: &ast::Operand) -> Result<bytecode::RegOperand, AssembleError> {
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
) -> Result<bytecode::Operand, AssembleError> {
    match operand {
        ast::Operand::Reg(reg) => Ok(bytecode::Operand::Reg(infer_reg(reg.clone(), infer_as))),
        ast::Operand::Constant(x) => Ok(bytecode::Operand::UnsignedConstant(*x)),
        ast::Operand::Label(_) => Err(AssembleError::InvalidOperand),
    }
}

fn infer_reg(reg: ast::ASTRegisterOperand, default: &RegisterSet) -> bytecode::RegOperand {
    bytecode::RegOperand {
        set: reg.set.unwrap_or_else(|| default.clone()),
        index: reg.index,
    }
}
