use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;

use clap::Parser;
use umc_model::binary::decode;
use vm::VirtualMachine;

use umc_model::Program;

use crate::vm::VMOptions;

mod vm;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The UMC file to run (bytecode or assembly)
    program: PathBuf,

    /// Turn verbose mode on
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let path = args.program;
    let is_assembly = path.extension().is_some_and(|ext| ext == "umc");

    let prog: Program = if is_assembly {
        let prog_str = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read file: {:?} - {}", &path, e);
                return;
            }
        };
        match umc_compiler::error_display::assemble_prog(&prog_str) {
            Ok(prog) => prog,
            Err(_) => return,
        }
    } else {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open file: {:?} - {}", &path, e);
                return;
            }
        };
        match file.metadata() {
            Ok(m) => {
                if m.is_dir() {
                    eprintln!("Expected program to be a file, not a directory!");
                    return;
                }
            }
            Err(e) => {
                eprintln!("Failed to read file metadata: {:?} - {}", &path, e);
            }
        }
        let buf_reader = BufReader::new(file);
        decode(buf_reader).expect("Invalid UMC Bytecode file")
    };

    let options = VMOptions {
        verbose: args.verbose,
    };
    println!("Executing program");
    VirtualMachine::new(prog, options).execute();
}
