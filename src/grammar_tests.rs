use crate::ast::{ASTRegisterOperand, Instruction, Operand};
use crate::grammar::{
    InstructionParser, OperandParser, ProgramParser, RegOperandParser, StatementParser,
};
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
        set: Some(RegisterSet::Single(RegType::UnsignedInt(32))),
        index: 0,
    };
    assert_eq!(Operand::Reg(exp), parser.parse("u32:0").unwrap())
}

#[test]
fn parse_vector_reg_operand() {
    let parser = OperandParser::new();
    let exp = ASTRegisterOperand {
        set: Some(RegisterSet::Vector(RegType::SignedInt(64), 8)),
        index: 4,
    };
    assert_eq!(Operand::Reg(exp), parser.parse("i64x8:4").unwrap())
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
    const INSTR: &str = "mov u32:0, #100";

    let parser = InstructionParser::new();
    let exp = Instruction {
        opcode: "mov".to_string(),
        operands: vec![
            (
                Operand::Reg(ASTRegisterOperand {
                    set: Some(RegisterSet::Single(RegType::UnsignedInt(32))),
                    index: 0,
                }),
                0,
                4..=8,
            ),
            (Operand::Constant(100), 1, 11..=INSTR.len() - 1),
        ],
        loc: 0..=INSTR.len() - 1,
    };
    assert_eq!(exp, parser.parse(INSTR).unwrap());
}

#[test]
fn parse_add_instruction() {
    const INSTR: &str = "add u32:1, u32:0, #100";
    let parser = InstructionParser::new();

    let exp = Instruction {
        opcode: "add".to_string(),
        operands: vec![
            (
                Operand::Reg(ASTRegisterOperand {
                    set: Some(RegisterSet::Single(RegType::UnsignedInt(32))),
                    index: 1,
                }),
                0,
                4..=8,
            ),
            (
                Operand::Reg(ASTRegisterOperand {
                    set: Some(RegisterSet::Single(RegType::UnsignedInt(32))),
                    index: 0,
                }),
                1,
                11..=15,
            ),
            (Operand::Constant(100), 2, 18..=21),
        ],
        loc: 0..=INSTR.len() - 1,
    };
    assert_eq!(exp, parser.parse(INSTR).unwrap())
}

#[test]
fn parse_labelled_statement() {
    let parser = StatementParser::new();
    let prog_str = ".L:\n mov u32:0, #1";

    parser.parse(prog_str).unwrap();
}

#[test]
fn parse_simple_prog() {
    let parser = ProgramParser::new();
    let prog_str = "mov u32:0, #1";

    parser.parse(prog_str).unwrap();
}
