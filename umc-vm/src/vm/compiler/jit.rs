use crate::vm::compiler::block::scan;
use crate::vm::compiler::lower::lower_block;
use crate::vm::compiler::{
    BlockExit, CompileError, CompiledBlock, CompiledRequest, CompiledStats, Compiler, ExecuteError,
    ExitTo, bridge::FrameBuffer, emit::Emitter, emit::NativeEmitter, exec_mem::ExecPage,
};
use crate::vm::memory::safe::SafeMemoryManager;
use crate::vm::{RegState, SafeAddress};

use std::ffi::c_void;
use std::marker::PhantomData;
use umc_model::instructions::Instruction;

pub type NativeJit = Jit<NativeEmitter>;

pub struct Jit<E: Emitter> {
    frame: FrameBuffer,
    _emitter: PhantomData<E>,
}

impl<E: Emitter> Jit<E> {
    pub fn new() -> Self {
        Self {
            frame: FrameBuffer::new(),
            _emitter: PhantomData,
        }
    }
}

impl<E: Emitter> Compiler for Jit<E> {
    // compile basic block starting at 'entry_pc'
    fn compile_block<'p>(
        &mut self,
        program: &'p [Instruction],
        entry_pc: usize,
    ) -> Result<CompiledRequest<'p>, CompileError> {
        let covered = scan(program, entry_pc);
        match lower_block::<E>(program, covered.clone()) {
            Ok(lowered) => {
                let page = match ExecPage::commit(&lowered.code) {
                    Ok(p) => p,
                    Err(e) => return Err(CompileError::ExecMem(e)),
                };
                // clone before layout is moved into the block
                let slots = lowered.layout.slots.clone();
                Ok(CompiledRequest {
                    block: CompiledBlock {
                        entry_pc,
                        covered,
                        layout: lowered.layout,
                        exits: lowered.exits,
                        page,
                    },
                    stats: CompiledStats {
                        program,
                        code: lowered.code,
                        slots,
                    },
                })
            }
            Err(e) => Err(e),
        }
    }

    // load registers from 'state' to frame, call host code, spill registers, return where to continue from
    fn execute_block(
        &mut self,
        block: &CompiledBlock,
        state: &mut RegState<SafeAddress>,
        memory: &mut SafeMemoryManager,
        constants: &[SafeAddress],
        entry_pc: usize,
    ) -> Result<BlockExit, ExecuteError> {
        // load register states into frame layout
        self.frame.prepare(&block.layout, state, constants);

        // convert page into CompiledFn
        let f = unsafe { block.page.as_fn() };
        // convert frame into mut ptr to pass into CompiledFn
        let frame_ptr = self.frame.as_mut_ptr();

        // Execute compiled code
        let exit_id = unsafe { f(frame_ptr, memory as *mut _ as *mut c_void) };

        // spill results from frame back into register state
        self.frame.commit(&block.layout, state);

        // exit_id indexes into block.exits to give where to continue
        match block.exits.get(exit_id as usize) {
            Some(ExitTo::Static(exit)) => Ok(*exit),
            Some(ExitTo::Slot(slot)) => Ok(BlockExit::Goto(self.frame.get(*slot) as usize)),
            None => Err(ExecuteError::BadExit { entry_pc, exit_id }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::compiler::CompileError;
    use crate::vm::helper::{read_float, read_iaddr, read_uint};
    use crate::vm::state::StorePrim;
    use crate::vm::types::CastSingleFloat;
    use crate::vm::widths::uint::UIntWidth;
    use umc_compiler::error_display::assemble_prog;
    use umc_model::instructions::{AddParams, AnyConsistentNumOp, ConsistentOp};
    use umc_model::reg_model::{FloatRegT, InstrRegT, Reg, RegOrConstant, UnsignedRegT};
    use umc_model::{RegIndex, RegWidth};

    fn unsigned(index: RegIndex, width: RegWidth) -> Reg<UnsignedRegT> {
        Reg { index, width }
    }

    fn u64_reg(index: RegIndex) -> Reg<UnsignedRegT> {
        unsigned(index, 64)
    }

    // compile block at pc 0, run it once with given u64 regs set
    fn run_block(src: &str, regs: &[(RegIndex, u64)]) -> (BlockExit, RegState<SafeAddress>) {
        let regs: Vec<_> = regs.iter().map(|&(i, v)| (u64_reg(i), v)).collect();
        run_block_with(src, &regs)
    }

    fn run_block_with(
        src: &str,
        regs: &[(Reg<UnsignedRegT>, u64)],
    ) -> (BlockExit, RegState<SafeAddress>) {
        run_instructions(&assemble_prog(src).unwrap().instructions, regs)
    }

    // for instructions the assembler can't produce
    fn run_instructions(
        program: &[Instruction],
        regs: &[(Reg<UnsignedRegT>, u64)],
    ) -> (BlockExit, RegState<SafeAddress>) {
        run_on(program, |state| {
            for &(reg, v) in regs {
                UIntWidth::store_u64(reg, state, v);
            }
        })
    }

    // compile at pc 0, run once on a state built by setup
    fn run_on(
        program: &[Instruction],
        setup: impl FnOnce(&mut RegState<SafeAddress>),
    ) -> (BlockExit, RegState<SafeAddress>) {
        let mut jit = NativeJit::new();
        let request = jit.compile_block(program, 0).unwrap();
        let mut state = RegState::new();
        let mut memory = SafeMemoryManager::new();
        setup(&mut state);
        let exit = jit
            .execute_block(&request.block, &mut state, &mut memory, &[], 0)
            .unwrap();
        (exit, state)
    }

    fn compile_err(src: &str) -> CompileError {
        compile_err_instrs(&assemble_prog(src).unwrap().instructions)
    }

    fn compile_err_instrs(program: &[Instruction]) -> CompileError {
        match NativeJit::new().compile_block(program, 0) {
            Ok(_) => panic!("expected compile error"),
            Err(e) => e,
        }
    }

    fn f64_reg(index: RegIndex) -> Reg<FloatRegT> {
        Reg { index, width: 64 }
    }

    fn f32_reg(index: RegIndex) -> Reg<FloatRegT> {
        Reg { index, width: 32 }
    }

    // compile block at pc 0, run it once with given f64 regs set
    fn run_f64(src: &str, regs: &[(RegIndex, f64)]) -> (BlockExit, RegState<SafeAddress>) {
        run_on(&assemble_prog(src).unwrap().instructions, |state| {
            for &(i, v) in regs {
                state.store_prim(f64_reg(i), v);
            }
        })
    }

    fn readf<T: CastSingleFloat>(state: &RegState<SafeAddress>, r: Reg<FloatRegT>) -> T {
        read_float(&RegOrConstant::Reg(r), state)
    }

    fn getf(state: &RegState<SafeAddress>, index: RegIndex) -> f64 {
        readf(state, f64_reg(index))
    }

    fn read(state: &RegState<SafeAddress>, reg: Reg<UnsignedRegT>) -> u64 {
        read_uint::<u64, _>(&RegOrConstant::Reg(reg), state).unwrap_or_default()
    }

    fn get(state: &RegState<SafeAddress>, index: RegIndex) -> u64 {
        read(state, u64_reg(index))
    }

    const DIV_PROG: &str = "
        mov u64:3, #1
        div u64:2, u64:0, u64:1
        mov u64:4, #9
    ";

    #[test]
    fn div_runs_natively_when_divisor_nonzero() {
        let (exit, state) = run_block(DIV_PROG, &[(0, 17), (1, 5)]);
        assert_eq!(exit, BlockExit::Goto(3));
        assert_eq!([get(&state, 2), get(&state, 3), get(&state, 4)], [3, 1, 9]);
    }

    #[test]
    fn div_by_zero_register_hands_instruction_to_interpreter() {
        let (exit, state) = run_block(DIV_PROG, &[(0, 17), (1, 0), (2, 5), (4, 7)]);
        assert_eq!(exit, BlockExit::Interpret(1));
        // effects before the div are kept, registers written after it are untouched
        assert_eq!([get(&state, 2), get(&state, 3), get(&state, 4)], [5, 1, 7]);
    }

    #[test]
    fn mod_by_zero_register_hands_instruction_to_interpreter() {
        let (exit, _) = run_block("mod u64:2, u64:0, u64:1\n", &[(0, 17), (1, 0)]);
        assert_eq!(exit, BlockExit::Interpret(0));
    }

    #[test]
    fn div_by_zero_constant_hands_instruction_to_interpreter() {
        let (exit, state) = run_block("div u64:2, u64:0, #0\n", &[(0, 17), (2, 5)]);
        assert_eq!(exit, BlockExit::Interpret(0));
        assert_eq!(get(&state, 2), 5);
    }

    #[test]
    fn straight_line_unsigned_ops() {
        const PROG: &str = "
            mov u64:2, u64:0
            add u64:3, u64:0, u64:1
            sub u64:4, u64:0, u64:1
            mul u64:5, u64:0, #3
            and u64:6, u64:0, u64:1
            xor u64:7, u64:0, u64:1
            not u64:8, u64:1
            sgt u64:9, u64:0, u64:1
            slt u64:10, u64:0, #5
            mod u64:11, u64:0, #5
        ";
        let (exit, state) = run_block(PROG, &[(0, 12), (1, 10)]);
        assert_eq!(exit, BlockExit::Goto(10));
        let got: Vec<u64> = (2..=11).map(|i| get(&state, i)).collect();
        assert_eq!(got, [12, 22, 2, 36, 8, 6, !10, 1, 0, 2]);
    }

    #[test]
    fn jmp_exits_to_target() {
        let (exit, _) = run_block("jmp .END\nnop\n.END:\nnop\n", &[]);
        assert_eq!(exit, BlockExit::Goto(2));
    }

    #[test]
    fn jal_links_next_pc() {
        let (exit, state) = run_block("jal .END, n:0\nnop\n.END:\nnop\n", &[]);
        assert_eq!(exit, BlockExit::Goto(2));
        let link = read_iaddr(&RegOrConstant::Reg(Reg::<InstrRegT>::from_index(0)), &state);
        assert_eq!(link.pc(), 1);
    }

    #[test]
    fn dbg_calls_host_and_continues() {
        let (exit, _) = run_block("dbg u64:0\n", &[(0, 3)]);
        assert_eq!(exit, BlockExit::Goto(1));
    }

    #[test]
    fn non_native_width_is_unsupported() {
        let e = compile_err("add u65:0, u65:0, #1\n");
        assert!(
            matches!(e, CompileError::Unsupported { pc: 0, .. }),
            "{e:?}"
        );
    }

    #[test]
    fn float_arith_f64() {
        const PROG: &str = "
            add f64:2, f64:0, f64:1
            sub f64:3, f64:0, f64:1
            mul f64:4, f64:0, f64:1
            div f64:5, f64:0, f64:1
            mod f64:6, f64:0, f64:1
            add f64:7, f64:0, #0.25
        ";
        let (exit, state) = run_f64(PROG, &[(0, -7.5), (1, 2.0)]);
        assert_eq!(exit, BlockExit::Goto(6));
        let got: Vec<f64> = (2..=7).map(|i| getf(&state, i)).collect();
        assert_eq!(
            got,
            [-5.5, -9.5, -15.0, -3.75, (-7.5f64).rem_euclid(2.0), -7.25]
        );
    }

    #[test]
    fn float_f32_narrows_constants_and_mods() {
        let prog = assemble_prog("add f32:1, f32:0, #0.1\nmod f32:2, f32:0, #2.0\n").unwrap();
        let (_, state) = run_on(&prog.instructions, |s| s.store_prim(f32_reg(0), -7.5f32));
        assert_eq!(readf::<f32>(&state, f32_reg(1)), -7.5f32 + 0.1f64 as f32);
        assert_eq!(readf::<f32>(&state, f32_reg(2)), (-7.5f32).rem_euclid(2.0));
    }

    #[test]
    fn float_mov_turns_neg_zero_positive() {
        let (_, state) = run_f64("mov f64:1, f64:0\n", &[(0, -0.0)]);
        assert_eq!(getf(&state, 1).to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn casts_match_rust_as() {
        use crate::vm::helper::read_int;
        use umc_model::reg_model::SignedRegT;
        let s = |index, width| Reg::<SignedRegT> { index, width };
        let u = unsigned;
        const PROG: &str = "
            cast i64:1, f32:0
            cast u32:2, f64:0
            cast i32:3, f64:1
            cast f64:2, i32:4
            cast f32:3, i64:5
            cast f64:4, f32:0
            cast i64:6, i32:4
            cast u64:7, u32:8
            cast u32:9, u64:10
            cast u32:11, i32:4
            cast u64:12, f32:5
        ";
        let prog = assemble_prog(PROG).unwrap();
        let (exit, st) = run_on(&prog.instructions, |st| {
            st.store_prim(f32_reg(0), -3.75f32);
            st.store_prim(f64_reg(0), 1e20f64);
            st.store_prim(f64_reg(1), f64::NAN);
            st.store_prim(s(4, 32), -7i32);
            st.store_prim(s(5, 64), i64::MAX);
            st.store_prim(u(8, 32), u32::MAX);
            st.store_prim(u(10, 64), 0x1_2345_6789u64);
            st.store_prim(f32_reg(5), -1.0f32);
        });
        assert_eq!(exit, BlockExit::Goto(11));
        let int = |r| read_int::<i64, _>(&RegOrConstant::Reg(r), &st);
        assert_eq!(int(s(1, 64)), -3); // trunc toward zero
        assert_eq!(read(&st, u(2, 32)), u32::MAX as u64); // saturates
        assert_eq!(int(s(3, 32)), 0); // NaN
        assert_eq!(readf::<f64>(&st, f64_reg(2)), -7.0);
        assert_eq!(readf::<f32>(&st, f32_reg(3)), i64::MAX as f32);
        assert_eq!(readf::<f64>(&st, f64_reg(4)), -3.75);
        assert_eq!(int(s(6, 64)), -7); // sign extends
        assert_eq!(read(&st, u(7, 64)), u32::MAX as u64); // zero extends
        assert_eq!(read(&st, u(9, 32)), 0x2345_6789); // truncates
        assert_eq!(read(&st, u(11, 32)), -7i32 as u32 as u64); // bitwise
        assert_eq!(read(&st, u(12, 64)), 0); // negative float to unsigned
    }

    #[test]
    fn float_div_by_zero_is_inf() {
        let (exit, state) = run_f64("div f64:1, f64:0, #0.0\n", &[(0, 1.0)]);
        assert_eq!(exit, BlockExit::Goto(1));
        assert_eq!(getf(&state, 1), f64::INFINITY);
    }

    // Only works on x86, NAN is greater than 0 on arm
    #[test]
    #[cfg(target_arch = "x86_64")]
    fn float_compare() {
        const PROG: &str = "
            seq u64:2, f64:0, f64:1
            sgt u64:3, f64:0, f64:1
            sge u64:4, f64:0, f64:1
            slt u64:5, f64:0, f64:1
            sle u64:6, f64:0, f64:1
        ";
        let flags = |a, b| {
            let (_, state) = run_f64(PROG, &[(0, a), (1, b)]);
            (2..=6).map(|i| get(&state, i)).collect::<Vec<_>>()
        };
        assert_eq!(flags(1.0, 2.0), [0, 0, 0, 1, 1]);
        assert_eq!(flags(1.0, f64::NAN), [0, 0, 0, 0, 0]);
    }

    #[test]
    fn float_bitwise_and_mixed_widths_are_unsupported() {
        let and = Instruction::And(AnyConsistentNumOp::Float(ConsistentOp::Single(
            f64_reg(0),
            RegOrConstant::Reg(f64_reg(1)),
            RegOrConstant::Const(1.0),
        )));
        let mixed = Instruction::Add(AddParams::Float(ConsistentOp::Single(
            f64_reg(0),
            RegOrConstant::Reg(f32_reg(1)),
            RegOrConstant::Const(1.0),
        )));
        for instr in [and, mixed] {
            let e = compile_err_instrs(&[instr]);
            assert!(
                matches!(e, CompileError::Unsupported { pc: 0, .. }),
                "{e:?}"
            );
        }
    }
}
