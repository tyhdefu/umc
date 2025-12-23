pub mod ast;

use std::env;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod grammar_tests;

mod assembler;
mod error_display;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You must provide a .umc file to assemble!");
        return;
    }
    let prog_str = std::fs::read_to_string(&args[1]).expect("Failed to read file");
    let _ = error_display::assemble_prog(&prog_str).expect("Compilation Failed");
    println!("Compilation successful!")
}
