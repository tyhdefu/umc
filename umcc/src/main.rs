pub mod ast;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use clap::Parser;
use lalrpop_util::lalrpop_mod;
use umc_model::Program;
use umc_model::binary::{DisassembleResult, DisassemblyInfo, InnerDisassembly, encode};
use umc_model::format::DisplayAssemblyParams;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod grammar_tests;

mod assembler;
mod error_display;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The UMC file to compile
    file: PathBuf,

    /// Whether to disassemble instead, printing disassembly to standard output
    #[clap(short, long)]
    disassemble: bool,
}

fn main() {
    let args = Args::parse();

    let input_file = Path::new(&args.file);

    if args.disassemble {
        disassemble(input_file);
    } else {
        assemble(input_file);
    }
}

fn assemble(input_file: &Path) {
    let prog_str = std::fs::read_to_string(&input_file).expect("Failed to read file");
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

fn disassemble(input_file: &Path) {
    let file = File::open(input_file).expect("Failed to open input file");
    let buf_reader = BufReader::new(file);
    match umc_model::binary::disassemble(buf_reader) {
        DisassembleResult::Failed(decode_error) => {
            eprintln!("Bad file: {:?}", decode_error);
        }
        DisassembleResult::Partial(disassembly_info, decode_error) => {
            eprintln!("Invalid UMC Bytecode: {:?}", decode_error);
            eprintln!("Warning: Partial Disassembly");
            print_disassembly(disassembly_info, None);
        }
        DisassembleResult::Full(program, disassembly_info) => {
            print_disassembly(disassembly_info, Some(program));
        }
    }
}

fn print_disassembly(info: DisassemblyInfo, prog: Option<Program>) {
    println!("; UMC Bytecode File Version {}", info.get_version());
    match info.inner {
        InnerDisassembly::None => {
            if let Some(prog) = prog {
                eprintln!("Only basic disassembly available");
                println!("{}", prog);
            }
        }
        InnerDisassembly::V0(v0_dissassembler) => {
            let prog = prog.unwrap_or_else(|| Program {
                instructions: v0_dissassembler.instructions(),
                pre_init_mem: vec![],
                mem_labels: HashMap::new(),
                instr_labels: HashMap::new(),
            });
            let instr_labels = prog.create_instr_labels();
            let mem_labels = prog.create_mem_labels();

            let opts = DisplayAssemblyParams::WithSymbols {
                instr_labels: &instr_labels,
                mem_labels: &mem_labels,
            };

            for (m_const, m_label) in &mem_labels {
                let data: Vec<String> = prog.pre_init_mem[*m_const]
                    .iter()
                    .map(|b| format!("{:#X}", b))
                    .collect();
                println!("&{}: [{}]", m_label, data.join(","));
            }

            println!("{}", v0_dissassembler.to_instruction_assembly(&opts));
        }
    }
}
