#[cfg(target_os = "linux")]
fn main() {
    use clap::Parser;
    use std::path::PathBuf;

    /// Translate LLVM IR (.bc or .ll) into UMC assembly
    #[derive(Parser)]
    struct Args {
        input: PathBuf,
        /// Output .umc file (stdout if omitted)
        #[arg(short, long)]
        output: Option<PathBuf>,
    }

    let args = Args::parse();
    let module = if args.input.extension().is_some_and(|e| e == "ll") {
        llvm_ir::Module::from_ir_path(&args.input)
    } else {
        llvm_ir::Module::from_bc_path(&args.input)
    }
    .unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {e}", args.input.display());
        std::process::exit(1)
    });

    let asm = umc_llvm::llvm::translate(&module).to_string();
    match args.output {
        Some(p) => std::fs::write(&p, asm).unwrap_or_else(|e| {
            eprintln!("Failed to write {}: {e}", p.display());
            std::process::exit(1)
        }),
        None => print!("{asm}"),
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("umc-llvm depends on llvm-ir, which is only built on Linux");
    std::process::exit(1);
}
