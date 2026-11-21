use crate::vm::compiler::emit::*;

mod float;
mod int;
#[cfg(test)]
mod tests;

use dynasmrt::{
    DynamicLabel, DynasmApi, DynasmLabelApi, VecAssembler, aarch64::Aarch64Relocation, dynasm,
};

// regs are plain numbers, width pitched per instruction with W(n) / X(n)
const FRAME_BASE: u8 = 19; // callee-saved, survives host calls (rbx on x86)
const MEM_BASE: u8 = 20; // also callee-saved (r12 on x86)
const SCRATCH: u8 = 16; // IP0: also holds host fn address for blr
const SCRATCH2: u8 = 17; // IP1: slot offsets that dont fit in ldr/str immediate
const STAGE: [u8; 2] = [9, 10]; // caller-saved
// AAPCS64 is the same on linux/windows so no shadow space or different call args needed
const CALL_ARGS: [u8; 4] = [0, 1, 2, 3];

// T0 must be x0 so that host call lands in x0 with no extra mov
pub(super) fn reg(t: Tmp) -> u8 {
    match t {
        Tmp::T0 => 0,
        Tmp::T1 => 1,
        Tmp::T2 => 2,
    }
}

pub(super) fn freg(t: FTmp) -> u8 {
    match t {
        FTmp::F0 => 0,
        FTmp::F1 => 1,
        FTmp::F2 => 2,
    }
}

// macro to save repeating `.arch aarch64` in every dynasm! call
macro_rules! arm {
    ($ops:expr; $($t:tt)*) => { dynasm!($ops ; .arch aarch64 ; $($t)*) };
}
pub(super) use arm;

struct LabelState {
    dl: DynamicLabel,
    bound: bool,
    referenced: bool,
}

pub struct ArmEmitter {
    ops: VecAssembler<Aarch64Relocation>,
    labels: Vec<LabelState>,
    error: Option<EmitError>,
}

fn set_error(slot: &mut Option<EmitError>, e: EmitError) {
    if slot.is_none() {
        *slot = Some(e);
    }
}

// dynasm won't split a 64bit immediate so movz + one movk per non-zero 16bit chunk
fn mov_imm64(ops: &mut VecAssembler<Aarch64Relocation>, rd: u8, imm: u64) {
    arm!(ops; movz X(rd), (imm & 0xFFFF) as u32);
    let (h1, h2, h3) = ((imm >> 16) & 0xFFFF, (imm >> 32) & 0xFFFF, imm >> 48);
    if h1 != 0 {
        arm!(ops; movk X(rd), #h1 as u32, LSL 16 );
    }
    if h2 != 0 {
        arm!(ops; movk X(rd), #h2 as u32, LSL 32);
    }
    if h3 != 0 {
        arm!(ops; movk X(rd), #h3 as u32, LSL 48);
    }
}

impl CoreEmitter for ArmEmitter {
    const ARCH: &'static str = "aarch64";
    fn new() -> Self {
        Self {
            ops: VecAssembler::new(0),
            labels: Vec::new(),
            error: None,
        }
    }

    fn new_label(&mut self) -> Label {
        let id = self.labels.len() as u32;
        let dl = self.ops.new_dynamic_label();
        self.labels.push(LabelState {
            dl,
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
        arm!(self.ops; =>state.dl);
    }

    fn prologue(&mut self, _layout: &FrameLayout) {
        arm!(self.ops
            ; stp x29, x30, [sp, #-32]!
            ; mov x29, sp
            ; stp X(FRAME_BASE), X(MEM_BASE), [sp, #16]
            ; mov X(FRAME_BASE), x0
            ; mov X(MEM_BASE), x1
        );
    }
    fn ret_exit(&mut self, exit_id: u32) {
        mov_imm64(&mut self.ops, 0, exit_id as u64);
        arm!(self.ops
            ; ldp X(FRAME_BASE), X(MEM_BASE), [sp, #16]
            ; ldp x29, x30, [sp], #32
            ; ret
        );
    }

    fn call_host(&mut self, f: usize, args: &[Arg]) {
        debug_assert!(args.len() <= CALL_ARGS.len());
        // copy in Tmp args first
        let mut staged = 0;
        let mut from = [None; 4];
        for (i, a) in args.iter().enumerate() {
            if let Arg::Tmp(t) = *a {
                debug_assert!(staged < STAGE.len(), "at most 2 Tmp args");
                arm!(self.ops
                    ; mov X(STAGE[staged]), X(reg(t))
                );
                from[i] = Some(STAGE[staged]);
                staged += 1;
            }
        }
        for (i, a) in args.iter().enumerate() {
            match (*a, from[i]) {
                (_, Some(s)) => arm!(self.ops ; mov X(CALL_ARGS[i as usize]), X(s)),
                (Arg::Imm(v), None) => mov_imm64(&mut self.ops, CALL_ARGS[i as usize], v),
                (Arg::Frame, None) => arm!(self.ops ; mov X(CALL_ARGS[i as usize]), X(FRAME_BASE)),
                (Arg::Mem, None) => arm!(self.ops ; mov X(CALL_ARGS[i as usize]), X(MEM_BASE)),
                (Arg::Tmp(_), None) => unreachable!(),
            }
        }
        mov_imm64(&mut self.ops, SCRATCH, f as u64);
        arm!(self.ops ; blr X(SCRATCH));
    }

    fn jump(&mut self, target: Label) {
        let st = &mut self.labels[target.0 as usize];
        st.referenced = true;
        let dl = st.dl;

        arm!(self.ops ; b =>dl)
    }

    fn finish(mut self) -> Result<Vec<u8>, EmitError> {
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        if let Some(i) = self.labels.iter().position(|s| s.referenced && !s.bound) {
            return Err(EmitError::UnboundLabel(Label(i as u32)));
        }
        self.ops
            .finalize()
            .map_err(|e| EmitError::Asm(e.to_string()))
    }
}
