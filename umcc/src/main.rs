pub mod ast;

use std::{env, ffi::OsStr, fs::File, io::BufWriter, path::Path};

use lalrpop_util::lalrpop_mod;
use umc_model::binary::encode;

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
    let input_file = Path::new(&args[1]);
    let prog_str = std::fs::read_to_string(&args[1]).expect("Failed to read file");
    let prog_model = error_display::assemble_prog(&prog_str).expect("Compilation Failed");

    let file_name = input_file.file_name().unwrap_or(&OsStr::new("a"));
    let output_file = Path::new(file_name).with_extension("umb");
    if output_file == input_file {
        panic!("Output file would have the same name!");
    }
    let file = File::create(&output_file).expect("Failed to open output file");

    let mut buf_writer = BufWriter::new(file);
    match encode(&prog_model, &mut buf_writer) {
        Ok(()) => match (input_file.to_str(), output_file.to_str()) {
            (Some(x), Some(y)) => println!("Compiled {} -> {}", x, y),
            _ => println!("Compilation successful!"),
        },
        Err(e) => {
            eprintln!("Failed to write bytecode file: {:?}", e);
        }
    }
}
