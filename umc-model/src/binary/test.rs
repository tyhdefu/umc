use std::io::Cursor;

use crate::Program;
use crate::binary::{decode, encode, v0};
use crate::instructions::{
    AnyReg, AnySingleReg, BinaryCondition, CompareParams, CompareToZero, ConsistentComparison,
    Instruction, MovParams, NotParams,
};
use crate::reg_model::{Reg, RegOrConstant, UnsignedRegT};

#[test]
fn encode_basic_program() {
    let instructions = vec![Instruction::Mov(MovParams::UnsignedInt(
        Reg {
            index: 1,
            width: 32,
        },
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
            Reg {
                index: 0,
                width: 32,
            },
            RegOrConstant::Const(5),
        )),
        Instruction::Mov(MovParams::UnsignedInt(
            Reg {
                index: 1,
                width: 32,
            },
            RegOrConstant::Const(7),
        )),
        Instruction::Dbg(AnyReg::Single(AnySingleReg::Unsigned(Reg {
            index: 2,
            width: 32,
        }))),
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
    let u32_0: Reg<UnsignedRegT> = Reg {
        index: 0,
        width: 32,
    };
    let u1_0: Reg<UnsignedRegT> = Reg { index: 0, width: 1 };

    let instructions = vec![
        Instruction::Mov(MovParams::UnsignedInt(
            u32_0.clone(),
            RegOrConstant::Const(5),
        )),
        // u1:0 = 0
        Instruction::Compare {
            cond: BinaryCondition::Equal,
            params: CompareParams {
                dst: u1_0.clone(),
                args: ConsistentComparison::UnsignedCompare(
                    RegOrConstant::from_reg(u32_0.clone()),
                    RegOrConstant::Const(10),
                ),
            },
        },
        Instruction::Not(NotParams::UnsignedInt(
            u1_0.clone(),
            RegOrConstant::from_reg(u1_0.clone()),
        )),
        Instruction::Bz(
            RegOrConstant::Const(2),
            CompareToZero::Unsigned(RegOrConstant::from_reg(u1_0)),
        ),
    ];

    let program = Program { instructions };

    let mut buffer = vec![];
    encode(&program, &mut buffer).expect("Failed to encode program");

    let mut cursor = Cursor::new(buffer);
    let decoded_prog = decode(&mut cursor).expect("Failed to decode program");

    assert_eq!(program.instructions, decoded_prog.instructions);
}
