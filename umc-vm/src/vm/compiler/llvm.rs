use inkwell::{
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    types::{BasicMetadataTypeEnum, IntType},
    values::IntMathValue,
};
use umc_model::{
    instructions::{AddParams, AnyReg, AnySingleReg, ConsistentOp, Instruction},
    reg_model::RegOrConstant,
};

use crate::vm::compiler::{CompileError, CompiledBlock, CompiledRequest, Compiler};

pub struct LLVMCompiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> Compiler for LLVMCompiler<'ctx> {
    fn compile_block(instructions: &[Instruction]) -> Result<CompiledRequest, CompileError> {
        todo!()
    }

    fn execute_block(block: CompiledBlock) -> Result<(), ()> {
        todo!()
    }
}

#[derive(PartialEq, Debug)]
enum LLVMIntType {
    I32,
    I64,
}

#[derive(PartialEq, Debug)]
struct BrokenInstruction {
    /// Arguments pulled from UMC state
    args: Vec<AnyReg>,
    /// One or more LLVM instructions that transform the input, ending in out
    instrs: Vec<LLVMInstruction>,
    /// The output to be returned to UMC state
    out: AnyReg,
}

#[derive(PartialEq, Debug)]
enum LLVMOperand<V> {
    Prev(usize),
    Constant(V),
}

#[derive(PartialEq, Debug)]
enum LLVMInstruction {
    IntAdd(LLVMOperand<u64>, LLVMOperand<u64>, LLVMIntType),
}

#[derive(PartialEq, Debug)]
struct UnsupportedInstruction;

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn create(context: &'ctx Context) -> Self {
        let module = context.create_module("umc-jit");
        let execution_engine = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap();
        Self {
            context,
            module,
            execution_engine,
        }
    }

    fn break_instruction(instr: &Instruction) -> Result<BrokenInstruction, UnsupportedInstruction> {
        match instr {
            Instruction::Add(AddParams::UnsignedInt(op)) => match op {
                ConsistentOp::Single(reg, p, p2) => match reg.width {
                    i32::BITS => {
                        let mut args = vec![];
                        let op1 = match p {
                            RegOrConstant::Reg(reg) => {
                                args.push(AnyReg::Single(AnySingleReg::Unsigned(*reg)));
                                LLVMOperand::Prev(0)
                            }
                            RegOrConstant::Const(v) => LLVMOperand::Constant(*v),
                        };
                        let op2 = match p2 {
                            RegOrConstant::Reg(reg) => {
                                args.push(AnyReg::Single(AnySingleReg::Unsigned(*reg)));
                                LLVMOperand::Prev(1)
                            }
                            RegOrConstant::Const(v) => LLVMOperand::Constant(*v),
                        };
                        Ok(BrokenInstruction {
                            args,
                            instrs: vec![LLVMInstruction::IntAdd(op1, op2, LLVMIntType::I32)],
                            out: AnyReg::Single(AnySingleReg::Unsigned(*reg)),
                        })
                    }
                    _ => Err(UnsupportedInstruction),
                },
                _ => Err(UnsupportedInstruction),
            },
            _ => Err(UnsupportedInstruction),
        }
    }

    fn compile_instruction_chain(
        &mut self,
        name: &str,
        instrs: &[BrokenInstruction],
    ) -> Result<(), ()> {
        if instrs.len() != 1 {
            todo!("Multiple instructions not yet supported");
        }
        let instr = &instrs[0];

        let out_type = match instr.out {
            AnyReg::Single(AnySingleReg::Unsigned(reg)) => {
                assert_eq!(32, reg.width);
                self.context.i32_type()
            }
            _ => todo!(),
        };

        let builder = self.context.create_builder();
        let llvm_arg_types: Vec<_> = instr.args.iter().map(|r| self.to_basic_type(r)).collect();
        let function_type = out_type.fn_type(&llvm_arg_types, false);

        let function = self.module.add_function(name, function_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let llvm_instr = &instr.instrs[0];
        match llvm_instr {
            LLVMInstruction::IntAdd(op1, op2, llvm_type) => {
                assert_eq!(LLVMIntType::I32, *llvm_type);
                let lhs = match op1 {
                    LLVMOperand::Prev(i) => {
                        function.get_nth_param(*i as u32).unwrap().into_int_value()
                    }
                    LLVMOperand::Constant(c) => self.context.i32_type().const_int(*c as u64, false),
                };
                let rhs = match op2 {
                    LLVMOperand::Prev(i) => {
                        function.get_nth_param(*i as u32).unwrap().into_int_value()
                    }
                    LLVMOperand::Constant(c) => self.context.i32_type().const_int(*c as u64, false),
                };
                let r = builder.build_int_add(lhs, rhs, "v").unwrap();
                builder.build_return(Some(&r)).unwrap();
            }
        }

        Ok(())
    }

    fn to_basic_type(&self, reg: &AnyReg) -> BasicMetadataTypeEnum<'ctx> {
        match reg {
            AnyReg::Single(AnySingleReg::Unsigned(reg)) => {
                assert_eq!(32, reg.width);
                BasicMetadataTypeEnum::IntType(self.context.i32_type())
            }
            _ => todo!(),
        }
    }

    fn execute_function(&self, s: &str) -> Result<u32, ()> {
        type NoArgFunc = unsafe extern "C" fn() -> u32;
        unsafe {
            let function: JitFunction<'_, NoArgFunc> =
                self.execution_engine.get_function(s).unwrap();
            return Ok(function.call());
        }
    }
}

#[cfg(test)]
mod test {
    use inkwell::context::Context;
    use inkwell::execution_engine::JitFunction;
    use umc_model::instructions::Instruction;
    use umc_model::reg_model::Reg;

    use super::*;

    #[test]
    fn get_add_instruction() {
        let context = Context::create();
        let module = context.create_module("umc-jit");
        let execution_engine = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap();

        let builder = context.create_builder();
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
        let function = module.add_function("test_add", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");

        builder.position_at_end(basic_block);

        let x = function.get_nth_param(0).unwrap().into_int_value();
        let y = function.get_nth_param(1).unwrap().into_int_value();

        let r = builder.build_int_add(x, y, "r").unwrap();

        builder.build_return(Some(&r)).unwrap();

        type TestAddFunc = unsafe extern "C" fn(u32, u32) -> u32;
        unsafe {
            let build_func: JitFunction<'_, TestAddFunc> =
                execution_engine.get_function("test_add").unwrap();
            let result = build_func.call(5, 1);
            assert_eq!(6, result);
        }
    }

    #[test]
    fn break_simple_add() {
        let instr = Instruction::Add(AddParams::UnsignedInt(ConsistentOp::Single(
            Reg {
                index: 1,
                width: 32,
            },
            RegOrConstant::Const(10),
            RegOrConstant::Const(5),
        )));
        let result = LLVMCompiler::break_instruction(&instr);
        assert_eq!(
            Ok(BrokenInstruction {
                args: vec![],
                instrs: vec![LLVMInstruction::IntAdd(
                    LLVMOperand::Constant(10),
                    LLVMOperand::Constant(5),
                    LLVMIntType::I32,
                )],
                out: AnyReg::Single(AnySingleReg::Unsigned(Reg {
                    index: 1,
                    width: 32
                })),
            }),
            result
        );
    }

    #[test]
    fn compile_and_run_basic_add() {
        let instr = Instruction::Add(AddParams::UnsignedInt(ConsistentOp::Single(
            Reg {
                index: 1,
                width: 32,
            },
            RegOrConstant::Const(11),
            RegOrConstant::Const(5),
        )));

        let context = Context::create();
        let mut compiler = LLVMCompiler::create(&context);

        let broken_instr = LLVMCompiler::break_instruction(&instr).unwrap();
        compiler
            .compile_instruction_chain("basic_add", &[broken_instr])
            .unwrap();
        let result = compiler.execute_function("basic_add");
        assert_eq!(Ok(16), result);
    }
}
