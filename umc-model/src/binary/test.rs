use std::io::Cursor;

use crate::binary::v0;
use crate::instructions::{Instruction, MovParams, NumReg, RegOrConstant};
use crate::operand::RegOperand;
use crate::{NumRegType, Program, RegType, RegisterSet};

#[test]
fn encode_basic_program() {
    let instructions = vec![Instruction::Mov(MovParams::UnsignedInt(
        NumReg {
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
            NumReg {
                index: 0,
                width: 32,
            },
            RegOrConstant::Const(5),
        )),
        Instruction::Mov(MovParams::UnsignedInt(
            NumReg {
                index: 1,
                width: 32,
            },
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
