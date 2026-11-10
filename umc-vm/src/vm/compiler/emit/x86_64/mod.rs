use crate::vm::compiler::emit::*;

mod float;
mod int;
#[cfg(test)]
mod tests;

use iced_x86::{Code, Instruction, Register, code_asm::*};

const FRAME_BASE: AsmRegister64 = rbx;
const SCRATCH32: AsmRegister32 = r11d;
const SCRATCH64: AsmRegister64 = r11;
// staging registers for host call args, using since doesn't overlap with either ABI call_args
const STAGE: [AsmRegister64; 2] = [r10, r11];
const MEM_BASE: AsmRegister64 = r12;

#[cfg(windows)]
mod sys {
    use super::*;
    pub(super) const ARGS: [AsmRegister64; 2] = [rcx, rdx];
    pub(super) const CALL_ARGS: [AsmRegister64; 4] = [rcx, rdx, r8, r9];
    pub(super) const SHADOW: i32 = 32;
}

#[cfg(unix)]
mod sys {
    use super::*;
    pub(super) const ARGS: [AsmRegister64; 2] = [rdi, rsi];
    pub(super) const CALL_ARGS: [AsmRegister64; 4] = [rdi, rsi, rdx, rcx];
    pub(super) const SHADOW: i32 = 0;
}

use sys::*;

pub(super) fn r64(t: Tmp) -> AsmRegister64 {
    match t {
        Tmp::T0 => rax,
        Tmp::T1 => rcx,
        Tmp::T2 => rdx,
    }
}

pub(super) fn r32(t: Tmp) -> AsmRegister32 {
    match t {
        Tmp::T0 => eax,
        Tmp::T1 => ecx,
        Tmp::T2 => edx,
    }
}

pub(super) fn reg16(t: Tmp) -> AsmRegister16 {
    match t {
        Tmp::T0 => ax,
        Tmp::T1 => cx,
        Tmp::T2 => dx,
    }
}

pub(super) fn reg8(t: Tmp) -> AsmRegister8 {
    match t {
        Tmp::T0 => al,
        Tmp::T1 => cl,
        Tmp::T2 => dl,
    }
}

pub(super) fn xmm(t: FTmp) -> AsmRegisterXmm {
    match t {
        FTmp::F0 => xmm0,
        FTmp::F1 => xmm1,
        FTmp::F2 => xmm2,
    }
}

// what emitter knows about label
struct LabelState {
    iced: CodeLabel,
    bound: bool,
    referenced: bool, // set by jump/branch_cmp
}

pub struct X86Emitter {
    asm: CodeAssembler,
    labels: Vec<LabelState>,
    error: Option<EmitError>,
}

// keep only first error
fn set_error(slot: &mut Option<EmitError>, e: EmitError) {
    if slot.is_none() {
        *slot = Some(e);
    }
}

// bridge iced result to trait methods
fn record(slot: &mut Option<EmitError>, r: Result<(), IcedError>) {
    if let Err(e) = r {
        set_error(slot, EmitError::Asm(e.to_string()));
    }
}

impl CoreEmitter for X86Emitter {
    const ARCH: &'static str = "x86_64";
    fn new() -> Self {
        Self {
            asm: CodeAssembler::new(64).expect("64 always valid bitness"),
            labels: Vec::new(),
            error: None,
        }
    }

    fn new_label(&mut self) -> Label {
        let id = self.labels.len() as u32;
        let iced = self.asm.create_label();
        self.labels.push(LabelState {
            iced,
            bound: false,
            referenced: false,
        });
        Label(id)
    }
    fn bind(&mut self, l: Label) {
        let state = &mut self.labels[l.0 as usize];
        if state.bound {
            set_error(&mut self.error, EmitError::RebindLabel(l));
            return;
        }
        state.bound = true;
        record(&mut self.error, self.asm.set_label(&mut state.iced));
    }

    /// ABI aware methods
    fn prologue(&mut self, _layout: &FrameLayout) {
        record(&mut self.error, self.asm.push(FRAME_BASE));
        record(&mut self.error, self.asm.push(MEM_BASE));
        // entry rsp is 8 mod 16, two pushes leave it there, realign so host calls see 16
        record(&mut self.error, self.asm.sub(rsp, 8));
        record(&mut self.error, self.asm.mov(FRAME_BASE, ARGS[0]));
        record(&mut self.error, self.asm.mov(MEM_BASE, ARGS[1]));
    }
    fn ret_exit(&mut self, exit_id: u32) {
        // rax / eax for return values
        record(&mut self.error, self.asm.mov(eax, exit_id)); // CompiledFn returns u32 so mov into eax for same result ith short form
        record(&mut self.error, self.asm.add(rsp, 8));
        record(&mut self.error, self.asm.pop(MEM_BASE));
        record(&mut self.error, self.asm.pop(FRAME_BASE));
        record(&mut self.error, self.asm.ret());
    }

    fn call_host(&mut self, f: usize, args: &[Arg]) {
        debug_assert!(args.len() <= CALL_ARGS.len());
        // copy Tmp args in first
        let mut staged = 0;
        let mut from = [None; 4];
        for (i, a) in args.iter().enumerate() {
            if let Arg::Tmp(t) = *a {
                debug_assert!(staged < STAGE.len(), "at most 2 Tmp args");
                record(&mut self.error, self.asm.mov(STAGE[staged], r64(t)));
                from[i] = Some(STAGE[staged]);
                staged += 1;
            }
        }
        for (i, a) in args.iter().enumerate() {
            let r = match (*a, from[i]) {
                (_, Some(s)) => self.asm.mov(CALL_ARGS[i], s),
                (Arg::Imm(v), None) => self.asm.mov(CALL_ARGS[i], v),
                (Arg::Frame, None) => self.asm.mov(CALL_ARGS[i], FRAME_BASE),
                (Arg::Mem, None) => self.asm.mov(CALL_ARGS[i], MEM_BASE),
                (Arg::Tmp(_), None) => unreachable!(),
            };
            record(&mut self.error, r);
        }
        // Win64 ABI requires 32 bytes of shadow space before calls so push then pop it after call
        // rsp is 16-aligned, 8 mod 16 on entry, plus prologues push rbx
        if SHADOW > 0 {
            record(&mut self.error, self.asm.sub(rsp, SHADOW));
        }
        record(&mut self.error, self.asm.mov(SCRATCH64, f as u64));
        record(&mut self.error, self.asm.call(SCRATCH64));
        if SHADOW > 0 {
            record(&mut self.error, self.asm.add(rsp, SHADOW));
        }
    }

    fn jump(&mut self, target: Label) {
        let st = &mut self.labels[target.0 as usize];
        st.referenced = true;
        let l = st.iced;

        let r = self.asm.jmp(l);
        record(&mut self.error, r);
    }

    fn finish(mut self) -> Result<Vec<u8>, EmitError> {
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        if let Some(i) = self.labels.iter().position(|s| s.referenced && !s.bound) {
            return Err(EmitError::UnboundLabel(Label(i as u32)));
        }
        self.asm
            .assemble(0)
            .map_err(|e| EmitError::Asm(e.to_string()))
    }
}
