//! Compiler for Universal Machine Code to a specific machine code
//! This is used in Just-in-Time compilation

// Not currently used
#![allow(unused)]

#[cfg(feature = "llvm-jit")]
mod llvm;

use std::ops::RangeInclusive;

use umc_model::instructions::Instruction;

pub struct CompileError {}

pub trait Compiler {
    /// Attempt to compiler a block of UMC Instructions.
    /// The compiler will attempt to compile as many of these as possible, but only a sub-range of this may be compiled,
    /// which will still count as a success, and the range of compiled instructions will be indicated
    fn compile_block(instructions: &[Instruction]) -> Result<CompiledRequest, CompileError>;

    /// TODO: What are the inputs and outputs of this?
    fn execute_block(block: CompiledBlock) -> Result<(), ()>;
}

/// A compiled block of instructions
pub struct CompiledRequest {
    compiled_range: RangeInclusive<usize>,
    compiled: CompiledBlock,
}

pub struct CompiledBlock {}
