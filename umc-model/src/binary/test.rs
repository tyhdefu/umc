use std::io::Cursor;

use crate::binary::{decode, encode, v0};
use crate::instructions::{
    BinaryCondition, CompareParams, CompareToZero, ConsistentComparison, Instruction, MovParams,
    NotParams,
};
use crate::operand::RegOperand;
use crate::reg_model::{NumReg, Reg, RegOrConstant};
use crate::{NumRegType, Program, RegType, RegisterSet};

#[test]
fn encode_basic_program() {
    let instructions = vec![Instruction::Mov(MovParams::UnsignedInt(
        Reg(NumReg {
            index: 1,
            width: 32,
        }),
        RegOrConstant::Const(23),
    ))];

    let prog = Program { instructions };

    let mut buffer = vec![];
    v0::encode(&prog, &mut buffer).expect("Failed to encode program");

    print!("BUFFER [");
    for b in &buffer {
        print!("{:08b},", b);
    }
    println!("]");

    let mut cursor = Cursor::new(buffer);
    let decoded_prog = v0::decode(&mut cursor).expect("Failed to decode program");

    assert_eq!(prog.instructions, decoded_prog.instructions);
}

#[test]
fn encode_mov_add_program() {
    let instructions = vec![
        Instruction::Mov(MovParams::UnsignedInt(
            Reg(NumReg {
                index: 0,
                width: 32,
            }),
            RegOrConstant::Const(5),
        )),
        Instruction::Mov(MovParams::UnsignedInt(
            Reg(NumReg {
                index: 1,
                width: 32,
            }),
            RegOrConstant::Const(7),
        )),
        Instruction::Dbg(RegOperand {
            set: RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(32))),
            index: 2,
        }),
    ];

    let prog = Program { instructions };

    let mut buffer = vec![];

    v0::encode(&prog, &mut buffer).expect("Failed to encode program");

    let mut cursor = Cursor::new(buffer);

    let decoded_prog = v0::decode(&mut cursor).expect("Failed to decode program");
    assert_eq!(prog.instructions, decoded_prog.instructions);
}

#[test]
fn encode_complex_prog() {
    let u32_0 = NumReg {
        index: 0,
        width: 32,
    };
    let u1_0 = NumReg { index: 0, width: 1 };

    let instructions = vec![
        Instruction::Mov(MovParams::UnsignedInt(
            Reg(u32_0.clone()),
            RegOrConstant::Const(5),
        )),
        // u1:0 = 0
        Instruction::Compare {
            cond: BinaryCondition::Equal,
            params: CompareParams {
                dst: Reg(u1_0.clone()),
                args: ConsistentComparison::UnsignedCompare(
                    RegOrConstant::reg(u32_0.clone()),
                    RegOrConstant::Const(10),
                ),
            },
        },
        Instruction::Not(NotParams::UnsignedInt(
            Reg(u1_0.clone()),
            RegOrConstant::reg(u1_0.clone()),
        )),
        Instruction::Bz(
            RegOrConstant::Const(2),
            CompareToZero::Unsigned(RegOrConstant::reg(u1_0)),
        ),
    ];

    let program = Program { instructions };

    let mut buffer = vec![];
    encode(&program, &mut buffer).expect("Failed to encode program");

    let mut cursor = Cursor::new(buffer);
    let decoded_prog = decode(&mut cursor).expect("Failed to decode program");

    assert_eq!(program.instructions, decoded_prog.instructions);
}
