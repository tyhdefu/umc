use super::*;

use yaxpeax_arch::{Decoder, U8Reader};
use yaxpeax_arm::armv8::a64::InstDecoder;

/// Decodes bytes and fails if invalid or trailing bytes don't form whole instruction
fn disasm(bytes: &[u8]) -> Vec<String> {
    assert_eq!(bytes.len() % 4, 0, "trailing bytes");
    let decoder = InstDecoder::default();
    let mut reader = U8Reader::new(bytes);
    (0..bytes.len() / 4)
        .map(|i| match decoder.decode(&mut reader) {
            Ok(inst) => inst.to_string(),
            Err(e) => panic!(
                "invalid instruction at {:#x}: {e:?} in {:02X?}",
                i * 4,
                bytes
            ),
        })
        .collect()
}

#[test]
fn empty_emitter_finishes() {
    assert_eq!(ArmEmitter::new().finish(), Ok(vec![]));
}

#[test]
fn rebinding_a_label_is_an_error() {
    let mut e = ArmEmitter::new();
    let l = e.new_label();
    e.bind(l);
    e.bind(l);
    assert_eq!(e.finish(), Err(EmitError::RebindLabel(l)));
}

// unlike iced, dynasm is fine with a label bound past the last instruction
#[test]
fn label_at_end_of_code_is_allowed() {
    let mut e = ArmEmitter::new();
    let l = e.new_label();
    e.bind(l);
    assert_eq!(e.finish(), Ok(vec![]));
}

#[test]
fn disasm_round_trip() {
    let mut e = ArmEmitter::new();
    arm!(e.ops
        ; mov X(FRAME_BASE), x0
        ; blr X(SCRATCH)
        ; ret
    );
    let text = disasm(&e.finish().unwrap());
    assert_eq!(text, ["mov x19, x0", "blr x16", "ret"])
}

#[test]
fn mov_imm_emits_movz_then_movk_per_nonzero_chunk() {
    let mut e = ArmEmitter::new();
    e.mov_imm(Tmp::T0, 7, OpWidth::W64);
    e.mov_imm(Tmp::T1, u64::MAX, OpWidth::W32); // W32 masks to low half
    e.mov_imm(Tmp::T2, 0x1_0000_0000, OpWidth::W64); // zero chunks skipped
    assert_eq!(
        disasm(&e.finish().unwrap()),
        [
            "mov x0, #0x7",
            "mov x1, #0xffff",
            "movk x1, #0xffff, lsl #16",
            "mov x2, #0x0",
            "movk x2, #0x1, lsl #32",
        ]
    );
}

#[test]
fn alu_sub_with_dst_aliasing_rhs_is_one_instruction() {
    let mut e = ArmEmitter::new();
    // T0 = T1 - T0
    e.alu(
        AluOp::Sub,
        Tmp::T0,
        Tmp::T1,
        Src::Tmp(Tmp::T0),
        OpWidth::W64,
        Signedness::Unsigned,
    );
    assert_eq!(disasm(&e.finish().unwrap()), ["sub x0, x1, x0"]);
}

#[test]
fn alu_small_imm_is_encoded_inline() {
    let mut e = ArmEmitter::new();
    e.alu(
        AluOp::Add,
        Tmp::T0,
        Tmp::T1,
        Src::Imm(4095),
        OpWidth::W32,
        Signedness::Unsigned,
    );
    assert_eq!(disasm(&e.finish().unwrap()), ["add w0, w1, #0xfff"]);
}

#[test]
fn alu_wide_imm_goes_through_scratch() {
    let mut e = ArmEmitter::new();
    e.alu(
        AluOp::Add,
        Tmp::T0,
        Tmp::T1,
        Src::Imm(0x1_0000_0000),
        OpWidth::W64,
        Signedness::Unsigned,
    );
    assert_eq!(
        disasm(&e.finish().unwrap()),
        [
            "mov x16, #0x0",
            "movk x16, #0x1, lsl #32",
            "add x0, x1, x16"
        ]
    );
}

#[test]
fn not_of_imm_is_folded() {
    let mut e = ArmEmitter::new();
    e.unop(UnOp::Not, Tmp::T0, Src::Imm(0xF0), OpWidth::W32);
    assert_eq!(
        disasm(&e.finish().unwrap()),
        ["mov x0, #0xff0f", "movk x0, #0xffff, lsl #16"]
    );
}

#[test]
fn truncate_and_sign_extend_use_bitfield_extract() {
    let mut e = ArmEmitter::new();
    e.truncate_unsigned(Tmp::T0, 8, OpWidth::W64);
    e.truncate_unsigned(Tmp::T0, 32, OpWidth::W32); // natural width -> nothing
    e.sign_extend(Tmp::T1, 16, OpWidth::W32);
    assert_eq!(
        disasm(&e.finish().unwrap()),
        // sbfx #0, #16 disassembles as its sxth alias
        ["ubfx x0, x0, #0x0, #0x8", "sxth w1, w1"]
    );
}

#[test]
fn set_cmp_unsigned_uses_lo() {
    let mut e = ArmEmitter::new();
    e.set_cmp(
        Tmp::T2,
        CmpCond::Lt,
        Tmp::T0,
        Src::Imm(5),
        OpWidth::W64,
        Signedness::Unsigned,
    );
    assert_eq!(
        disasm(&e.finish().unwrap()),
        ["cmp x0, #0x5", "cset w2, lo"]
    );
}

#[test]
fn branch_cmp_against_zero_uses_cbz() {
    let mut e = ArmEmitter::new();
    let l = e.new_label();
    e.bind(l);
    e.branch_cmp(
        CmpCond::Eq,
        Tmp::T1,
        Src::Imm(0),
        OpWidth::W64,
        Signedness::Unsigned,
        l,
    );
    assert_eq!(disasm(&e.finish().unwrap()), ["cbz x1, $+0x0"]);
}

#[test]
fn branch_to_unbound_label_is_an_error() {
    let mut e = ArmEmitter::new();
    let l = e.new_label();
    e.jump(l);
    assert_eq!(e.finish(), Err(EmitError::UnboundLabel(l)));
}

// everything below runs emitted code, so only on an aarch64 host
#[cfg(target_arch = "aarch64")]
mod exec {
    use super::*;
    use crate::vm::compiler::exec_mem::ExecPage;

    fn run(e: ArmEmitter, frame: &mut [u64]) -> u32 {
        let page = ExecPage::commit(&e.finish().unwrap()).unwrap();
        unsafe { page.as_fn()(frame.as_mut_ptr(), std::ptr::null_mut()) }
    }

    // slot2 = slot0 - slot1; exit 2 if slot0 < slot1 (signed) else exit 1
    fn sub_and_branch() -> ArmEmitter {
        let mut e = ArmEmitter::new();
        let less = e.new_label();
        e.prologue(&FrameLayout::new());
        e.load_slot(Tmp::T0, FrameSlot::new(0), OpWidth::W64);
        e.load_slot(Tmp::T1, FrameSlot::new(1), OpWidth::W64);
        e.alu(
            AluOp::Sub,
            Tmp::T2,
            Tmp::T0,
            Src::Tmp(Tmp::T1),
            OpWidth::W64,
            Signedness::Unsigned,
        );
        e.store_slot(FrameSlot::new(2), Tmp::T2, OpWidth::W64);
        e.branch_cmp(
            CmpCond::Lt,
            Tmp::T0,
            Src::Tmp(Tmp::T1),
            OpWidth::W64,
            Signedness::Signed,
            less,
        );
        e.ret_exit(1);
        e.bind(less);
        e.ret_exit(2);
        e
    }

    #[test]
    fn emitted_block_runs_on_frame() {
        let mut frame = [10u64, 3, 0];
        assert_eq!((run(sub_and_branch(), &mut frame), frame), (1, [10, 3, 7]));

        let mut frame = [3u64, 10, 0];
        assert_eq!(
            (run(sub_and_branch(), &mut frame), frame),
            (2, [3, 10, (-7i64) as u64])
        );
    }

    // run dst = slot0 op rhs for each (op, rhs), results stored from slot 2 onwards
    fn run_div_mod(w: OpWidth, ops: &[(AluOp, Src)], frame: &mut [u64]) {
        let mut e = ArmEmitter::new();
        e.prologue(&FrameLayout::new());
        for (i, (op, rhs)) in ops.iter().enumerate() {
            e.load_slot(Tmp::T0, FrameSlot::new(0), w);
            e.load_slot(Tmp::T1, FrameSlot::new(1), w);
            e.alu(*op, Tmp::T0, Tmp::T0, *rhs, w, Signedness::Unsigned);
            e.store_slot(FrameSlot::new(2 + i as u16), Tmp::T0, w);
        }
        e.ret_exit(0);
        run(e, frame);
    }

    const DIV_MOD_OPS: [(AluOp, Src); 4] = [
        (AluOp::Div, Src::Tmp(Tmp::T1)),
        (AluOp::Mod, Src::Tmp(Tmp::T1)),
        (AluOp::Div, Src::Imm(7)),
        (AluOp::Mod, Src::Imm(7)),
    ];

    #[test]
    fn div_mod_64() {
        let mut frame = [100u64, 3, 0, 0, 0, 0];
        run_div_mod(OpWidth::W64, &DIV_MOD_OPS, &mut frame);
        assert_eq!(frame, [100, 3, 33, 1, 14, 2]);
    }

    #[test]
    fn div_mod_32() {
        let mut frame = [100u64, 3, 0, 0, 0, 0];
        run_div_mod(OpWidth::W32, &DIV_MOD_OPS, &mut frame);
        assert_eq!(frame, [100, 3, 33, 1, 14, 2]);
    }

    // mod keeps the quotient in SCRATCH2, so dst aliasing the divisor must still work
    #[test]
    fn div_mod_dst_aliasing_divisor() {
        let mut e = ArmEmitter::new();
        e.prologue(&FrameLayout::new());
        // slot2 = slot0 / T0 where T0 = slot1
        e.load_slot(Tmp::T1, FrameSlot::new(0), OpWidth::W64);
        e.load_slot(Tmp::T0, FrameSlot::new(1), OpWidth::W64);
        e.alu(
            AluOp::Div,
            Tmp::T0,
            Tmp::T1,
            Src::Tmp(Tmp::T0),
            OpWidth::W64,
            Signedness::Unsigned,
        );
        e.store_slot(FrameSlot::new(2), Tmp::T0, OpWidth::W64);
        // slot3 = slot0 % T0 where T0 = slot1
        e.load_slot(Tmp::T1, FrameSlot::new(0), OpWidth::W64);
        e.load_slot(Tmp::T0, FrameSlot::new(1), OpWidth::W64);
        e.alu(
            AluOp::Mod,
            Tmp::T0,
            Tmp::T1,
            Src::Tmp(Tmp::T0),
            OpWidth::W64,
            Signedness::Unsigned,
        );
        e.store_slot(FrameSlot::new(3), Tmp::T0, OpWidth::W64);
        e.ret_exit(0);

        let mut frame = [100u64, 3, 0, 0];
        run(e, &mut frame);
        assert_eq!(frame, [100, 3, 33, 1]);
    }

    extern "C" fn host_sub(a: u64, b: u64) -> u64 {
        a.wrapping_sub(b)
    }

    // without staging, x0 <- x1 would clobber T0 (x0) before it's moved into x1
    #[test]
    fn call_host_stages_tmp_args() {
        let mut e = ArmEmitter::new();
        e.prologue(&FrameLayout::new());
        e.load_slot(Tmp::T0, FrameSlot::new(0), OpWidth::W64);
        e.load_slot(Tmp::T1, FrameSlot::new(1), OpWidth::W64);
        e.call_host(
            host_sub as *const () as usize,
            &[Arg::Tmp(Tmp::T1), Arg::Tmp(Tmp::T0)],
        );
        e.store_slot(FrameSlot::new(2), Tmp::T0, OpWidth::W64);
        e.ret_exit(0);
        let mut frame = [10u64, 3, 0];
        run(e, &mut frame);
        assert_eq!(frame[2], 3u64.wrapping_sub(10));
    }
}
