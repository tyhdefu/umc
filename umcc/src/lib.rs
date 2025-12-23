use lalrpop_util::lalrpop_mod;

pub mod assembler;
mod ast;
pub mod error_display;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP
