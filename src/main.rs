pub mod ast;

use std::env;
use std::ops::Range;

use annotate_snippets::Renderer;
use annotate_snippets::{AnnotationKind, Group, Level, Snippet};
use bytecode::Instruction;
use bytecode::Operand;
use bytecode::RegOperand;
use lalrpop_util::lalrpop_mod;
use vm::VirtualMachine;

use crate::assembler::{AssembleError, AssembleInstructionError, InvalidOperandError};
use crate::model::NumRegType;
use crate::model::RegisterSet;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod grammar_tests;

mod assembler;
mod bytecode;
mod model;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        execute_program(&args[1]);
    } else {
        dummy_program();
    }
}
fn execute_program(file: &str) {
    let prog_str = std::fs::read_to_string(file).expect("Failed to read file");
    let prog_parser = grammar::ProgramParser::new();

    let ast_prog = match prog_parser.parse(&prog_str) {
        Ok(ast) => ast,
        Err(e) => {
            let renderer = Renderer::styled();
            let report = format_syntax_error(e, &prog_str);
            eprintln!("{}", renderer.render(&report));
            return;
        }
    };

    let prog_bc = match assembler::compile_prog(ast_prog) {
        Ok(p) => p,
        Err(errors) => {
            display_errors(&prog_str, errors);
            return;
        }
    };
    println!("Compilation Successful");

    println!("Executing program");
    VirtualMachine::new(prog_bc).execute();
}

fn display_errors(prog: &str, errors: Vec<AssembleError>) {
    eprintln!("Assembling failed with {} errors", errors.len());

    let renderer = Renderer::styled();

    for err in errors.into_iter() {
        //eprintln!("Error: {:?}", err);
        let report = format_assemble_error(&err, prog);
        println!("{}", renderer.render(&report));
    }
}

fn dummy_program() {
    let regset = RegisterSet::single_num(NumRegType::UnsignedInt(64));
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

fn format_syntax_error<'a, T>(
    error: lalrpop_util::ParseError<usize, T, ast::ParseError>,
    prog: &'a str,
) -> Vec<Group<'a>> {
    match error {
        lalrpop_util::ParseError::InvalidToken { location } => {
            let line = get_line_num_at(&prog, location);
            vec![
                Level::ERROR.primary_title("Invalid Token").element(
                    Snippet::source(prog).line_start(line).annotation(
                        AnnotationKind::Primary
                            .span(location..location + 1)
                            .label("Unknown token"),
                    ),
                ),
            ]
        }
        lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
            let line = get_line_num_at(&prog, location);
            vec![
                Level::ERROR
                    .primary_title("Unexpected end of file")
                    .element(
                        Snippet::source(prog).line_start(line).annotation(
                            AnnotationKind::Primary
                                .span(location..location + 1)
                                .label(format!("Expected {:?} next", expected)),
                        ),
                    ),
            ]
        }
        lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
            let line = get_line_num_at(&prog, token.0);
            let token_range = token.0..token.2 + 1;
            vec![
                Level::ERROR.primary_title("Unexpected token").element(
                    Snippet::source(prog).line_start(line).annotation(
                        AnnotationKind::Primary
                            .span(token_range)
                            .label(format!("Expected {:?} here", expected)),
                    ),
                ),
            ]
        }
        lalrpop_util::ParseError::ExtraToken { token } => {
            let line = get_line_num_at(&prog, token.0);
            let token_range = token.0..token.2 + 1;
            vec![
                Level::ERROR
                    .primary_title("Expected end of file, but found more")
                    .element(
                        Snippet::source(prog).line_start(line).annotation(
                            AnnotationKind::Primary
                                .span(token_range)
                                .label("Expected nothing here"),
                        ),
                    ),
            ]
        }
        lalrpop_util::ParseError::User { error } => match error {
            ast::ParseError::RegErr(reg_err, range) => {
                let line = get_line_num_at(&prog, *range.start());
                let span = *range.start()..range.end() + 1;
                match reg_err {
                    ast::ParseRegError::InvalidInt(e) => vec![
                        Level::ERROR
                            .primary_title("Invalid integer in register operand")
                            .element(
                                Snippet::source(prog).line_start(line).annotation(
                                    AnnotationKind::Primary
                                        .span(span)
                                        .label(format!("Invalid integer: {}", e)),
                                ),
                            ),
                    ],
                    ast::ParseRegError::InvalidFormat => vec![
                        Level::ERROR
                            .primary_title("Malformed register operand")
                            .element(
                                Snippet::source(prog).line_start(line).annotation(
                                    AnnotationKind::Primary
                                        .span(span)
                                        .label("Incorrect register operand syntax"),
                                ),
                            ),
                    ],
                    ast::ParseRegError::InvalidRegisterType => vec![
                        Level::ERROR.primary_title("Unknown register type").element(
                            Snippet::source(prog).line_start(line).annotation(
                                AnnotationKind::Primary
                                    .span(span)
                                    .label("Not a valid register type"),
                            ),
                        ),
                    ],
                }
            }
        },
    }
}

fn format_assemble_error<'a>(error: &'a AssembleError, prog: &'a str) -> Vec<Group<'a>> {
    match error {
        AssembleError::DuplicateLabel(l, range) => {
            let line = get_line_num_at(prog, *range.start());
            vec![
                Level::ERROR.primary_title("Repeated label").element(
                    Snippet::source(prog).line_start(line).annotation(
                        AnnotationKind::Primary
                            .span(*range.start()..range.end() + 1)
                            .label(format!("The label `{}` is defined before this", l)),
                    ),
                ),
            ]
        }
        AssembleError::InvalidInstruction(instr_error, instr_loc) => {
            let line = get_line_num_at(prog, *instr_loc.start());
            let instr_range = *instr_loc.start()..instr_loc.end() + 1;
            format_assemble_instruction_error(instr_error, instr_range, line, prog)
        }
    }
}

fn format_assemble_instruction_error<'a>(
    instr_error: &'a AssembleInstructionError,
    instr_range: Range<usize>,
    line: usize,
    prog: &'a str,
) -> Vec<Group<'a>> {
    match instr_error {
        AssembleInstructionError::UnknownOpCode(opcode_loc) => {
            vec![
                Level::ERROR.primary_title("Unknown Op Code").element(
                    Snippet::source(prog).line_start(line).annotation(
                        AnnotationKind::Primary
                            .span(*opcode_loc.start()..opcode_loc.end() + 1)
                            .label("No such opcode"),
                    ),
                ),
            ]
        }
        AssembleInstructionError::InvalidOperandCount(expected, got) => {
            vec![
                Level::ERROR
                    .primary_title("Incorrect number of operands")
                    .element(
                        Snippet::source(prog).line_start(line).annotation(
                            AnnotationKind::Primary
                                .span(instr_range)
                                .label(format!("Expected {} operands but got {}", expected, got)),
                        ),
                    ),
            ]
        }
        AssembleInstructionError::InvalidOperand(op_error, op_num, op_loc) => {
            let op_span = *op_loc.start()..op_loc.end() + 1;
            match op_error {
                InvalidOperandError::ExpectedDstReg => vec![
                    Level::ERROR
                        .primary_title("Expected destination register")
                        .element(Snippet::source(prog).line_start(line).annotation(
                            AnnotationKind::Primary.span(op_span).label(format!(
                                "Operand {} should be a destination register!",
                                op_num
                            )),
                        )),
                ],
                InvalidOperandError::CannotInferReg => vec![
                    Level::ERROR
                        .primary_title("Register set cannot be inferred")
                        .element(
                            Snippet::source(prog).line_start(line).annotation(
                                AnnotationKind::Primary
                                    .span(op_span)
                                    .label(format!("The register set cannot be inferred!")),
                            ),
                        ),
                ],
                InvalidOperandError::UnknownLabel(label) => vec![
                    Level::ERROR.primary_title("Undefined Label").element(
                        Snippet::source(prog).line_start(line).annotation(
                            AnnotationKind::Primary
                                .span(op_span)
                                .label(format!("The label `{}` is undefined", label)),
                        ),
                    ),
                ],
                InvalidOperandError::InvalidType => vec![
                    Level::ERROR
                        .primary_title("Operand type not valid for this instruction")
                        .element(
                            Snippet::source(prog).line_start(line).annotation(
                                AnnotationKind::Primary
                                    .span(op_span)
                                    .label("This operand type is not allowed for this instruction"),
                            ),
                        ),
                ],
            }
        }
    }
}

fn get_line_num_at(s: &str, pos: usize) -> usize {
    let mut line = 0;
    for c in s[0..pos].bytes() {
        if c == b'\n' {
            line += 1;
        }
    }
    line
}
