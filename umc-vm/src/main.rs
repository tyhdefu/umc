use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use umc_model::binary::decode;
use umc_model::reg_model::{Reg, RegOrConstant};
use vm::VirtualMachine;

use umc_model::Program;
use umc_model::instructions::{
    AddParams, AnyReg, AnySingleReg, ConsistentOp, Instruction, MovParams,
};

mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let path = Path::new(&args[1]);
        let prog: Program = if path.extension().is_some_and(|ext| ext == "umc") {
            let prog_str = std::fs::read_to_string(&args[1]).expect("Failed to read umc file");
            umc_compiler::error_display::assemble_prog(&prog_str)
                .expect("Failed to assemble program")
        } else {
            let file = File::open(path).expect("Failed to open file");
            let buf_reader = BufReader::new(file);
            decode(buf_reader).expect("Invalid UMC Bytecode file")
        };

        println!("Executing program");
        VirtualMachine::new(prog).execute();
    } else {
        dummy_program();
    }
}

fn dummy_program() {
    let reg0 = Reg {
        index: 0,
        width: u64::BITS,
    };
    let reg1 = Reg {
        index: 1,
        width: u64::BITS,
    };
    let reg2 = Reg {
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
            Instruction::Add(AddParams::UnsignedInt(ConsistentOp::Single(
                reg2.clone(),
                RegOrConstant::from_reg(reg1),
                RegOrConstant::from_reg(reg0),
            ))),
            Instruction::Dbg(AnyReg::Single(AnySingleReg::Unsigned(reg2))),
        ],
    };

    let mut vm = VirtualMachine::new(prog);
    vm.execute();
}
