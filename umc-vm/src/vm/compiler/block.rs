use std::ops::Range;

use umc_model::instructions::*;

pub(crate) fn scan(program: &[Instruction], entry_pc: usize) -> Range<usize> {
    for i in entry_pc..program.len() {
        if is_control_flow(&program[i]) {
            return entry_pc..i + 1;
        }
    }
    return entry_pc..program.len();
}

fn is_control_flow(instr: &Instruction) -> bool {
    match instr {
        Instruction::Bz(..)
        | Instruction::Bnz(..)
        | Instruction::Jmp(..)
        | Instruction::Jal(..) => {
            return true;
        }
        _ => return false,
    }
}
