pub mod ast;

use bytecode::Instruction;
use bytecode::Operand;
use bytecode::RegOperand;
use bytecode::RegisterSet;
use lalrpop_util::lalrpop_mod;
use vm::VirtualMachine;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod grammar_tests;

mod bytecode;
mod model;
mod vm;

fn main() {
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
