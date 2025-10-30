use crate::ast::{ASTRegisterOperand, Instruction, Operand};
use crate::grammar::{InstructionParser, OperandParser, RegOperandParser};
use crate::model::{RegType, RegisterSet};

#[test]
fn parse_hex_constant() {
    let parser = OperandParser::new();
    let exp = Operand::Constant(0x1A);
    assert_eq!(exp, parser.parse("0x1A").unwrap())
}

#[test]
fn parse_binary_constant() {
    let parser = OperandParser::new();
    let exp = Operand::Constant(0b101);
    assert_eq!(exp, parser.parse("0b101").unwrap())
}

#[test]
fn parse_base_10_constant() {
    let parser = OperandParser::new();
    let exp = Operand::Constant(303);
    assert_eq!(exp, parser.parse("#303").unwrap());
}

#[test]
fn parse_label_operand() {
    let parser = OperandParser::new();
    let exp = Operand::Label("LABEL".to_string());
    assert_eq!(exp, parser.parse(".LABEL").unwrap());
}

#[test]
fn parse_reg_operand() {
    let parser = OperandParser::new();
    let exp = ASTRegisterOperand {
        set: Some(RegisterSet::Single(RegType::UnsignedInt, 32)),
        index: 0,
    };
    assert_eq!(Operand::Reg(exp), parser.parse("u32:0").unwrap())
}

#[test]
fn parse_vector_reg_operand() {
    let parser = OperandParser::new();
    let exp = ASTRegisterOperand {
        set: Some(RegisterSet::Vector(RegType::SignedInt, 64, 8)),
        index: 4,
    };
    assert_eq!(Operand::Reg(exp), parser.parse("vi64x8:4").unwrap())
}

#[test]
fn parse_implicit_reg_operand() {
    let parser = RegOperandParser::new();
    let exp = ASTRegisterOperand {
        set: None,
        index: 0,
    };
    assert_eq!(exp, parser.parse(":0").unwrap())
}

#[test]
fn parse_mov_constant_instruction() {
    let parser = InstructionParser::new();
    let exp = Instruction {
        opcode: "mov".to_string(),
        operands: vec![
            Operand::Reg(ASTRegisterOperand {
                set: Some(RegisterSet::Single(RegType::UnsignedInt, 32)),
                index: 0,
            }),
            Operand::Constant(100),
        ],
    };
    assert_eq!(exp, parser.parse("mov u32:0, #100").unwrap());
}

#[test]
fn parse_add_instruction() {
    let parser = InstructionParser::new();

    let exp = Instruction {
        opcode: "add".to_string(),
        operands: vec![
            Operand::Reg(ASTRegisterOperand {
                set: Some(RegisterSet::Single(RegType::UnsignedInt, 32)),
                index: 1,
            }),
            Operand::Reg(ASTRegisterOperand {
                set: Some(RegisterSet::Single(RegType::UnsignedInt, 32)),
                index: 0,
            }),
            Operand::Constant(100),
        ],
    };
    assert_eq!(exp, parser.parse("add u32:1, u32:0, #100").unwrap())
}
