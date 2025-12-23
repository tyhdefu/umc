use std::env;

use vm::VirtualMachine;

use umc_model::instructions::{
    AnyCoherentNumOp, ConsistentNumOp, Instruction, MovParams, NumReg, RegOrConstant,
};
use umc_model::operand::RegOperand;
use umc_model::{NumRegType, RegType};
use umc_model::{Program, RegisterSet};

mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let prog_str = std::fs::read_to_string(&args[1]).expect("Failed to read file");
        let prog = umc_compiler::error_display::assemble_prog(&prog_str)
            .expect("Failed to assemble program");
        println!("Executing program");
        VirtualMachine::new(prog).execute();
    } else {
        dummy_program();
    }
}

fn dummy_program() {
    let reg0 = NumReg {
        index: 0,
        width: u64::BITS,
    };
    let reg1 = NumReg {
        index: 1,
        width: u64::BITS,
    };
    let reg2 = NumReg {
        index: 2,
        width: u64::BITS,
    };

    let prog = Program {
        instructions: vec![
            Instruction::Mov(MovParams::UnsignedInt(
                reg0.clone(),
                RegOrConstant::Const(5),
            )),
            Instruction::Mov(MovParams::UnsignedInt(
                reg1.clone(),
                RegOrConstant::Const(10),
            )),
            Instruction::Add(AnyCoherentNumOp::UnsignedInt(ConsistentNumOp::Single(
                reg2,
                RegOrConstant::Reg(reg1),
                RegOrConstant::Reg(reg0),
            ))),
            Instruction::Dbg(RegOperand {
                set: RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(u64::BITS))),
                index: 2,
            }),
        ],
    };

    let mut vm = VirtualMachine::new(prog);
    vm.execute();
}
