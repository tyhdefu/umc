pub mod ast;

use std::env;

use bytecode::Instruction;
use bytecode::Operand;
use bytecode::RegOperand;
use lalrpop_util::lalrpop_mod;
use model::RegisterSet;
use vm::VirtualMachine;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod grammar_tests;

mod assembler;
mod bytecode;
mod model;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        execute_program(&args[1]);
    } else {
        dummy_program();
    }
}
fn execute_program(file: &str) {
    let prog_str = std::fs::read_to_string(file).expect("Failed to read file");
    //let instr_parser = grammar::InstructionParser::new();
    let prog_parser = grammar::ProgramParser::new();

    let ast_prog = prog_parser.parse(&prog_str).expect("Parsing failed");
    let prog_bc = assembler::compile_prog(ast_prog).expect("Compilation failed!");

    println!("Compilation Successful");

    println!("Executing program");
    VirtualMachine::new(prog_bc).execute();
}

fn dummy_program() {
    let regset = RegisterSet::Single(model::RegType::UnsignedInt, 64);
    let reg0 = RegOperand {
        set: regset.clone(),
        index: 0,
    };
    let reg1 = RegOperand {
        set: regset.clone(),
        index: 1,
    };
    let reg2 = RegOperand {
        set: regset,
        index: 2,
    };

    let mut vm = VirtualMachine::new(vec![
        Instruction::Mov(reg0.clone(), Operand::UnsignedConstant(5)),
        Instruction::Mov(reg1.clone(), Operand::UnsignedConstant(10)),
        Instruction::Add(reg2.clone(), Operand::Reg(reg0), Operand::Reg(reg1)),
        Instruction::Dbg(reg2),
    ]);
    vm.execute();
}
