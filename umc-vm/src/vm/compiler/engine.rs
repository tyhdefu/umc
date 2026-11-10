use rustc_hash::{FxHashMap, FxHashSet};

use crate::vm::{
    compiler::{BlockExit, CompiledBlock, Compiler, jit::*},
    memory::safe::{SafeAddress, SafeMemoryManager},
    state::RegState,
};
use umc_model::instructions::Instruction;

pub struct JitEngine {
    pub jit: NativeJit,
    pub counters: Vec<u32>,
    pub blocks: FxHashMap<usize, CompiledBlock>,
    pub blacklist: FxHashSet<usize>,
    pub threshold: u32,
    pub verbose: bool,
}

impl JitEngine {
    // returns Some(exit) if compiled block ran correctly, None if should be interpreted
    // BadExit is a codegen error not a program error so just blacklist block
    pub fn try_run(
        &mut self,
        pc: usize,
        program: &[Instruction],
        state: &mut RegState<SafeAddress>,
        memory: &mut SafeMemoryManager,
        constants: &[SafeAddress],
    ) -> Option<BlockExit> {
        // Blacklisted blocks should be interpreted
        if self.blacklist.contains(&pc) {
            return None;
        }
        // COLD
        if self.counters[pc] < self.threshold {
            self.counters[pc] += 1;
            return None;
        } else if !self.blocks.contains_key(&pc) {
            // HOT - NOT COMPILED
            println!("Threshold reached for block at pc: {pc}, compiling block...");
            match self.jit.compile_block(program, pc) {
                Ok(request) => self.blocks.insert(pc, request.block),
                Err(_) => {
                    println!("Failed compilation: blacklisting block...");
                    self.blacklist.insert(pc);
                    return None;
                }
            };
        }
        // HOT & COMPILED
        match self
            .jit
            .execute_block(self.blocks.get(&pc)?, state, memory, constants, pc)
        {
            Ok(exit) => return Some(exit),
            Err(_) => {
                self.blacklist.insert(pc);
                println!("Failed execution of compiled block, blacklisting...");
                return None;
            }
        }
    }
}
