use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use int_enum::IntEnum;
use std::collections::HashMap;
use std::fmt::{Display, UpperHex};
use std::io;

use crate::binary::leb128::LEBEncodable;
use crate::binary::{BinaryFormatVersion, DecodeError, EncodeError};
use crate::instructions::{
    AddParams, AnyConsistentNumOp, BinaryCondition, CompareParams, CompareToZero, ECallParams,
    Instruction, MovParams, NotParams, SimpleCast,
};
use crate::operand::{Operand, RegOperand};
use crate::parse::{parse_any_reg, parse_any_single_reg};
use crate::reg_model::{InstrRegT, Reg, RegOrConstant};
use crate::unparse::instr_to_raw;
use crate::{NumRegType, Program, RegIndex, RegType, RegWidth, RegisterSet};

pub const VERSION: BinaryFormatVersion = BinaryFormatVersion { major: 0, minor: 3 };

pub fn encode<W: io::Write>(program: &Program, mut dst: W) -> Result<(), EncodeError> {
    let instrs: Vec<(OpCode, Vec<Operand>)> = program
        .instructions
        .iter()
        .map(|i| split_instruction(&i))
        .collect();

    // Calculate the most used register sets
    let mut counts: HashMap<RTHeaderEntry, u32> = HashMap::new();
    for (_, operands) in instrs.iter() {
        for o in operands {
            let key = match o {
                Operand::Reg(reg_operand) => RTHeaderEntry::Register(reg_operand.set.clone()),
                Operand::UnsignedConstant(_) => {
                    RTHeaderEntry::Constant(RegType::Num(NumRegType::UnsignedInt(u64::BITS)))
                }
                Operand::SignedConstant(_) => {
                    RTHeaderEntry::Constant(RegType::Num(NumRegType::SignedInt(i64::BITS)))
                }
                Operand::FloatConstant(_) => {
                    RTHeaderEntry::Constant(RegType::Num(NumRegType::Float(64)))
                }
                Operand::LabelConstant(_) => RTHeaderEntry::Constant(RegType::InstructionAddress),
                Operand::MemLabelConstant(_) => RTHeaderEntry::Constant(RegType::MemoryAddress),
            };
            *counts.entry(key).or_insert(0) += 1
        }
    }
    let mut counts: Vec<(RTHeaderEntry, u32)> = counts.into_iter().collect();
    counts.sort_by_key(|x| x.1);
    let rt_header = RTHeader {
        entries: counts.into_iter().map(|x| x.0).collect(),
    };

    rt_header.write(&mut dst)?;

    for (opcode, operands) in instrs {
        encode_instruction(&mut dst, &rt_header, opcode, operands)?;
    }

    Ok(())
}

fn encode_instruction<W: io::Write>(
    dst: &mut W,
    rt_header: &RTHeader,
    opcode: OpCode,
    operands: Vec<Operand>,
) -> Result<(), EncodeError> {
    dst.write(&[opcode as u8])?;

    if opcode == OpCode::ECALL {
        // Write length for instructions with a variable operand count
        dst.write_u8(operands.len() as u8)?;
    }

    for o in operands {
        match o {
            Operand::Reg(reg_operand) => {
                let rs_index: usize = rt_header
                    .reverse_lookup(RTHeaderEntry::Register(reg_operand.set))
                    .expect("Missing register set entry in constructed RT table");
                rs_index.encode_leb128(dst)?;
                reg_operand.index.encode_leb128(dst)?;
            }
            Operand::UnsignedConstant(c) => {
                let rs_index: usize = rt_header
                    .reverse_lookup_constant(RegType::Num(NumRegType::UnsignedInt(u64::BITS)))
                    .expect("No matching rt entry for unsigned constant");
                rs_index.encode_leb128(dst)?;
                c.encode_leb128(dst)?;
            }
            Operand::SignedConstant(c) => {
                let rs_index = rt_header
                    .reverse_lookup_constant(RegType::Num(NumRegType::UnsignedInt(64)))
                    .expect("No matching rt entry for 64-bit signed constant");
                rs_index.encode_leb128(dst)?;
                c.encode_leb128(dst)?;
            }
            Operand::FloatConstant(c) => {
                let rs_index: usize = rt_header
                    .reverse_lookup(RTHeaderEntry::Constant(RegType::Num(NumRegType::Float(64))))
                    .expect("No matching rt entry for 64-bit float");
                rs_index.encode_leb128(dst)?;
                dst.write(&c.to_le_bytes())?;
            }
            Operand::LabelConstant(c) => {
                let rs_index: usize = rt_header
                    .reverse_lookup(RTHeaderEntry::Constant(RegType::InstructionAddress))
                    .expect("No register set for instruction address in table");
                rs_index.encode_leb128(dst)?;
                c.encode_leb128(dst)?;
            }
            Operand::MemLabelConstant(c) => {
                let rs_index: usize = rt_header
                    .reverse_lookup(RTHeaderEntry::Constant(RegType::MemoryAddress))
                    .expect("No register set for memory address constant in table");
                rs_index.encode_leb128(dst)?;
                c.encode_leb128(dst)?;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct V0Dissassembler {
    type_header: RTHeader,
    instructions: Vec<Option<Instruction>>,
    track_raw: bool,
    raw_instructions: Vec<RawInstruction>,
}

impl V0Dissassembler {
    pub fn new(type_header: RTHeader) -> Self {
        Self {
            type_header,
            instructions: vec![],
            track_raw: false,
            raw_instructions: vec![],
        }
    }

    pub fn new_tracking(type_header: RTHeader) -> Self {
        Self {
            type_header,
            instructions: vec![],
            track_raw: true,
            raw_instructions: vec![],
        }
    }

    fn decode_fixed_instruction<const N: usize, R: io::Read, F>(
        &mut self,
        src: &mut R,
        opcode_raw: u8,
        parse: F,
    ) -> Result<Instruction, DecodeError>
    where
        F: Fn(&[Operand; N]) -> Result<Instruction, DecodeError>,
    {
        let mut operands = vec![];
        let mut raw_operands = vec![];
        for _ in 0..N {
            let raw_operand = self.decode_operand(src)?;
            operands.push(raw_operand.0.clone());
            if self.track_raw {
                raw_operands.push(raw_operand);
            }
        }

        self.raw_instructions.push(RawInstruction {
            opcode: opcode_raw,
            operands: raw_operands,
        });
        let operand_array: &[Operand; N] = operands.as_slice().try_into().unwrap();
        match parse(operand_array) {
            Ok(instr) => {
                self.instructions.push(Some(instr.clone()));
                return Ok(instr);
            }
            Err(err) => {
                self.instructions.push(None);
                return Err(err);
            }
        }
    }

    fn decode_variable_instruction<R: io::Read, F>(
        &mut self,
        src: &mut R,
        opcode_raw: u8,
        parse: F,
    ) -> Result<Instruction, DecodeError>
    where
        F: Fn(&[&Operand]) -> Result<Instruction, DecodeError>,
    {
        let op_count = src.read_u8()?;

        let mut operands = vec![];
        let mut raw_operands = vec![];
        for _ in 0..op_count {
            let raw_operand = self.decode_operand(src)?;
            operands.push(raw_operand.0.clone());
            if self.track_raw {
                raw_operands.push(raw_operand);
            }
        }

        self.raw_instructions.push(RawInstruction {
            opcode: opcode_raw,
            operands: raw_operands,
        });
        let op_refs: Vec<&Operand> = operands.iter().collect();
        match parse(&op_refs) {
            Ok(instr) => {
                self.instructions.push(Some(instr.clone()));
                return Ok(instr);
            }
            Err(err) => {
                self.instructions.push(None);
                return Err(err);
            }
        }
    }

    fn decode_operand<R: io::Read>(
        &mut self,
        src: &mut R,
    ) -> Result<(Operand, usize, OpValue), DecodeError> {
        let type_index: usize = usize::decode_leb128(src)?;

        let rs = self
            .type_header
            .lookup(type_index)
            .ok_or_else(|| DecodeError::Malformed(format!("Register set entry does not exist")))?;

        let op_value: OpValue;

        let op = match rs {
            RTHeaderEntry::Register(rs) => {
                let index = usize::decode_leb128(src)?;
                op_value = OpValue::LEBUnsigned(index as u64);
                Operand::Reg(RegOperand {
                    set: rs.clone(),
                    index: index as RegIndex,
                })
            }
            RTHeaderEntry::Constant(reg_type) => match reg_type {
                RegType::Num(NumRegType::UnsignedInt(_)) => {
                    let v = u64::decode_leb128(src)?;
                    op_value = OpValue::LEBUnsigned(v);
                    Operand::UnsignedConstant(v)
                }
                RegType::Num(NumRegType::SignedInt(_)) => {
                    let v = i64::decode_leb128(src)?;
                    op_value = OpValue::LEBSigned(v);
                    Operand::SignedConstant(v)
                }
                RegType::Num(NumRegType::Float(_)) => {
                    let v: u64 = src.read_u64::<LE>()?;
                    op_value = OpValue::F64(v);
                    let v: f64 = f64::from_le_bytes(v.to_le_bytes());
                    Operand::FloatConstant(v)
                }
                RegType::InstructionAddress => {
                    let v = usize::decode_leb128(src)?;
                    op_value = OpValue::LEBUnsigned(v as u64);
                    Operand::LabelConstant(v)
                }
                RegType::MemoryAddress => {
                    let v = usize::decode_leb128(src)?;
                    op_value = OpValue::LEBUnsigned(v as u64);
                    Operand::MemLabelConstant(v)
                }
            },
        };

        return Ok((op, type_index, op_value));
    }

    fn decode_3_num_op<R: io::Read, F>(
        &mut self,
        src: &mut R,
        opcode_raw: u8,
        to_instr: F,
    ) -> Result<Instruction, DecodeError>
    where
        F: Fn(AnyConsistentNumOp) -> Instruction,
    {
        self.decode_fixed_instruction::<3, _, _>(src, opcode_raw, |ops| {
            let ops_ref: Vec<&Operand> = ops.iter().collect();
            Ok(to_instr(
                AnyConsistentNumOp::try_from(ops_ref.as_slice()).map_err(|i| {
                    DecodeError::Malformed(format!("Invalid Instruction: {:?} for {:?}", i, ops))
                })?,
            ))
        })
    }
}

impl Display for V0Dissassembler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.track_raw {
            for instr in &self.instructions {
                match instr {
                    Some(instr) => writeln!(f, "{}", instr)?,
                    None => writeln!(f, "! Invalid !")?,
                }
            }
            return Ok(());
        }
        for (raw, instr) in self.raw_instructions.iter().zip(&self.instructions) {
            write!(f, "{:02X} | ", raw.opcode)?;
            for op in &raw.operands {
                write!(f, "{:02X} : {:X} ({}), ", op.1, op.2, op.0)?;
            }
            writeln!(f, "")?;
            match instr {
                Some(instr) => writeln!(f, "{}", instr)?,
                None => writeln!(f, "! Invalid !")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RawInstruction {
    pub opcode: u8,
    pub operands: Vec<(Operand, usize, OpValue)>,
}

#[derive(Debug)]
pub enum OpValue {
    LEBUnsigned(u64),
    LEBSigned(i64),
    F64(u64),
}

impl UpperHex for OpValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf: Vec<u8> = Vec::with_capacity(8);
        match self {
            OpValue::LEBUnsigned(v) => {
                v.encode_leb128(&mut buf).unwrap();
            }
            OpValue::LEBSigned(v) => {
                v.encode_leb128(&mut buf).unwrap();
            }
            OpValue::F64(v) => {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
        write!(f, "{:02X}", buf.first().unwrap())?;
        for b in &buf[1..] {
            write!(f, " {:02X}", b)?;
        }
        Ok(())
    }
}

pub fn decode<R: io::Read>(mut src: R) -> Result<Program, DecodeError> {
    let rt_header = RTHeader::read(&mut src)?;

    let mut disassembler = V0Dissassembler::new_tracking(rt_header);
    let mut instrs = vec![];
    while let Some(instr) = decode_instruction(&mut src, &mut disassembler)? {
        instrs.push(instr);
    }

    println!("{}", disassembler);

    Ok(Program::from_instrs(instrs))
}

pub fn decode_instruction<R: io::Read>(
    src: &mut R,
    disassembler: &mut V0Dissassembler,
) -> Result<Option<Instruction>, DecodeError> {
    let opcode_raw = match src.read_u8() {
        Ok(b) => b,
        Err(e) => match e.kind() {
            io::ErrorKind::UnexpectedEof => return Ok(None),
            _ => return Err(DecodeError::ReadError(e)),
        },
    };

    fn iaddr_op(operand: &Operand) -> Result<RegOrConstant<InstrRegT>, DecodeError> {
        RegOrConstant::from_instr_addr(operand).map_err(|_| {
            DecodeError::Malformed(format!("Expected Instruction Register or Constant"))
        })
    }

    fn iaddr_reg(operand: &Operand) -> Result<Reg<InstrRegT>, DecodeError> {
        Reg::from_instr_addr(operand)
            .map_err(|_| DecodeError::Malformed(format!("Expected Instruction Register")))
    }

    fn cmp_zero_op(operand: &Operand) -> Result<CompareToZero, DecodeError> {
        CompareToZero::try_from(operand).map_err(|_| {
            DecodeError::Malformed(format!("Expected operand that can be compared to zero"))
        })
    }

    let opcode = OpCode::from_value(opcode_raw)?;
    let instr = match opcode {
        OpCode::NOP => {
            disassembler.decode_fixed_instruction(src, opcode_raw, |[]| Ok(Instruction::Nop))?
        }
        OpCode::MOV => {
            disassembler.decode_fixed_instruction::<2, _, _>(src, opcode_raw, |[d, p]| {
                Ok(Instruction::Mov(MovParams::try_from([d, p].as_slice())?))
            })?
        }
        OpCode::ADD => {
            disassembler.decode_fixed_instruction::<3, _, _>(src, opcode_raw, |ops| {
                let ops_ref: Vec<&Operand> = ops.iter().collect();
                Ok(Instruction::Add(AddParams::try_from(ops_ref.as_slice())?))
            })?
        }
        OpCode::SUB => disassembler.decode_3_num_op(src, opcode_raw, |p| Instruction::Sub(p))?,
        OpCode::MUL => disassembler.decode_3_num_op(src, opcode_raw, |p| Instruction::Mul(p))?,
        OpCode::DIV => disassembler.decode_3_num_op(src, opcode_raw, |p| Instruction::Div(p))?,
        OpCode::MOD => disassembler.decode_3_num_op(src, opcode_raw, |p| Instruction::Mod(p))?,
        OpCode::JMP => disassembler.decode_fixed_instruction(src, opcode_raw, |[op]| {
            Ok(Instruction::Jmp(iaddr_op(op)?))
        })?,
        OpCode::JAL => disassembler.decode_fixed_instruction(src, opcode_raw, |[d, r]| {
            let target = iaddr_op(&d)?;
            let reg = iaddr_reg(&r)?;
            Ok(Instruction::Jal(target, reg))
        })?,
        OpCode::BZ => disassembler.decode_fixed_instruction(src, opcode_raw, |[dest, cond]| {
            let cmp_zero: CompareToZero = cmp_zero_op(&cond)?;
            Ok(Instruction::Bz(iaddr_op(&dest)?, cmp_zero))
        })?,
        OpCode::BNZ => disassembler.decode_fixed_instruction(src, opcode_raw, |[dest, cond]| {
            let cmp_zero: CompareToZero = cmp_zero_op(&cond)?;
            Ok(Instruction::Bnz(iaddr_op(&dest)?, cmp_zero))
        })?,
        OpCode::EQ | OpCode::GT | OpCode::GTE => {
            disassembler.decode_fixed_instruction(src, opcode_raw, |[d, a, b]| {
                let ops_ref: [&Operand; 3] = [&d, &a, &b];
                let params = CompareParams::try_from(ops_ref.as_slice()).map_err(|i| {
                    DecodeError::Malformed(format!("Invalid Instruction: {:?} | {:?}", i, ops_ref))
                })?;
                let cond = match opcode {
                    OpCode::EQ => BinaryCondition::Equal,
                    OpCode::GT => BinaryCondition::GreaterThan,
                    OpCode::GTE => BinaryCondition::GreaterThanOrEqualTo,
                    _ => unreachable!("All branches should be covered"),
                };
                Ok(Instruction::Compare { cond, params })
            })?
        }
        OpCode::AND => disassembler.decode_3_num_op(src, opcode_raw, |p| Instruction::And(p))?,
        OpCode::OR => disassembler.decode_3_num_op(src, opcode_raw, |p| Instruction::Or(p))?,
        OpCode::XOR => disassembler.decode_3_num_op(src, opcode_raw, |p| Instruction::Xor(p))?,
        OpCode::NOT => disassembler.decode_fixed_instruction(src, opcode_raw, |[dst, op]| {
            let params = NotParams::try_from([dst, op].as_slice())?;
            Ok(Instruction::Not(params))
        })?,
        OpCode::ALLOC => disassembler.decode_fixed_instruction(src, opcode_raw, |[op1, op2]| {
            let mem_reg = Reg::from_mem_reg(&op1).map_err(|_| {
                DecodeError::Malformed(format!(
                    "Invalid Alloc Instruction: Expected mem dst register"
                ))
            })?;
            let size = RegOrConstant::from_unsigned(&op2).map_err(|_| {
                DecodeError::Malformed(format!(
                    "Invalid Alloc Instruction: Expected unsigned size register"
                ))
            })?;
            Ok(Instruction::Alloc(mem_reg, size))
        })?,
        OpCode::FREE => disassembler.decode_fixed_instruction(src, opcode_raw, |[op1]| {
            let mem_reg = Reg::from_mem_reg(&op1).map_err(|_| {
                DecodeError::Malformed(format!("Invalid Free Instruction: Expected mem reg"))
            })?;
            Ok(Instruction::Free(mem_reg))
        })?,
        OpCode::LOAD => disassembler.decode_fixed_instruction(src, opcode_raw, |[op1, op2]| {
            let value_reg = match op1 {
                Operand::Reg(reg_operand) => parse_any_single_reg(&reg_operand).map_err(|_| {
                    DecodeError::Malformed(format!("Invalid Load Instruction: Expected Mem reg"))
                })?,
                _ => {
                    return Err(DecodeError::Malformed(format!(
                        "Expected destination register for load"
                    )));
                }
            };
            let mem_reg = RegOrConstant::from_mem_addr(&op2).map_err(|_| {
                DecodeError::Malformed(format!("Expected destination register for load"))
            })?;
            Ok(Instruction::Load(value_reg, mem_reg))
        })?,
        OpCode::STORE => disassembler.decode_fixed_instruction(src, opcode_raw, |[op1, op2]| {
            let mem_reg = RegOrConstant::from_mem_addr(&op1).map_err(|_| {
                DecodeError::Malformed(format!("Expected destination register for load"))
            })?;
            let value_reg = match op2 {
                Operand::Reg(reg_operand) => parse_any_single_reg(&reg_operand).map_err(|_| {
                    DecodeError::Malformed(format!("Invalid Load Instruction: Expected Mem reg"))
                })?,
                _ => {
                    return Err(DecodeError::Malformed(format!(
                        "Expected destination register for load"
                    )));
                }
            };
            Ok(Instruction::Store(mem_reg, value_reg))
        })?,
        OpCode::CAST => disassembler.decode_fixed_instruction(src, opcode_raw, |[d, p]| {
            let cast = SimpleCast::try_from([d, p].as_slice())
                .map_err(|e| DecodeError::Malformed(format!("Invalid cast: {:?}", e)))?;
            Ok(Instruction::Cast(cast))
        })?,
        OpCode::ECALL => disassembler.decode_variable_instruction(src, opcode_raw, |ops| {
            Ok(Instruction::ECall(ECallParams::try_from(ops)?))
        })?,
        OpCode::DBG => disassembler.decode_fixed_instruction(src, opcode_raw, |[op]| match op {
            Operand::Reg(reg_operand) => Ok(Instruction::Dbg(parse_any_reg(&reg_operand))),
            _ => {
                return Err(DecodeError::Malformed(format!(
                    "Debug Instruction requires a register operand"
                )));
            }
        })?,
    };
    Ok(Some(instr))
}

fn split_instruction(instr: &Instruction) -> (OpCode, Vec<Operand>) {
    let mut ops = instr_to_raw(instr);
    let opcode = match instr {
        Instruction::Nop => OpCode::NOP,
        Instruction::Mov(_) => OpCode::MOV,
        Instruction::Add(_) => OpCode::ADD,
        Instruction::Sub(_) => OpCode::SUB,
        Instruction::Mul(_) => OpCode::MUL,
        Instruction::Div(_) => OpCode::DIV,
        Instruction::Mod(_) => OpCode::MOD,
        Instruction::And(_) => OpCode::AND,
        Instruction::Or(_) => OpCode::OR,
        Instruction::Xor(_) => OpCode::XOR,
        Instruction::Not(_) => OpCode::NOT,
        Instruction::Compare { cond, .. } => match cond {
            BinaryCondition::Equal => OpCode::EQ,
            BinaryCondition::GreaterThan => OpCode::GT,
            BinaryCondition::GreaterThanOrEqualTo => OpCode::GTE,
            BinaryCondition::LessThan => {
                ops.swap(1, 2);
                OpCode::GTE
            }
            BinaryCondition::LessThanOrEqualTo => {
                ops.swap(1, 2);
                OpCode::GT
            }
        },
        Instruction::Jmp(_) => OpCode::JMP,
        Instruction::Jal(_, _) => OpCode::JAL,
        Instruction::Bz(_, _) => OpCode::BZ,
        Instruction::Bnz(_, _) => OpCode::BNZ,
        Instruction::Alloc(_, _) => OpCode::ALLOC,
        Instruction::Free(_) => OpCode::FREE,
        Instruction::Load(_, _) => OpCode::LOAD,
        Instruction::Store(_, _) => OpCode::STORE,
        Instruction::Cast(_) => OpCode::CAST,
        Instruction::ECall(_) => OpCode::ECALL,
        Instruction::Dbg(_) => OpCode::DBG,
    };

    (opcode, ops)
}

/// Register Types header
#[derive(Debug, PartialEq)]
pub struct RTHeader {
    pub entries: Vec<RTHeaderEntry>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RTHeaderEntry {
    Register(RegisterSet),
    Constant(RegType),
}

impl RTHeader {
    pub fn lookup(&self, index: usize) -> Option<&RTHeaderEntry> {
        self.entries.get(index)
    }

    pub fn reverse_lookup(&self, entry: RTHeaderEntry) -> Option<usize> {
        self.entries.iter().position(|x| *x == entry)
    }

    pub fn reverse_lookup_constant(&self, reg_type: RegType) -> Option<usize> {
        self.reverse_lookup(RTHeaderEntry::Constant(reg_type))
    }

    const VEC_FLAG: u8 = 1 << 7;
    const CONSTANT_FLAG: u8 = 1 << 6;
    const TYPE_ID_BITS: u8 = 0b111;

    pub fn write<W: io::Write>(&self, dst: &mut W) -> Result<(), EncodeError> {
        // Number of entries:
        dst.write(&[self.entries.len() as u8])?;

        // Control Byte: Vec/Constant/Size/Type
        for entry in self.entries.iter() {
            let mut constant = false;
            let ((id, width), length) = match entry {
                RTHeaderEntry::Register(RegisterSet::Single(reg_type)) => {
                    (RegTypeId::from_type(&reg_type), None)
                }
                RTHeaderEntry::Register(RegisterSet::Vector(reg_type, length)) => {
                    (RegTypeId::from_type(&reg_type), Some(length))
                }
                RTHeaderEntry::Constant(reg_type) => {
                    constant = true;
                    (RegTypeId::from_type(&reg_type), None)
                }
            };

            let mut control_byte = id as u8;
            if length.is_some() {
                control_byte |= Self::VEC_FLAG;
            }
            if constant {
                control_byte |= Self::CONSTANT_FLAG;
            }

            dst.write(&[control_byte])?;

            if let Some(x) = width {
                x.encode_leb128(dst)?;
            }
            if let Some(x) = length {
                x.encode_leb128(dst)?;
            }
        }
        Ok(())
    }

    pub fn read<R: io::Read>(src: &mut R) -> Result<RTHeader, DecodeError> {
        let mut entries = vec![];
        let entry_count = src.read_u8()?;

        for _ in 0..entry_count {
            let control_byte = src.read_u8()?;
            let reg_type_id = RegTypeId::from_value(control_byte & Self::TYPE_ID_BITS)?;
            let has_length = control_byte & Self::VEC_FLAG != 0;
            let is_constant = control_byte & Self::CONSTANT_FLAG != 0;

            let mut width = None;
            let mut length = None;

            if reg_type_id.is_variable_width() {
                width = Some(u32::decode_leb128(src)?);
            }
            if has_length {
                length = Some(u32::decode_leb128(src)?);
            }

            if is_constant {
                let reg_type = reg_type_id.to_reg_type(width);
                entries.push(RTHeaderEntry::Constant(reg_type));
            } else {
                let reg_set = reg_type_id.to_reg_set(width, length);
                entries.push(RTHeaderEntry::Register(reg_set));
            }
        }
        Ok(Self { entries })
    }
}

#[derive(IntEnum, Debug)]
#[repr(u8)]
enum RegTypeId {
    Unsigned = 0b000,
    Signed = 0b001,
    Float = 0b010,
    MemoryAddress = 0b011,
    InstructionAddress = 0b100,
}

pub struct InvalidRegTypeId(u8);
impl From<InvalidRegTypeId> for DecodeError {
    fn from(value: InvalidRegTypeId) -> Self {
        Self::Malformed(format!("Invalid Reg Type Id: {:b}", value.0))
    }
}

impl RegTypeId {
    pub fn is_variable_width(&self) -> bool {
        match self {
            RegTypeId::Unsigned => true,
            RegTypeId::Signed => true,
            RegTypeId::Float => true,
            RegTypeId::MemoryAddress => false,
            RegTypeId::InstructionAddress => false,
        }
    }

    pub fn from_value(value: u8) -> Result<RegTypeId, InvalidRegTypeId> {
        value.try_into().map_err(|x| InvalidRegTypeId(x))
    }

    pub fn from_type(value: &RegType) -> (RegTypeId, Option<RegWidth>) {
        match value {
            RegType::Num(NumRegType::UnsignedInt(width)) => (RegTypeId::Unsigned, Some(*width)),
            RegType::Num(NumRegType::SignedInt(width)) => (RegTypeId::Signed, Some(*width)),
            RegType::Num(NumRegType::Float(width)) => (RegTypeId::Float, Some(*width)),
            RegType::InstructionAddress => (RegTypeId::InstructionAddress, None),
            RegType::MemoryAddress => (RegTypeId::MemoryAddress, None),
        }
    }

    pub fn to_reg_type(&self, width: Option<RegWidth>) -> RegType {
        if self.is_variable_width() && width.is_none() {
            panic!("Width required for this variant ({:?})", &self);
        }
        match self {
            RegTypeId::Unsigned => RegType::Num(NumRegType::UnsignedInt(width.unwrap())),
            RegTypeId::Signed => RegType::Num(NumRegType::SignedInt(width.unwrap())),
            RegTypeId::Float => RegType::Num(NumRegType::Float(width.unwrap())),
            RegTypeId::MemoryAddress => RegType::MemoryAddress,
            RegTypeId::InstructionAddress => RegType::InstructionAddress,
        }
    }

    pub fn to_reg_set(&self, width: Option<RegWidth>, length: Option<RegWidth>) -> RegisterSet {
        let reg_type = self.to_reg_type(width);
        if let Some(length) = length {
            RegisterSet::Vector(reg_type, length)
        } else {
            RegisterSet::Single(reg_type)
        }
    }
}

#[repr(u8)]
#[derive(IntEnum, Debug, PartialEq, Clone, Copy)]
enum OpCode {
    NOP = 0b000000,
    MOV = 0b000001,

    // Arithmetic Instructions
    ADD = 0b000010,
    SUB = 0b000011,
    MUL = 0b000100,
    DIV = 0b000101,
    MOD = 0b000110,

    // Jump and conditional jump
    JMP = 0b001000,
    JAL = 0b001001,
    BZ = 0b001010,
    BNZ = 0b001011,

    // Comparison
    EQ = 0b001100,
    GT = 0b001101,
    GTE = 0b001110,

    // Bitwise Operations
    AND = 0b010000,
    OR = 0b010001,
    XOR = 0b010010,
    NOT = 0b010011,

    // Dynamic Memory Management
    ALLOC = 0b100000,
    FREE = 0b100001,
    LOAD = 0b100010,
    STORE = 0b100011,

    // Cast
    CAST = 0b110001,

    // Environment call
    ECALL = 0b110100,

    // Debug
    DBG = 0b111111,
}

pub struct InvalidOpCode(u8);
impl From<InvalidOpCode> for DecodeError {
    fn from(value: InvalidOpCode) -> Self {
        Self::Malformed(format!("Invalid opcode: {:b}", value.0))
    }
}

impl OpCode {
    pub fn from_value(value: u8) -> Result<Self, InvalidOpCode> {
        value.try_into().map_err(|x| InvalidOpCode(x))
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;
    use crate::instructions::{
        AnyReg, AnySingleReg, BinaryCondition, CompareParams, ConsistentComparison, Instruction,
        MovParams,
    };
    use crate::operand::{Operand, RegOperand};
    use crate::reg_model::{Reg, RegOrConstant};
    use crate::{NumRegType, Program, RegType, RegisterSet};

    #[test]
    fn encode_debug_instruction() {
        let reg_set_u16 = RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(32)));
        let instr = Instruction::Dbg(AnyReg::Single(AnySingleReg::Unsigned(Reg {
            index: 0,
            width: 32,
        })));

        let rt_header = RTHeader {
            entries: vec![RTHeaderEntry::Register(reg_set_u16.clone())],
        };

        let mut buffer = vec![];
        encode_instruction(
            &mut buffer,
            &rt_header,
            OpCode::DBG,
            vec![Operand::Reg(RegOperand {
                set: reg_set_u16,
                index: 0,
            })],
        )
        .expect("Encoding failed");

        let mut disassembler = V0Dissassembler::new_tracking(rt_header);
        let mut cursor = Cursor::new(buffer);

        let decoded_instr =
            decode_instruction(&mut cursor, &mut disassembler).expect("Decoding failed");

        assert_eq!(Some(instr), decoded_instr);
    }

    #[test]
    fn encode_mov_constant() {
        let reg_set_u32 = RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(32)));

        let instr = Instruction::Mov(MovParams::UnsignedInt(
            Reg {
                index: 2,
                width: 32,
            },
            RegOrConstant::Const(39),
        ));

        let rt_header = RTHeader {
            entries: vec![
                RTHeaderEntry::Constant(RegType::Num(NumRegType::UnsignedInt(64))),
                RTHeaderEntry::Register(reg_set_u32.clone()),
            ],
        };

        let mut buffer = vec![];
        encode_instruction(
            &mut buffer,
            &rt_header,
            OpCode::MOV,
            vec![
                Operand::Reg(RegOperand {
                    set: reg_set_u32,
                    index: 2,
                }),
                Operand::UnsignedConstant(39),
            ],
        )
        .expect("Encoding failed");

        let mut disassembler = V0Dissassembler::new_tracking(rt_header);
        let mut cursor = Cursor::new(buffer);
        let decoded_instr =
            decode_instruction(&mut cursor, &mut disassembler).expect("Decoding failed");

        assert_eq!(Some(instr), decoded_instr);
    }

    #[test]
    fn encode_rt_header() {
        let rt_header = RTHeader {
            entries: vec![
                RTHeaderEntry::Register(RegisterSet::Single(RegType::Num(
                    NumRegType::UnsignedInt(32),
                ))),
                RTHeaderEntry::Constant(RegType::Num(NumRegType::UnsignedInt(64))),
            ],
        };

        let mut buffer = vec![];
        rt_header
            .write(&mut buffer)
            .expect("Failed to encode RT Header");

        let mut cursor = Cursor::new(buffer);
        let decoded_rt_header = RTHeader::read(&mut cursor).expect("Failed to decode RT Header");

        assert_eq!(rt_header, decoded_rt_header);
    }

    #[test]
    fn encode_float_constant() {
        let instr = Instruction::Mov(MovParams::Float(
            Reg {
                index: 1,
                width: 64,
            },
            RegOrConstant::Const(12.5),
        ));
        let prog = Program::from_instrs(vec![instr]);

        let mut buf = vec![];
        encode(&prog, &mut buf).expect("Failed to encode program");

        let mut cursor = Cursor::new(buf);
        let decoded_prog = decode(&mut cursor).expect("Failed to decode program");
        assert_eq!(prog.instructions, decoded_prog.instructions);
    }

    #[test]
    fn encode_lt_as_gte() {
        let instr = Instruction::Compare {
            cond: BinaryCondition::LessThan,
            params: CompareParams {
                dst: Reg { index: 0, width: 1 },
                args: ConsistentComparison::UnsignedCompare(
                    RegOrConstant::from_reg(Reg {
                        index: 0,
                        width: 32,
                    }),
                    RegOrConstant::Const(100),
                ),
            },
        };

        let prog = Program::from_instrs(vec![instr]);

        let mut buf = vec![];
        encode(&prog, &mut buf).expect("Failed to encode program");

        let expected_instr = Instruction::Compare {
            cond: BinaryCondition::GreaterThanOrEqualTo,
            params: CompareParams {
                dst: Reg { index: 0, width: 1 },
                args: ConsistentComparison::UnsignedCompare(
                    RegOrConstant::Const(100),
                    RegOrConstant::from_reg(Reg {
                        index: 0,
                        width: 32,
                    }),
                ),
            },
        };
        let mut cursor = Cursor::new(buf);
        let decoded_prog = decode(&mut cursor).expect("Failed to decode program");
        assert_eq!(expected_instr, decoded_prog.instructions[0]);
    }
}
