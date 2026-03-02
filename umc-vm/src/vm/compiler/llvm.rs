use std::collections::HashMap;

use inkwell::AddressSpace;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{FunctionValue, PointerValue};
use umc_model::{
    instructions::{AddParams, AnyReg, AnySingleReg, ConsistentOp, Instruction},
    reg_model::RegOrConstant,
};

use crate::vm::state::RegState;
use crate::vm::{
    compiler::{CompileError, CompiledBlock, CompiledRequest, Compiler},
    helper,
};
use crate::vm::{memory::safe::SafeAddress, state::StorePrim};

pub struct LLVMCompiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    compiled_blocks: HashMap<String, ExecutableBlockFn>,
    builtin_funcs: BuiltInFunctions<'ctx>,
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
struct LLVMInstructionSequence {
    /// One or more LLVM instructions
    instrs: Vec<LLVMInstruction>,
}

#[derive(PartialEq, Debug)]
enum LLVMOperand<V> {
    Reg(AnyReg),
    Constant(V),
}

impl<V> LLVMOperand<V> {
    pub fn acc_reg(&self, vec: &mut Vec<AnyReg>) {
        match self {
            LLVMOperand::Reg(any_reg) => {
                if !vec.contains(any_reg) {
                    vec.push(any_reg.clone());
                }
            }
            _ => {}
        }
    }
}

#[derive(PartialEq, Debug)]
enum LLVMInstruction {
    IntAdd(AnyReg, LLVMOperand<u64>, LLVMOperand<u64>, LLVMIntType),
}

impl LLVMInstruction {
    pub fn acc_uses(&self, used: &mut Vec<AnyReg>) {
        match self {
            LLVMInstruction::IntAdd(_, a, b, _) => {
                a.acc_reg(used);
                b.acc_reg(used);
            }
        }
    }

    pub fn acc_modified(&self, modified: &mut Vec<AnyReg>) {
        let mut acc = |a: &AnyReg| {
            if !modified.contains(a) {
                modified.push(a.clone());
            }
        };
        match self {
            LLVMInstruction::IntAdd(r, _, _, _) => acc(r),
        }
    }
}

#[derive(PartialEq, Debug)]
struct UnsupportedInstruction;

struct BuiltInFunctions<'ctx> {
    get_u32: FunctionValue<'ctx>,
    save_u32: FunctionValue<'ctx>,
}

#[unsafe(no_mangle)]
pub extern "C" fn putchard(x: f64) -> f64 {
    println!("{}", x as u8 as char);
    x
}

#[unsafe(no_mangle)]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("{x}");
    x
}

// Adding the functions above to a global array,
// so Rust compiler won't remove them.
#[used]
static EXTERNAL_FNS: [extern "C" fn(f64) -> f64; 2] = [putchard, printd];

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn create(context: &'ctx Context) -> Self {
        let module = context.create_module("umc-jit");

        let ptr_type =
            BasicMetadataTypeEnum::PointerType(context.ptr_type(AddressSpace::default()));
        let get_u32_type = context.i32_type().fn_type(
            &[BasicMetadataTypeEnum::IntType(context.i32_type()), ptr_type],
            false,
        );
        // Declare the get_u32 function
        let get_u32 = module.add_function("get_u32", get_u32_type, Some(Linkage::External));

        let save_u32_type = context.void_type().fn_type(
            &[
                BasicMetadataTypeEnum::IntType(context.i32_type()),
                BasicMetadataTypeEnum::PointerType(context.ptr_type(AddressSpace::default())),
                BasicMetadataTypeEnum::IntType(context.i32_type()),
            ],
            false,
        );
        let save_u32 = module.add_function("save_u32", save_u32_type, Some(Linkage::External));

        println!("{}", module.print_to_string());

        let execution_engine = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap();

        println!(
            "save_u32 (EE): {:?}",
            execution_engine.get_function_address("save_u32")
        );

        println!("save_u32 (Module): {:?}", module.get_function("save_u32"));

        Self {
            context,
            module,
            execution_engine,
            compiled_blocks: HashMap::new(),
            builtin_funcs: BuiltInFunctions { get_u32, save_u32 },
        }
    }

    fn convert_instruction(
        instr: &Instruction,
    ) -> Result<LLVMInstructionSequence, UnsupportedInstruction> {
        match instr {
            Instruction::Add(AddParams::UnsignedInt(op)) => match op {
                ConsistentOp::Single(reg, p, p2) => match reg.width {
                    i32::BITS => {
                        let op1 = match p {
                            RegOrConstant::Reg(reg) => {
                                LLVMOperand::Reg(AnyReg::Single(AnySingleReg::Unsigned(*reg)))
                            }
                            RegOrConstant::Const(v) => LLVMOperand::Constant(*v),
                        };
                        let op2 = match p2 {
                            RegOrConstant::Reg(reg) => {
                                LLVMOperand::Reg(AnyReg::Single(AnySingleReg::Unsigned(*reg)))
                            }
                            RegOrConstant::Const(v) => LLVMOperand::Constant(*v),
                        };
                        Ok(LLVMInstructionSequence {
                            instrs: vec![LLVMInstruction::IntAdd(
                                AnyReg::Single(AnySingleReg::Unsigned(*reg)),
                                op1,
                                op2,
                                LLVMIntType::I32,
                            )],
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
        chain: &[LLVMInstructionSequence],
    ) -> Result<CompiledInstructions, ()> {
        if chain.len() != 1 {
            todo!("Multiple instructions not yet supported");
        }
        let seq = &chain[0];
        let builder = self.context.create_builder();

        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let function_type = self.context.void_type().fn_type(
            &[
                // PassedState pointer
                BasicMetadataTypeEnum::PointerType(ptr_type.clone()),
            ],
            false,
        );

        let function = self.module.add_function(name, function_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        // All UMC registers whose values are used (i.e. values matter)
        let mut used_regs: Vec<AnyReg> = vec![];
        // All UMC registers who are modified in the block (i.e. need saving)
        let mut modified_regs: Vec<AnyReg> = vec![];
        for i in &seq.instrs {
            i.acc_uses(&mut used_regs);
            i.acc_modified(&mut modified_regs);
        }

        // All UMC registers mentioned in this block (used or modified)
        let mut all_regs: Vec<AnyReg> = used_regs.clone();

        for m in &modified_regs {
            if !all_regs.contains(m) {
                all_regs.push(m.clone());
            }
        }
        // alloca a memory address slot for each all umc registers.
        let mut umc_llvm_registers: Vec<PointerValue<'ctx>> = vec![];

        for reg in &all_regs {
            let reg_name = format!("{}", reg).replace(':', "_");
            match reg {
                AnyReg::Single(AnySingleReg::Unsigned(r)) => {
                    assert_eq!(32, r.width);
                    let r = builder
                        .build_alloca(self.context.i32_type(), &reg_name)
                        .unwrap();
                    umc_llvm_registers.push(r);
                }
                AnyReg::Single(AnySingleReg::Signed(r)) => {
                    assert_eq!(32, r.width);
                    let r = builder
                        .build_alloca(self.context.i32_type(), &reg_name)
                        .unwrap();
                    umc_llvm_registers.push(r);
                }
                _ => todo!(),
            }
        }
        // Get the LLVM memory slot for the given register
        let get_llvm_reg_ptr = |a: &AnyReg| {
            let index = all_regs.iter().position(|x| x == a).unwrap();
            (index, umc_llvm_registers[index])
        };
        println!(
            "EE func {:?}",
            self.execution_engine.get_function_address("get_u32")
        );
        println!("func {:?}", self.module.get_function("get_u32"));

        let passed_state_ptr = function.get_nth_param(0).unwrap();

        // load starting values in for all used registers
        for used in &used_regs {
            let reg_name = format!("{}_v", used).replace(':', "_");
            let (i, ptr) = get_llvm_reg_ptr(used);
            match used {
                AnyReg::Single(AnySingleReg::Unsigned(r)) => {
                    assert_eq!(32, r.width);
                    let r = builder
                        .build_call(
                            self.builtin_funcs.get_u32,
                            &[
                                passed_state_ptr.into(),
                                self.context.i32_type().const_int(i as u64, false).into(),
                            ],
                            &format!("{}_v", reg_name),
                        )
                        .unwrap();
                    let v = r.try_as_basic_value().basic().unwrap().into_int_value();
                    builder.build_store(ptr, v).unwrap();
                }
                AnyReg::Single(AnySingleReg::Signed(r)) => {
                    assert_eq!(32, r.width);
                    // let r = builder
                    // .build_alloca(self.context.i32_type(), &reg_name)
                    // .unwrap();
                }
                _ => todo!(),
            }
        }

        let llvm_instr = &seq.instrs[0];
        match llvm_instr {
            LLVMInstruction::IntAdd(dst, op1, op2, llvm_type) => {
                assert_eq!(LLVMIntType::I32, *llvm_type);
                let lhs = match op1 {
                    LLVMOperand::Reg(arg) => {
                        let (i, ptr) = get_llvm_reg_ptr(arg);
                        let v = builder
                            .build_load(self.context.i32_type(), ptr, "tmp")
                            .unwrap();
                        v.into_int_value()
                    }
                    LLVMOperand::Constant(c) => self.context.i32_type().const_int(*c as u64, false),
                };
                let rhs = match op2 {
                    LLVMOperand::Reg(arg) => {
                        let (i, ptr) = get_llvm_reg_ptr(arg);
                        let v = builder
                            .build_load(self.context.i32_type(), ptr, "tmp")
                            .unwrap();
                        v.into_int_value()
                    }
                    LLVMOperand::Constant(c) => self.context.i32_type().const_int(*c as u64, false),
                };
                let r = builder.build_int_add(lhs, rhs, "r").unwrap();
                let (i, dst_ptr) = get_llvm_reg_ptr(dst);
                builder.build_store(dst_ptr, r).unwrap();
            }
        }

        // Save all modified registers
        for modified in &modified_regs {
            let (i, ptr) = get_llvm_reg_ptr(modified);
            let reg_name = format!("{}_v", modified);
            match modified {
                AnyReg::Single(AnySingleReg::Unsigned(r)) => {
                    assert_eq!(32, r.width);
                    let v = builder
                        .build_load(self.context.i32_type(), ptr, &reg_name)
                        .unwrap();
                    builder
                        .build_call(
                            self.builtin_funcs.save_u32,
                            &[
                                self.context.i32_type().const_int(i as u64, false).into(),
                                passed_state_ptr.into_pointer_value().into(),
                                v.into_int_value().into(),
                            ],
                            name,
                        )
                        .unwrap();
                }
                _ => todo!(),
            }
        }

        builder.build_return(None).unwrap();
        println!("MODULE:\n{}", self.module.print_to_string().to_string());

        unsafe {
            let function: JitFunction<'ctx, ExecutableBlockFn> =
                self.execution_engine.get_function(name).unwrap();
            Ok(CompiledInstructions {
                func: function,
                registers: all_regs,
            })
        }
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
}

type ExecutableBlockFn = unsafe extern "C" fn(*const PassedState);

struct CompiledInstructions<'ctx> {
    func: JitFunction<'ctx, ExecutableBlockFn>,
    registers: Vec<AnyReg>,
}

impl<'ctx> CompiledInstructions<'ctx> {
    pub fn execute(&self, state: &PassedState) -> Result<(), ()> {
        unsafe {
            self.func.call(state);
        }
        Ok(())
    }
}

/// State passed to the
struct PassedState {
    state: *mut RegState<SafeAddress>,
    regs: Vec<AnyReg>,
}

impl PassedState {
    pub fn get_u32(&self, reg_n: u32) -> u32 {
        let s = unsafe { &mut *self.state };
        let reg = &self.regs[reg_n as usize];
        match reg {
            AnyReg::Single(AnySingleReg::Unsigned(r)) => {
                let v: u32 = helper::read_uint(&RegOrConstant::Reg(*r), s);
                v
            }
            AnyReg::Single(AnySingleReg::Signed(r)) => {
                let v: i32 = helper::read_int(&RegOrConstant::Reg(*r), s);
                v as u32
            }
            _ => panic!("Invalid register: get_u32 with {:?}", reg_n),
        }
    }

    pub fn get_u64(&self, reg_n: u32) -> u64 {
        let s = unsafe { &mut *self.state };
        let reg = &self.regs[reg_n as usize];
        match reg {
            AnyReg::Single(AnySingleReg::Unsigned(r)) => {
                let v: u64 = helper::read_uint(&RegOrConstant::Reg(*r), s);
                v
            }
            AnyReg::Single(AnySingleReg::Signed(r)) => {
                let v: i64 = helper::read_int(&RegOrConstant::Reg(*r), s);
                v as u64
            }
            _ => panic!("Invalid register: get_u32 with {:?}", reg_n),
        }
    }

    pub fn save_u32(&mut self, reg_n: u32, v: u32) {
        let s = unsafe { &mut *self.state };
        let reg = &self.regs[reg_n as usize];
        println!("Storing {} into {}", v, reg);
        match reg {
            AnyReg::Single(AnySingleReg::Unsigned(r)) => {
                s.store_prim(*r, v);
            }
            AnyReg::Single(AnySingleReg::Signed(r)) => {
                s.store_prim(*r, v as i32);
            }
            _ => panic!("Invalid register: save_u32 with {:?}", reg),
        }
    }
}

mod read_state {
    use super::*;

    use crate::vm::compiler::llvm::PassedState;

    #[used]
    static USED_GET: unsafe extern "C" fn(u32, &PassedState) -> u32 = get_u32;

    #[unsafe(export_name = "get_u32")]
    pub unsafe extern "C" fn get_u32(reg: u32, state: &PassedState) -> u32 {
        42
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn get_u64(reg: u32, state: &PassedState) -> u64 {
        state.get_u64(reg)
    }

    #[used]
    static USED_SAVE: unsafe extern "C" fn(u32, &mut PassedState, u32) = save_u32;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn save_u32(reg: u32, state: &mut PassedState, v: u32) {
        println!("saving u32");
        state.save_u32(reg, v);
    }
}

#[used]
static USED_DUMMY: unsafe extern "C" fn() -> u32 = dummy_fun;

#[unsafe(export_name = "dummy_fun")]
pub unsafe extern "C" fn dummy_fun() -> u32 {
    1
}

#[cfg(test)]
mod test {
    use std::ffi::{CStr, c_void};

    use inkwell::context::Context;
    use inkwell::execution_engine::JitFunction;
    use inkwell::llvm_sys::support::{LLVMAddSymbol, LLVMSearchForAddressOfSymbol};
    use umc_model::instructions::Instruction;
    use umc_model::reg_model::Reg;

    use crate::vm::compiler::llvm::read_state::save_u32;

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
        let result = LLVMCompiler::convert_instruction(&instr);
        assert_eq!(
            Ok(LLVMInstructionSequence {
                instrs: vec![LLVMInstruction::IntAdd(
                    AnyReg::Single(AnySingleReg::Unsigned(Reg {
                        index: 1,
                        width: 32
                    })),
                    LLVMOperand::Constant(10),
                    LLVMOperand::Constant(5),
                    LLVMIntType::I32,
                )],
            }),
            result
        );
    }

    #[test]
    fn compile_and_run_basic_add() {
        let c_str = CStr::from_bytes_with_nul(b"save_u32\0").unwrap();
        unsafe {
            LLVMAddSymbol(c_str.as_ptr(), save_u32 as *mut c_void);
        }

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

        let broken_instr = LLVMCompiler::convert_instruction(&instr).unwrap();
        let compiled_fn = compiler
            .compile_instruction_chain("basic_add", &[broken_instr])
            .unwrap();
        let mut reg_state = RegState::new();

        {
            let passed_state = PassedState {
                state: &mut reg_state,
                regs: vec![AnyReg::Single(AnySingleReg::Unsigned(Reg {
                    index: 1,
                    width: 32,
                }))],
            };
            println!("Executing function");
            compiled_fn.execute(&passed_state).unwrap();
        }

        let got: Option<u32> = reg_state.read_prim(Reg {
            index: 1,
            width: 32,
        });

        assert_eq!(Some(16), got);
    }
}
