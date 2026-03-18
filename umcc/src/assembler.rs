use std::collections::HashMap;
use std::ops::RangeInclusive;

use crate::ast::{self, OperandWithLoc};
use umc_model::instructions::{
    AddParams, AnyConsistentNumOp, BinaryCondition, CompareParams, CompareToZero,
    ConsistentComparison, ECallParams, Instruction, MovParams, NotParams, SimpleCast,
};
use umc_model::operand::{Operand, RegOperand};
use umc_model::parse::{InstructionValidateError, parse_any_reg, parse_any_single_reg};
use umc_model::reg_model::{InstrRegT, Reg, RegOrConstant};
use umc_model::{Program, operand as bc};
use umc_model::{RegType, RegisterSet};

type Loc = RangeInclusive<usize>;

pub enum AssembleError {
    DuplicateLabel(String, Loc),
    DuplicateMemLabel(String, Loc),
    InvalidInstruction(AssembleInstructionError, Loc),
}

#[derive(Debug)]
pub enum AssembleInstructionError {
    UnknownOpCode(Loc),
    // Expected, got
    InvalidOperandCount(usize, usize),
    InvalidOperand(InvalidOperandError, usize, Loc),
}

impl AssembleInstructionError {
    pub fn invalid_op(error: InvalidOperandError, operand: &OperandWithLoc) -> Self {
        Self::InvalidOperand(error, operand.1, operand.2.clone())
    }

    pub fn invalid_op_type(operand: &OperandWithLoc) -> Self {
        Self::invalid_op(InvalidOperandError::InvalidType, operand)
    }

    pub fn unknown_opcode(instr: &ast::Instruction) -> Self {
        let opcode_loc = *instr.loc.start()..=(instr.loc.start() + instr.opcode.len() - 1); // TODO?
        Self::UnknownOpCode(opcode_loc)
    }
}

#[derive(Debug)]
pub enum InvalidOperandError {
    ExpectedDstReg,
    ExpectedReg,
    CannotInferReg,
    UnknownLabel(String),
    UnknownMemLabel(String),
    /// The type of the operand does not agree with the destination / instruction
    InvalidType,
}

impl AssembleError {
    pub fn bad_instruction(error: AssembleInstructionError, instr: &ast::Instruction) -> Self {
        Self::InvalidInstruction(error, instr.loc.clone())
    }
}

fn add_ctx(err: InstructionValidateError, instr: &ast::Instruction) -> AssembleInstructionError {
    let operand_err =
        |err, index| AssembleInstructionError::invalid_op(err, &instr.operands[index]);
    let instr_err = match err {
        InstructionValidateError::InvalidOpCount { expected, got } => {
            AssembleInstructionError::InvalidOperandCount(expected, got)
        }
        InstructionValidateError::ExpectedDstReg => {
            operand_err(InvalidOperandError::ExpectedDstReg, 0)
        }
        InstructionValidateError::CannotInferReg { op_index } => {
            operand_err(InvalidOperandError::CannotInferReg, op_index)
        }
        InstructionValidateError::InvalidRegType { op_index } => {
            operand_err(InvalidOperandError::InvalidType, op_index)
        }
        InstructionValidateError::InconsistentOperand { op_index } => {
            operand_err(InvalidOperandError::InvalidType, op_index)
        }
        InstructionValidateError::CannotNarrowWidth { op_index } => {
            operand_err(InvalidOperandError::InvalidType, op_index)
        } // TODO: Assembler-level errors
    };
    instr_err
}

pub struct Labels {
    instrs: HashMap<String, usize>,
    mem: HashMap<String, usize>,
}

pub fn compile_prog(ast_prog: Vec<ast::Statement>) -> Result<Program, Vec<AssembleError>> {
    let mut labels = Labels {
        instrs: HashMap::new(),
        mem: HashMap::new(),
    };
    let mut instrs = Vec::new();
    let mut pre_init_mem = Vec::new();

    let mut errors = vec![];

    for (i, statement) in ast_prog.iter().filter_map(|s| s.as_instr()).enumerate() {
        if let Some(label) = &statement.0 {
            if labels.instrs.insert(label.0.clone(), i).is_some() {
                errors.push(AssembleError::DuplicateLabel(
                    label.0.to_owned(),
                    label.1.clone(),
                ));
            }
        }
    }

    for (i, (label, _)) in ast_prog
        .iter()
        .filter_map(|s| s.as_memory_data())
        .enumerate()
    {
        if labels.mem.insert(label.0.to_owned(), i).is_some() {
            errors.push(AssembleError::DuplicateMemLabel(
                label.0.to_owned(),
                label.1.clone(),
            ));
        }
    }

    for statement in ast_prog.into_iter() {
        let mut parse_instr = |i: ast::Instruction| match ast_to_bytecode(i.clone(), &labels) {
            Ok(bc) => instrs.push(bc),
            Err(e) => errors.push(AssembleError::bad_instruction(e, &i)),
        };
        match statement {
            ast::Statement::Instr(i) => parse_instr(i),
            ast::Statement::LabelledInstr(_, i) => parse_instr(i),
            ast::Statement::MemoryData(_, data) => pre_init_mem.push(data),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(Program {
        instructions: instrs,
        pre_init_mem: pre_init_mem,
        mem_labels: labels.mem,
        instr_labels: labels.instrs,
    })
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
    labels: &Labels,
) -> Result<Instruction, AssembleInstructionError> {
    fn infer_ops<'a, const N: usize>(
        instr: &ast::Instruction,
        labels: &Labels,
    ) -> Result<Vec<Operand>, AssembleInstructionError> {
        let dst = instr
            .operands
            .first()
            .ok_or(AssembleInstructionError::InvalidOperandCount(N, 0))?;
        let dst_reg = parse_dst_reg(dst)?;

        let mut ops = vec![];
        for o in instr.operands.iter() {
            let x = parse_reg_or_constant(o, Some(&dst_reg.set), labels)?;
            ops.push(x);
        }

        Ok(ops)
    }

    fn add_op(
        instr: &ast::Instruction,
        labels: &Labels,
    ) -> Result<AddParams, AssembleInstructionError> {
        let inferred = infer_ops::<3>(instr, labels)?;
        let refs: Vec<&Operand> = inferred.iter().collect();
        let params = AddParams::try_from(refs.as_slice()).map_err(|e| add_ctx(e, &instr))?;
        Ok(params)
    }

    fn coherent_num_op(
        instr: &ast::Instruction,
        labels: &Labels,
    ) -> Result<AnyConsistentNumOp, AssembleInstructionError> {
        let inferred = infer_ops::<3>(instr, labels)?;
        let refs: Vec<&Operand> = inferred.iter().collect();
        let params = AnyConsistentNumOp::try_from(&refs[..]).map_err(|e| add_ctx(e, &instr))?;
        Ok(params)
    }

    fn comparison_op(
        instr: &ast::Instruction,
        labels: &Labels,
    ) -> Result<CompareParams, AssembleInstructionError> {
        let [p0, p1, p2] = ops(instr)?;
        let dst_op = parse_dst_reg(p0)?;
        let dst = Reg::from_unsigned(&Operand::Reg(dst_op))
            .map_err(|_| AssembleInstructionError::invalid_op_type(&p0))?;
        let p1 = parse_reg_or_constant(p1, None, labels)?;
        let p2 = parse_reg_or_constant(p2, None, labels)?;
        let args = ConsistentComparison::try_from([&p1, &p2].as_slice())
            .map_err(|e| add_ctx(e, &instr))?;
        Ok(CompareParams { dst, args })
    }

    fn cond_branch(
        instr: &ast::Instruction,
        labels: &Labels,
    ) -> Result<(RegOrConstant<InstrRegT>, CompareToZero), AssembleInstructionError> {
        let [p0, p1] = ops(instr)?;
        let dst_loc = parse_reg_or_constant(
            p0,
            Some(&RegisterSet::Single(RegType::InstructionAddress)),
            &labels,
        )?;
        let jump_loc = RegOrConstant::from_instr_addr(&dst_loc)
            .map_err(|_| AssembleInstructionError::invalid_op_type(p0))?;

        let cmp_op = parse_reg_or_constant(p1, None, labels)?;
        let cmp_zero = CompareToZero::try_from(&cmp_op)
            .map_err(|_| AssembleInstructionError::invalid_op_type(p0))?;

        Ok((jump_loc, cmp_zero))
    }

    fn ecall_instr(
        instr: &ast::Instruction,
        labels: &Labels,
    ) -> Result<ECallParams, AssembleInstructionError> {
        let mut operands = vec![];
        for o in &instr.operands {
            operands.push(parse_reg_or_constant(&o, None, labels)?);
        }
        let ops_ref: Vec<&Operand> = operands.iter().collect();
        Ok(ECallParams::try_from(ops_ref.as_slice()).map_err(|e| add_ctx(e, &instr))?)
    }

    match instr.opcode.as_str() {
        "mov" => {
            let inferred = infer_ops::<2>(&instr, labels)?;
            let refs: Vec<&Operand> = inferred.iter().collect();
            let params = MovParams::try_from(&refs[..]).map_err(|e| add_ctx(e, &instr))?;
            Ok(Instruction::Mov(params))
        }
        "add" => {
            let params = add_op(&instr, labels)?;
            Ok(Instruction::Add(params))
        }
        "and" => {
            let params = coherent_num_op(&instr, labels)?;
            Ok(Instruction::And(params))
        }
        "sub" => {
            let params = coherent_num_op(&instr, labels)?;
            Ok(Instruction::Sub(params))
        }
        "mul" => {
            let params = coherent_num_op(&instr, labels)?;
            Ok(Instruction::Mul(params))
        }
        "div" => {
            let params = coherent_num_op(&instr, labels)?;
            Ok(Instruction::Div(params))
        }
        "mod" => {
            let params = coherent_num_op(&instr, labels)?;
            Ok(Instruction::Mod(params))
        }
        "xor" => {
            let params = coherent_num_op(&instr, labels)?;
            Ok(Instruction::Xor(params))
        }
        "not" => {
            let inferred = infer_ops::<2>(&instr, labels)?;
            let refs: Vec<&Operand> = inferred.iter().collect();
            let params = NotParams::try_from(&refs[..]).map_err(|e| add_ctx(e, &instr))?;
            Ok(Instruction::Not(params))
        }
        "eq" => {
            let params = comparison_op(&instr, labels)?;
            Ok(Instruction::Compare {
                cond: BinaryCondition::Equal,
                params,
            })
        }
        "gt" => {
            let params = comparison_op(&instr, labels)?;
            Ok(Instruction::Compare {
                cond: BinaryCondition::GreaterThan,
                params,
            })
        }
        "ge" => {
            let params = comparison_op(&instr, labels)?;
            Ok(Instruction::Compare {
                cond: BinaryCondition::GreaterThanOrEqualTo,
                params,
            })
        }
        "lt" => {
            let params = comparison_op(&instr, labels)?;
            Ok(Instruction::Compare {
                cond: BinaryCondition::LessThan,
                params,
            })
        }
        "le" => {
            let params = comparison_op(&instr, labels)?;
            Ok(Instruction::Compare {
                cond: BinaryCondition::LessThanOrEqualTo,
                params,
            })
        }
        "jmp" => {
            let [p1] = ops::<1>(&instr)?;
            let iaddr_op = parse_iaddress_operand(p1, labels)?;
            Ok(Instruction::Jmp(iaddr_op))
        }
        "jal" => {
            let [p1, p2] = ops::<2>(&instr)?;
            let dest = parse_iaddress_operand(p1, labels)?;
            let link_reg = parse_iaddress_reg(p2)?;
            Ok(Instruction::Jal(dest, link_reg))
        }
        "bz" => {
            let (dst, operand) = cond_branch(&instr, labels)?;
            Ok(Instruction::Bz(dst, operand))
        }
        "bnz" => {
            let (dst, operand) = cond_branch(&instr, labels)?;
            Ok(Instruction::Bnz(dst, operand))
        }
        "alloc" => {
            let [p1, p2] = ops::<2>(&instr)?;
            let mem_reg = parse_dst_reg(p1)?;
            let size_param = parse_reg_or_constant(p2, None, &labels)?;

            let mem_reg = Reg::from_mem_reg(&Operand::Reg(mem_reg))
                .map_err(|_| AssembleInstructionError::invalid_op_type(p1))?;
            let size_param = RegOrConstant::from_unsigned(&size_param)
                .map_err(|_| AssembleInstructionError::invalid_op_type(p2))?;
            Ok(Instruction::Alloc(mem_reg, size_param))
        }
        "free" => {
            let [p1] = ops::<1>(&instr)?;
            let reg = parse_dst_reg(p1)?;
            let mem_reg = Reg::from_mem_reg(&Operand::Reg(reg))
                .map_err(|_| AssembleInstructionError::invalid_op_type(p1))?;
            Ok(Instruction::Free(mem_reg))
        }
        "load" => {
            let [p1, p2] = ops::<2>(&instr)?;
            let dst_reg = parse_dst_reg(p1)?;
            let dst_reg = parse_any_single_reg(&dst_reg)
                .map_err(|_| AssembleInstructionError::invalid_op_type(p1))?;

            let mem_reg = parse_reg_or_constant(p2, None, labels)?;
            let mem_reg = RegOrConstant::from_mem_addr(&mem_reg)
                .map_err(|_| AssembleInstructionError::invalid_op_type(p2))?;
            Ok(Instruction::Load(dst_reg, mem_reg))
        }
        "store" => {
            let [p1, p2] = ops::<2>(&instr)?;
            let mem_reg = parse_dst_reg(p1)?;
            let mem_reg = RegOrConstant::from_mem_addr(&Operand::Reg(mem_reg))
                .map_err(|_| AssembleInstructionError::invalid_op_type(p1))?;

            let value_op = parse_reg(p2, None)?;
            let value_param = parse_any_single_reg(&value_op)
                .map_err(|_| AssembleInstructionError::invalid_op_type(p2))?;
            Ok(Instruction::Store(mem_reg, value_param))
        }
        "cast" => {
            let inferred = infer_ops::<2>(&instr, labels)?;
            let refs: Vec<&Operand> = inferred.iter().collect();
            let cast = SimpleCast::try_from(refs.as_slice()).map_err(|e| add_ctx(e, &instr))?;
            Ok(Instruction::Cast(cast))
        }
        "ecall" => {
            let ecall = ecall_instr(&instr, labels)?;
            Ok(Instruction::ECall(ecall))
        }
        "dbg" => {
            let [op1] = ops(&instr)?;
            // acts like a dst reg, as it cannot be inferred
            let operand = parse_dst_reg(&op1)?;
            let any_reg = parse_any_reg(&operand);
            Ok(Instruction::Dbg(any_reg))
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

fn parse_reg(
    operand: &OperandWithLoc,
    infer_as: Option<&RegisterSet>,
) -> Result<bc::RegOperand, AssembleInstructionError> {
    match &operand.0 {
        ast::Operand::Reg(ast_reg_op) => {
            let set = match (&ast_reg_op.set, infer_as) {
                (Some(set), _) => set.clone(),
                (None, Some(set)) => set.clone(),
                (None, None) => {
                    return Err(AssembleInstructionError::invalid_op(
                        InvalidOperandError::CannotInferReg,
                        operand,
                    ));
                }
            };
            Ok(bc::RegOperand {
                set: set,
                index: ast_reg_op.index,
            })
        }
        _ => Err(AssembleInstructionError::invalid_op(
            InvalidOperandError::ExpectedReg,
            operand,
        )),
    }
}

fn parse_reg_or_constant(
    operand: &OperandWithLoc,
    infer_as: Option<&RegisterSet>,
    labels: &Labels,
) -> Result<bc::Operand, AssembleInstructionError> {
    match &operand.0 {
        ast::Operand::Reg(reg) => match infer_as {
            Some(infer_set) => Ok(bc::Operand::Reg(infer_reg(reg.clone(), infer_set))),
            None => {
                let set = reg.set.as_ref().ok_or_else(|| {
                    AssembleInstructionError::invalid_op(
                        InvalidOperandError::CannotInferReg,
                        operand,
                    )
                })?;
                Ok(bc::Operand::Reg(RegOperand {
                    set: set.clone(),
                    index: reg.index,
                }))
            }
        },
        ast::Operand::UnsignedConstant(x) => Ok(bc::Operand::UnsignedConstant(*x)),
        ast::Operand::NegativeConstant(x) => Ok(bc::Operand::SignedConstant(*x)),
        ast::Operand::FloatConstant(x) => Ok(bc::Operand::FloatConstant(*x)),
        ast::Operand::Label(label) => {
            let pc = labels.instrs.get(label).ok_or_else(|| {
                AssembleInstructionError::invalid_op(
                    InvalidOperandError::UnknownLabel(label.to_owned()),
                    operand,
                )
            })?;
            Ok(bc::Operand::LabelConstant(*pc))
        }
        ast::Operand::MemLabel(label) => {
            let mem_id = labels.mem.get(label).ok_or_else(|| {
                AssembleInstructionError::invalid_op(
                    InvalidOperandError::UnknownMemLabel(label.to_owned()),
                    operand,
                )
            })?;
            Ok(bc::Operand::MemLabelConstant(*mem_id))
        }
    }
}

fn parse_iaddress_operand(
    operand: &OperandWithLoc,
    labels: &Labels,
) -> Result<RegOrConstant<InstrRegT>, AssembleInstructionError> {
    let op = parse_reg_or_constant(
        operand,
        Some(&RegisterSet::Single(RegType::InstructionAddress)),
        labels,
    )?;
    RegOrConstant::from_instr_addr(&op)
        .map_err(|_| AssembleInstructionError::invalid_op_type(operand))
}

fn parse_iaddress_reg(
    operand: &OperandWithLoc,
) -> Result<Reg<InstrRegT>, AssembleInstructionError> {
    let op = parse_reg(operand, None)?;
    Reg::from_instr_addr(&Operand::Reg(op))
        .map_err(|_| AssembleInstructionError::invalid_op_type(operand))
}

fn infer_reg(reg: ast::ASTRegisterOperand, default: &RegisterSet) -> bc::RegOperand {
    bc::RegOperand {
        set: reg.set.unwrap_or_else(|| default.clone()),
        index: reg.index,
    }
}
