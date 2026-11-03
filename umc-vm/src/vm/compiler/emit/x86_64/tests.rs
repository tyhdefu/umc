use super::*;
use iced_x86::{Decoder, DecoderOptions, Formatter, GasFormatter, Instruction};

/// Decodses bytes and fails if invalid or trailing bytes don't form whole instruction
fn disasm(bytes: &[u8]) -> Vec<String> {
    let mut decoder = Decoder::with_ip(64, bytes, 0, DecoderOptions::NONE);
    let mut formatter = GasFormatter::new();
    let mut instr = Instruction::default();
    let mut out = Vec::new();
    while decoder.can_decode() {
        decoder.decode_out(&mut instr);
        assert!(
            !instr.is_invalid(),
            "invalid instruction at {:#x}: {:02X?}",
            instr.ip(),
            bytes
        );
        let mut text = String::new();
        formatter.format(&instr, &mut text);
        out.push(text);
    }
    assert_eq!(decoder.position(), bytes.len(), "undecoded trailing bytes");
    out
}

#[test]
fn empty_emitter_finishes() {
    assert_eq!(X86Emitter::new().finish(), Ok(vec![]));
}

#[test]
fn rebinding_a_label_is_an_error() {
    let mut e = X86Emitter::new();
    let l = e.new_label();
    e.bind(l);
    // iced requires every label to be followed by instruction
    record(&mut e.error, e.asm.ret());
    e.bind(l);
    record(&mut e.error, e.asm.ret());
    assert_eq!(e.finish(), Err(EmitError::RebindLabel(l)));
}

#[test]
fn label_at_end_of_code_is_rejected_by_iced() {
    let mut e = X86Emitter::new();
    let l = e.new_label();
    e.bind(l);
    assert!(matches!(e.finish(), Err(EmitError::Asm(m)) if m.contains("Unused label")));
}

#[test]
fn disasm_round_trip() {
    let mut e = X86Emitter::new();
    record(&mut e.error, e.asm.push(FRAME_BASE));
    record(&mut e.error, e.asm.mov(FRAME_BASE, rcx));
    record(&mut e.error, e.asm.ret());
    let text = disasm(&e.finish().unwrap());
    assert_eq!(text, ["push %rbx", "mov %rcx,%rbx", "ret"])
}

#[test]
fn mov_imm_picks_shortest_encoding() {
    let mut e = X86Emitter::new();
    e.mov_imm(Tmp::T0, 7, OpWidth::W64); // fits u32 -> zero-extending 32 bit mov
    e.mov_imm(Tmp::T1, u64::MAX, OpWidth::W64); // -1 -> sign-extended imm32
    e.mov_imm(Tmp::T2, 0x1_0000_0000, OpWidth::W64); // needs full imm64
    let bytes = e.finish().unwrap();
    assert_eq!(
        disasm(&bytes),
        [
            "mov $7,%eax",
            "mov $0xFFFFFFFFFFFFFFFF,%rcx",
            "movabs $0x100000000,%rdx"
        ]
    );
    // 5 + 7 + 10 bytes
    assert_eq!(bytes.len(), 22);
}

#[test]
fn alu_sub_with_dst_aliasing_rhs_negates_first() {
    let mut e = X86Emitter::new();
    // T0 = T1 - T0
    e.alu(
        AluOp::Sub,
        Tmp::T0,
        Tmp::T1,
        Src::Tmp(Tmp::T0),
        OpWidth::W64,
        Signedness::Unsigned,
    );
    assert_eq!(disasm(&e.finish().unwrap()), ["neg %rax", "add %rcx,%rax"]);
}

#[test]
fn alu_wide_imm_goes_through_scratch() {
    let mut e = X86Emitter::new();
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
        ["mov %rcx,%rax", "movabs $0x100000000,%r11", "add %r11,%rax"]
    );
}

#[test]
fn not_of_imm_is_folded() {
    let mut e = X86Emitter::new();
    e.unop(UnOp::Not, Tmp::T0, Src::Imm(0xF0), OpWidth::W32);
    assert_eq!(disasm(&e.finish().unwrap()), ["mov $0xFFFFFF0F,%eax"]);
}

#[test]
fn mov_imm_w32_truncates_sign_extended() {
    let mut e = X86Emitter::new();
    e.mov_imm(Tmp::T0, (-23814i64) as u64, OpWidth::W32); // sign-extended i32 const
    assert_eq!(disasm(&e.finish().unwrap()), ["mov $0xFFFFA2FA,%eax"]);
}

#[test]
fn truncate_and_sign_extend_use_shift_pairs() {
    let mut e = X86Emitter::new();
    e.truncate_unsigned(Tmp::T0, 8, OpWidth::W64);
    e.truncate_unsigned(Tmp::T0, 32, OpWidth::W32); // natural width -> nop
    e.sign_extend(Tmp::T1, 16, OpWidth::W32);
    assert_eq!(
        disasm(&e.finish().unwrap()),
        [
            "shl $0x38,%rax",
            "shr $0x38,%rax",
            "nop",
            "shl $0x10,%ecx",
            "sar $0x10,%ecx"
        ]
    );
}

#[test]
fn set_cmp_unsigned_uses_below() {
    let mut e = X86Emitter::new();
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
        ["cmp $5,%rax", "setb %dl", "movzbl %dl,%edx"]
    );
}

#[test]
fn branch_to_unbound_label_is_an_error() {
    let mut e = X86Emitter::new();
    let l = e.new_label();
    e.jump(l);
    assert_eq!(e.finish(), Err(EmitError::UnboundLabel(l)));
}

// everything below runs emitted code, so only on an x86_64 host
#[cfg(target_arch = "x86_64")]
mod exec {
    use super::*;

    // emit a block, commit it to exec memory and run it on real frame
    #[test]
    fn emitted_block_runs_on_frame() {
        use crate::vm::compiler::exec_mem::ExecPage;
        use rustc_hash::FxHashMap;

        let layout = FrameLayout {
            slots: Vec::new(),
            lookup: FxHashMap::default(),
        };

        // slot2 = slot0 - slot1; exit 2 if slot0 < slot1 (signed) else exit 1
        let mut e = X86Emitter::new();
        let less = e.new_label();
        e.prologue(&layout);
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

        let page = ExecPage::commit(&e.finish().unwrap()).unwrap();

        let mut frame = [10u64, 3, 0];
        let exit = unsafe { page.as_fn()(frame.as_mut_ptr(), std::ptr::null_mut()) };
        assert_eq!((exit, frame), (1, [10, 3, 7]));

        let mut frame = [3u64, 10, 0];
        let exit = unsafe { page.as_fn()(frame.as_mut_ptr(), std::ptr::null_mut()) };
        assert_eq!((exit, frame), (2, [3, 10, (-7i64) as u64]));
    }

    // run dst = slot0 op rhs for each (op, rhs) with rdx dirtied beforehand, results stored from slot 2 onwards
    fn run_div_mod(w: OpWidth, ops: &[(AluOp, Src)], frame: &mut [u64]) {
        use crate::vm::compiler::exec_mem::ExecPage;
        use rustc_hash::FxHashMap;

        let layout = FrameLayout {
            slots: Vec::new(),
            lookup: FxHashMap::default(),
        };
        let mut e = X86Emitter::new();
        e.prologue(&layout);
        for (i, (op, rhs)) in ops.iter().enumerate() {
            e.mov_imm(Tmp::T2, 0xDEAD, OpWidth::W64); // garbage high half of dividend
            e.load_slot(Tmp::T0, FrameSlot::new(0), w);
            e.load_slot(Tmp::T1, FrameSlot::new(1), w);
            e.alu(*op, Tmp::T0, Tmp::T0, *rhs, w, Signedness::Unsigned);
            e.store_slot(FrameSlot::new(2 + i as u16), Tmp::T0, w);
        }
        e.ret_exit(0);
        let page = ExecPage::commit(&e.finish().unwrap()).unwrap();
        unsafe { page.as_fn()(frame.as_mut_ptr(), std::ptr::null_mut()) };
    }

    #[test]
    fn div_mod_64_ignore_dirty_rdx() {
        let ops = [
            (AluOp::Div, Src::Tmp(Tmp::T1)),
            (AluOp::Mod, Src::Tmp(Tmp::T1)),
            (AluOp::Div, Src::Imm(7)),
            (AluOp::Mod, Src::Imm(7)),
        ];
        let mut frame = [100u64, 3, 0, 0, 0, 0];
        run_div_mod(OpWidth::W64, &ops, &mut frame);
        assert_eq!(frame, [100, 3, 33, 1, 14, 2]);
    }

    #[test]
    fn div_mod_32_ignore_dirty_rdx() {
        let ops = [
            (AluOp::Div, Src::Tmp(Tmp::T1)),
            (AluOp::Mod, Src::Tmp(Tmp::T1)),
            (AluOp::Div, Src::Imm(7)),
            (AluOp::Mod, Src::Imm(7)),
        ];
        let mut frame = [100u64, 3, 0, 0, 0, 0];
        run_div_mod(OpWidth::W32, &ops, &mut frame);
        assert_eq!(frame, [100, 3, 33, 1, 14, 2]);
    }

    #[test]
    fn div_mod_divisor_aliasing_rax_or_rdx() {
        use crate::vm::compiler::exec_mem::ExecPage;
        use rustc_hash::FxHashMap;

        let layout = FrameLayout {
            slots: Vec::new(),
            lookup: FxHashMap::default(),
        };
        let mut e = X86Emitter::new();
        e.prologue(&layout);
        // slot2 = slot0 / T0 where T0 = slot1 (divisor in rax), lhs in rcx
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
        // slot3 = slot0 % T2 where T2 = slot1 (divisor in rdx)
        e.load_slot(Tmp::T0, FrameSlot::new(0), OpWidth::W64);
        e.load_slot(Tmp::T2, FrameSlot::new(1), OpWidth::W64);
        e.alu(
            AluOp::Mod,
            Tmp::T1,
            Tmp::T0,
            Src::Tmp(Tmp::T2),
            OpWidth::W64,
            Signedness::Unsigned,
        );
        e.store_slot(FrameSlot::new(3), Tmp::T1, OpWidth::W64);
        e.ret_exit(0);
        let page = ExecPage::commit(&e.finish().unwrap()).unwrap();

        let mut frame = [100u64, 3, 0, 0];
        unsafe { page.as_fn()(frame.as_mut_ptr(), std::ptr::null_mut()) };
        assert_eq!(frame, [100, 3, 33, 1]);
    }

    extern "C" fn host_sub(a: u64, b: u64) -> u64 {
        a.wrapping_sub(b)
    }

    // args in reverse Tmp order would clobber each other on Win64 without staging
    #[test]
    fn call_host_stages_tmp_args() {
        use crate::vm::compiler::exec_mem::ExecPage;
        let mut e = X86Emitter::new();
        e.prologue(&FrameLayout::new());
        e.load_slot(Tmp::T1, FrameSlot::new(0), OpWidth::W64);
        e.load_slot(Tmp::T2, FrameSlot::new(1), OpWidth::W64);
        e.call_host(
            host_sub as *const () as usize,
            &[Arg::Tmp(Tmp::T2), Arg::Tmp(Tmp::T1)],
        );
        e.store_slot(FrameSlot::new(2), Tmp::T0, OpWidth::W64);
        e.ret_exit(0);
        let page = ExecPage::commit(&e.finish().unwrap()).unwrap();
        let mut frame = [10u64, 3, 0];
        unsafe { page.as_fn()(frame.as_mut_ptr(), std::ptr::null_mut()) };
        assert_eq!(frame[2], 3u64.wrapping_sub(10));
    }
}
