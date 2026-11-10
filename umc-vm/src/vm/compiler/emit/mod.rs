mod aarch64;
mod x86_64;

use crate::vm::RegWidth;
use crate::vm::compiler::{FrameLayout, FrameSlot};

#[cfg(target_arch = "aarch64")]
pub type NativeEmitter = aarch64::ArmEmitter;

#[cfg(target_arch = "x86_64")]
pub type NativeEmitter = x86_64::X86Emitter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tmp {
    T0,
    T1,
    T2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FTmp {
    F0,
    F1,
    F2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpWidth {
    W32,
    W64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Src {
    Tmp(Tmp),
    Imm(u64),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FAluOp {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Mov,
    Not,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CmpCond {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signedness {
    Unsigned,
    Signed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Label(pub u32);

#[derive(Debug, PartialEq)]
pub enum EmitError {
    Unencodable { what: &'static str, value: u64 },
    UnboundLabel(Label),
    RebindLabel(Label),
    Asm(String), // for Iced errors that don't map easily
}

// argument to host call
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arg {
    Tmp(Tmp),
    Imm(u64),
    Frame, // frame base
    Mem,   // memory manager passed to block
}

/// block structure, ABI, host calls
pub trait CoreEmitter: Sized {
    const ARCH: &'static str;
    fn new() -> Self;

    fn new_label(&mut self) -> Label;
    fn bind(&mut self, l: Label);

    /// ABI aware methods
    fn prologue(&mut self, layout: &FrameLayout);
    fn ret_exit(&mut self, exit_id: u32);

    // call extern "C" fn at f, integer retrun lands in T0
    fn call_host(&mut self, f: usize, args: &[Arg]);

    fn jump(&mut self, target: Label);

    fn finish(self) -> Result<Vec<u8>, EmitError>;
}

/// integer slots, alu, cmps on Tmp
pub trait IntEmitter {
    fn load_slot(&mut self, dst: Tmp, slot: FrameSlot, w: OpWidth);
    fn store_slot(&mut self, slot: FrameSlot, src: Tmp, w: OpWidth);

    fn mov_imm(&mut self, dst: Tmp, imm: u64, w: OpWidth);
    fn mov_rr(&mut self, dst: Tmp, src: Tmp, w: OpWidth);
    fn alu_rr(&mut self, op: AluOp, dst: Tmp, src: Tmp, w: OpWidth, sign: Signedness);
    fn neg(&mut self, dst: Tmp, w: OpWidth);

    fn alu(&mut self, op: AluOp, dst: Tmp, lhs: Tmp, rhs: Src, w: OpWidth, sign: Signedness);
    fn unop(&mut self, op: UnOp, dst: Tmp, src: Src, w: OpWidth);

    fn lshift(&mut self, dst: Tmp, size: Src, w: OpWidth);
    fn l_rshift(&mut self, dst: Tmp, size: Src, w: OpWidth);
    fn a_rshift(&mut self, dst: Tmp, size: Src, w: OpWidth);

    /// UMC wrap at declared width - noop when bits already natural for withd
    fn truncate_unsigned(&mut self, dst: Tmp, bits: RegWidth, w: OpWidth);
    fn sign_extend(&mut self, dst: Tmp, bits: RegWidth, w: OpWidth);

    fn set_cmp(&mut self, dst: Tmp, cond: CmpCond, lhs: Tmp, rhs: Src, w: OpWidth, s: Signedness);
    fn branch_cmp(
        &mut self,
        cond: CmpCond,
        lhs: Tmp,
        rhs: Src,
        w: OpWidth,
        s: Signedness,
        target: Label,
    );
}

pub trait FloatEmitter {
    fn fload_slot(&mut self, dst: FTmp, slot: FrameSlot, w: OpWidth);
    fn fstore_slot(&mut self, slot: FrameSlot, src: FTmp, w: OpWidth);

    // dst = sqrt(dst)
    fn fsqrt(&mut self, dst: FTmp, w: OpWidth);

    // raw IEEE bits, f32 in low 32
    fn fmov_imm(&mut self, dst: FTmp, bits: u64, w: OpWidth);
    // dst = dst op rhs
    fn falu(&mut self, op: FAluOp, dst: FTmp, rhs: FTmp, w: OpWidth);
    fn fset_cmp(&mut self, dst: Tmp, cond: CmpCond, lhs: FTmp, rhs: FTmp, w: OpWidth);
}

/// everything lowering needs from backend
pub trait Emitter: CoreEmitter + IntEmitter + FloatEmitter {}
impl<T: CoreEmitter + IntEmitter + FloatEmitter> Emitter for T {}
