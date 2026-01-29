use byteorder::{LE, ReadBytesExt};
use std::collections::HashMap;
use std::io;

use crate::binary::{BinaryFormatVersion, DecodeError, EncodeError};
use crate::instructions::{
    AddParams, AnyConsistentNumOp, BinaryCondition, CompareParams, CompareToZero, Instruction,
    MovParams, NotParams,
};
use crate::operand::{Operand, RegOperand};
use crate::parse::{parse_any_reg, parse_any_single_reg};
use crate::reg_model::{InstrRegT, Reg, RegOrConstant};
use crate::unparse::instr_to_raw;
use crate::{NumRegType, Program, RegIndex, RegType, RegWidth, RegisterSet};

pub const VERSION: BinaryFormatVersion = BinaryFormatVersion { major: 0, minor: 1 };

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
    // TODO: Flags and stuff - check reg size first?
    dst.write(&[opcode as u8])?;

    for o in operands {
        match o {
            Operand::Reg(reg_operand) => {
                let rs_index = rt_header
                    .reverse_lookup(RTHeaderEntry::Register(reg_operand.set))
                    .expect("Missing register set entry in constructed RT table");
                let rs_byte: u8 = rs_index
                    .try_into()
                    .expect("Register set index does not fit in a byte");
                dst.write(&[rs_byte])?;

                let r_index_byte: u8 = reg_operand
                    .index
                    .try_into()
                    .expect("Register index does not fit in a byte!");
                dst.write(&[r_index_byte])?;
            }
            Operand::UnsignedConstant(c) => {
                let rs_index: usize = rt_header
                    .reverse_lookup(RTHeaderEntry::Constant(RegType::Num(
                        NumRegType::UnsignedInt(u64::BITS),
                    )))
                    .expect("No matching rt entry for unsigned constant");
                let rs_index_byte: u8 = rs_index
                    .try_into()
                    .expect("Constant Register doesn't fit into byte");
                dst.write(&[rs_index_byte])?;

                dst.write(&c.to_le_bytes())?;
            }
            Operand::SignedConstant(c) => {
                let rs_index = rt_header
                    .reverse_lookup(RTHeaderEntry::Constant(RegType::Num(
                        NumRegType::UnsignedInt(64),
                    )))
                    .expect("No matching rt entry for 64-bit signed constant");
                let rs_index_byte: u8 = rs_index
                    .try_into()
                    .expect("Constant register doesn't fit into a byte");
                dst.write(&[rs_index_byte])?;
                dst.write(&c.to_le_bytes())?;
            }
            Operand::FloatConstant(c) => {
                let rs_index = rt_header
                    .reverse_lookup(RTHeaderEntry::Constant(RegType::Num(NumRegType::Float(64))))
                    .expect("No matching rt entry for 64-bit float");
                let rs_index_byte: u8 = rs_index
                    .try_into()
                    .expect("Constant Register doesn't fit into byte");
                dst.write(&[rs_index_byte])?;
                dst.write(&c.to_le_bytes())?;
            }
            Operand::LabelConstant(c) => {
                let rs_index = rt_header
                    .reverse_lookup(RTHeaderEntry::Constant(RegType::InstructionAddress))
                    .expect("No register set for instruction address in table");
                let rs_index_byte: u8 = rs_index
                    .try_into()
                    .expect("No matching register set for unsigned constant");
                dst.write(&[rs_index_byte])?;

                dst.write(&(c as u64).to_le_bytes())?;
            }
        }
    }
    Ok(())
}

pub fn decode<R: io::Read>(mut src: R) -> Result<Program, DecodeError> {
    let rt_header = RTHeader::read(&mut src)?;

    let mut instrs = vec![];
    while let Some(instr) = decode_instruction(&mut src, &rt_header)? {
        instrs.push(instr);
    }
    Ok(Program {
        instructions: instrs,
    })
}

pub fn decode_instruction<R: io::Read>(
    src: &mut R,
    rt_header: &RTHeader,
) -> Result<Option<Instruction>, DecodeError> {
    let opcode_byte = match src.read_u8() {
        Ok(b) => b,
        Err(e) => match e.kind() {
            io::ErrorKind::UnexpectedEof => return Ok(None),
            _ => return Err(DecodeError::ReadError(e)),
        },
    };

    fn operands<R: io::Read, const N: usize>(
        src: &mut R,
        rt_header: &RTHeader,
    ) -> Result<[Operand; N], DecodeError> {
        let mut ops = vec![];
        for _ in 0..N {
            let rs_byte = src.read_u8()?;

            let rs = rt_header.lookup(rs_byte as usize).ok_or_else(|| {
                DecodeError::Malformed(format!("Register set entry does not exist"))
            })?;

            let op = match rs {
                RTHeaderEntry::Register(rs) => {
                    let index_byte = src.read_u8()?;
                    Operand::Reg(RegOperand {
                        set: rs.clone(),
                        index: index_byte as RegIndex,
                    })
                }
                RTHeaderEntry::Constant(reg_type) => {
                    let constant = src.read_u64::<LE>()?;
                    match reg_type {
                        RegType::Num(NumRegType::UnsignedInt(_)) => {
                            Operand::UnsignedConstant(constant)
                        }
                        RegType::Num(NumRegType::SignedInt(_)) => {
                            Operand::SignedConstant(constant as i64)
                        }
                        RegType::Num(NumRegType::Float(_)) => {
                            Operand::FloatConstant(f64::from_bits(constant))
                        }
                        RegType::InstructionAddress => Operand::LabelConstant(constant as usize),
                        RegType::MemoryAddress => {
                            return Err(DecodeError::Malformed(format!(
                                "Cannot decode constant for a memory address"
                            )));
                        }
                    }
                }
            };
            ops.push(op);
        }
        Ok(ops.try_into().unwrap())
    }

    fn read_add_op<R: io::Read>(
        src: &mut R,
        rt_header: &RTHeader,
    ) -> Result<AddParams, DecodeError> {
        let ops = operands::<R, 3>(src, rt_header)?;
        let ops_ref: Vec<&Operand> = ops.iter().collect();
        AddParams::try_from(ops_ref.as_slice()).map_err(|i| {
            DecodeError::Malformed(format!("Invalid Instruction: {:?} for {:?}", i, ops))
        })
    }

    fn read_3_num_op<R: io::Read>(
        src: &mut R,
        rt_header: &RTHeader,
    ) -> Result<AnyConsistentNumOp, DecodeError> {
        let ops = operands::<R, 3>(src, rt_header)?;
        let ops_ref: Vec<&Operand> = ops.iter().collect();
        AnyConsistentNumOp::try_from(ops_ref.as_slice()).map_err(|i| {
            DecodeError::Malformed(format!("Invalid Instruction: {:?} for {:?}", i, ops))
        })
    }

    fn iaddr_op(operand: &Operand) -> Result<RegOrConstant<InstrRegT>, DecodeError> {
        RegOrConstant::from_instr_addr(operand).map_err(|_| {
            DecodeError::Malformed(format!("Expected Instruction Register or Constant"))
        })
    }

    fn cmp_zero_op(operand: &Operand) -> Result<CompareToZero, DecodeError> {
        CompareToZero::try_from(operand).map_err(|_| {
            DecodeError::Malformed(format!("Expected operand that can be compared to zero"))
        })
    }

    let opcode = OpCode::from_value(opcode_byte)?;
    Ok(Some(match opcode {
        OpCode::NOP => Instruction::Nop,
        OpCode::MOV => {
            let ops = operands::<R, 2>(src, rt_header)?;
            let ops_ref: Vec<&Operand> = ops.iter().collect();
            Instruction::Mov(MovParams::try_from(ops_ref.as_slice())?)
        }
        OpCode::ADD => Instruction::Add(read_add_op(src, rt_header)?),
        OpCode::SUB => Instruction::Sub(read_3_num_op(src, rt_header)?),
        OpCode::MUL => Instruction::Mul(read_3_num_op(src, rt_header)?),
        OpCode::DIV => Instruction::Div(read_3_num_op(src, rt_header)?),
        OpCode::MOD => Instruction::Mod(read_3_num_op(src, rt_header)?),
        OpCode::JMP => {
            let ops = operands::<R, 1>(src, rt_header)?;
            Instruction::Jmp(iaddr_op(&ops[0])?)
        }
        OpCode::BZ => {
            let ops = operands::<R, 2>(src, rt_header)?;
            let cmp_zero: CompareToZero = cmp_zero_op(&ops[1])?;
            Instruction::Bz(iaddr_op(&ops[0])?, cmp_zero)
        }
        OpCode::BNZ => {
            let ops = operands::<R, 2>(src, rt_header)?;
            let cmp_zero: CompareToZero = cmp_zero_op(&ops[1])?;
            Instruction::Bnz(iaddr_op(&ops[0])?, cmp_zero)
        }
        OpCode::EQ | OpCode::GT | OpCode::GTE => {
            let ops = operands::<R, 3>(src, rt_header)?;
            let ops_ref: Vec<&Operand> = ops.iter().collect();
            let params = CompareParams::try_from(ops_ref.as_slice()).map_err(|i| {
                DecodeError::Malformed(format!("Invalid Instruction: {:?} | {:?}", i, ops))
            })?;
            let cond = match opcode {
                OpCode::EQ => BinaryCondition::Equal,
                OpCode::GT => BinaryCondition::GreaterThan,
                OpCode::GTE => BinaryCondition::GreaterThanOrEqualTo,
                _ => unreachable!("All branches should be covered"),
            };
            Instruction::Compare { cond, params }
        }
        OpCode::AND => Instruction::And(read_3_num_op(src, rt_header)?),
        OpCode::OR => Instruction::Or(read_3_num_op(src, rt_header)?),
        OpCode::XOR => Instruction::Xor(read_3_num_op(src, rt_header)?),
        OpCode::NOT => {
            let [dst, op] = operands::<R, 2>(src, rt_header)?;
            let params = NotParams::try_from([&dst, &op].as_slice())?;
            Instruction::Not(params)
        }
        OpCode::ALLOC => {
            let [op1, op2] = operands(src, rt_header)?;
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
            Instruction::Alloc(mem_reg, size)
        }
        OpCode::FREE => {
            let [op1] = operands(src, rt_header)?;
            let mem_reg = Reg::from_mem_reg(&op1).map_err(|_| {
                DecodeError::Malformed(format!("Invalid Free Instruction: Expected mem reg"))
            })?;
            Instruction::Free(mem_reg)
        }
        OpCode::LOAD => {
            let [op1, op2] = operands(src, rt_header)?;
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
            let mem_reg = Reg::from_mem_reg(&op2).map_err(|_| {
                DecodeError::Malformed(format!("Expected destination register for load"))
            })?;
            Instruction::Load(value_reg, mem_reg)
        }
        OpCode::STORE => {
            let [op1, op2] = operands(src, rt_header)?;
            let mem_reg = Reg::from_mem_reg(&op1).map_err(|_| {
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
            Instruction::Store(mem_reg, value_reg)
        }
        OpCode::DBG => {
            let [op] = operands::<R, 1>(src, rt_header)?;
            match op {
                Operand::Reg(reg_operand) => Instruction::Dbg(parse_any_reg(&reg_operand)),
                _ => {
                    return Err(DecodeError::Malformed(format!(
                        "Debug Instruction requires a register operand"
                    )));
                }
            }
        }
    }))
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
        Instruction::Bz(_, _) => OpCode::BZ,
        Instruction::Bnz(_, _) => OpCode::BNZ,
        Instruction::Alloc(_, _) => OpCode::ALLOC,
        Instruction::Free(_) => OpCode::FREE,
        Instruction::Load(_, _) => OpCode::LOAD,
        Instruction::Store(_, _) => OpCode::STORE,
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
                dst.write(&x.to_le_bytes())?;
            }
            if let Some(x) = length {
                dst.write(&x.to_le_bytes())?;
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
                width = Some(src.read_u32::<LE>()?);
            }
            if has_length {
                length = Some(src.read_u32::<LE>()?);
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

#[derive(Debug)]
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
        Ok(match value {
            x if x == RegTypeId::Unsigned as u8 => RegTypeId::Unsigned,
            x if x == RegTypeId::Signed as u8 => RegTypeId::Signed,
            x if x == RegTypeId::Float as u8 => RegTypeId::Float,
            x if x == RegTypeId::MemoryAddress as u8 => RegTypeId::MemoryAddress,
            x if x == RegTypeId::InstructionAddress as u8 => RegTypeId::InstructionAddress,
            x => return Err(InvalidRegTypeId(x)),
        })
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

#[derive(Debug)]
#[repr(u8)]
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
    BZ = 0b001001,
    BNZ = 0b001010,

    // Comparison
    EQ = 0b001011,
    GT = 0b001100,
    GTE = 0b001101,

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
        Ok(match value {
            x if x == Self::NOP as u8 => Self::NOP,
            x if x == Self::MOV as u8 => Self::MOV,

            x if x == Self::ADD as u8 => Self::ADD,
            x if x == Self::SUB as u8 => Self::SUB,
            x if x == Self::MUL as u8 => Self::MUL,
            x if x == Self::DIV as u8 => Self::DIV,
            x if x == Self::MOD as u8 => Self::MOD,

            x if x == Self::JMP as u8 => Self::JMP,
            x if x == Self::BZ as u8 => Self::BZ,
            x if x == Self::BNZ as u8 => Self::BNZ,

            x if x == Self::EQ as u8 => Self::EQ,
            x if x == Self::GT as u8 => Self::GT,
            x if x == Self::GTE as u8 => Self::GTE,

            x if x == Self::AND as u8 => Self::AND,
            x if x == Self::OR as u8 => Self::OR,
            x if x == Self::XOR as u8 => Self::XOR,
            x if x == Self::NOT as u8 => Self::NOT,
            x if x == Self::ALLOC as u8 => Self::ALLOC,
            x if x == Self::FREE as u8 => Self::FREE,
            x if x == Self::LOAD as u8 => Self::LOAD,
            x if x == Self::STORE as u8 => Self::STORE,

            x if x == Self::DBG as u8 => Self::DBG,
            x => return Err(InvalidOpCode(x)),
        })
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::binary::v0::{
        OpCode, RTHeader, RTHeaderEntry, decode, decode_instruction, encode, encode_instruction,
    };
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

        let mut cursor = Cursor::new(buffer);

        let decoded_instr = decode_instruction(&mut cursor, &rt_header).expect("Decoding failed");

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

        let mut cursor = Cursor::new(buffer);
        let decoded_instr = decode_instruction(&mut cursor, &rt_header).expect("Decoding failed");

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
        let prog = Program {
            instructions: vec![instr],
        };

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

        let prog = Program {
            instructions: vec![instr],
        };

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
