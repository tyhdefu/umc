use super::*;

impl ArmEmitter {
    /// arbitrary immediate into rd, W32 imms can arrive sign extended (e.g. -1 as u64::MAX) so only low half kept
    fn load_imm(&mut self, rd: u8, imm: u64, w: OpWidth) {
        let imm = if w == OpWidth::W32 {
            imm & 0xFFFF_FFFF
        } else {
            imm
        };
        mov_imm64(&mut self.ops, rd, imm);
    }

    fn cmp_flags(&mut self, lhs: Tmp, rhs: Src, w: OpWidth) {
        let l = reg(lhs);
        let r = match rhs {
            Src::Tmp(s) => reg(s),
            // cmp takes a 12 bit unsigned immediate directly, imm forms use the SP register family in dynasm
            Src::Imm(i) if i < 4096 => {
                match w {
                    OpWidth::W64 => arm!(self.ops ; cmp XSP(l), #i as u32),
                    OpWidth::W32 => arm!(self.ops ; cmp WSP(l), #i as u32),
                }
                return;
            }
            Src::Imm(i) => {
                self.load_imm(SCRATCH, i, w);
                SCRATCH
            }
        };
        match w {
            OpWidth::W64 => arm!(self.ops ; cmp X(l), X(r)),
            OpWidth::W32 => arm!(self.ops ; cmp W(l), W(r)),
        }
    }

    /// dst = lhs / rhs or lhs % rhs
    /// udiv/sdiv are 3 operand with no fixed regs, mod has no instruction so it's lhs - (lhs / rhs) * rhs via msub
    /// arm doesn't trap on /0 (gives 0) or MIN / -1 (gives MIN), lower::guard_divisor bails on those anyway
    fn div_mod(&mut self, op: AluOp, dst: Tmp, lhs: Tmp, rhs: Src, w: OpWidth, sign: Signedness) {
        let d = match rhs {
            Src::Tmp(s) => reg(s),
            Src::Imm(i) => {
                self.load_imm(SCRATCH, i, w);
                SCRATCH
            }
        };
        // mod keeps the quotient in SCRATCH2 so dst may alias lhs or rhs, msub still needs both
        let q = if op == AluOp::Mod { SCRATCH2 } else { reg(dst) };
        let l = reg(lhs);
        match (w, sign) {
            (OpWidth::W64, Signedness::Unsigned) => arm!(self.ops ; udiv X(q), X(l), X(d)),
            (OpWidth::W64, Signedness::Signed) => arm!(self.ops ; sdiv X(q), X(l), X(d)),
            (OpWidth::W32, Signedness::Unsigned) => arm!(self.ops ; udiv W(q), W(l), W(d)),
            (OpWidth::W32, Signedness::Signed) => arm!(self.ops ; sdiv W(q), W(l), W(d)),
        }
        if op == AluOp::Mod {
            match w {
                OpWidth::W64 => arm!(self.ops ; msub X(reg(dst)), X(q), X(d), X(l)),
                OpWidth::W32 => arm!(self.ops ; msub W(reg(dst)), W(q), W(d), W(l)),
            }
        }
    }
}

impl IntEmitter for ArmEmitter {
    // ldr/str imm offset is unsigned, scaled by access size, so reaches 4095 * size, past that index via SCRATCH2
    fn load_slot(&mut self, dst: Tmp, slot: FrameSlot, w: OpWidth) {
        let off = slot.byte_offset() as u32;
        let d = reg(dst);
        match w {
            OpWidth::W64 if off <= 4095 * 8 => arm!(self.ops ; ldr X(d), [X(FRAME_BASE), #off]),
            OpWidth::W32 if off <= 4095 * 4 => arm!(self.ops ; ldr W(d), [X(FRAME_BASE), #off]),
            _ => {
                mov_imm64(&mut self.ops, SCRATCH2, off as u64);
                match w {
                    OpWidth::W64 => arm!(self.ops ; ldr X(d), [X(FRAME_BASE), X(SCRATCH2)]),
                    OpWidth::W32 => arm!(self.ops ; ldr W(d), [X(FRAME_BASE), X(SCRATCH2)]),
                }
            }
        }
    }
    fn store_slot(&mut self, slot: FrameSlot, src: Tmp, w: OpWidth) {
        let off = slot.byte_offset() as u32;
        let s = reg(src);
        match w {
            OpWidth::W64 if off <= 4095 * 8 => arm!(self.ops ; str X(s), [X(FRAME_BASE), #off]),
            OpWidth::W32 if off <= 4095 * 4 => arm!(self.ops ; str W(s), [X(FRAME_BASE), #off]),
            _ => {
                mov_imm64(&mut self.ops, SCRATCH2, off as u64);
                match w {
                    OpWidth::W64 => arm!(self.ops ; str X(s), [X(FRAME_BASE), X(SCRATCH2)]),
                    OpWidth::W32 => arm!(self.ops ; str W(s), [X(FRAME_BASE), X(SCRATCH2)]),
                }
            }
        }
    }

    // shortcut: always movz + movk, negatives cost up to 4 instrs where movn would be 1, upgrade if hot
    fn mov_imm(&mut self, dst: Tmp, imm: u64, w: OpWidth) {
        self.load_imm(reg(dst), imm, w);
    }
    fn mov_rr(&mut self, dst: Tmp, src: Tmp, w: OpWidth) {
        if dst == src {
            return;
        }
        match w {
            OpWidth::W64 => arm!(self.ops ; mov X(reg(dst)), X(reg(src))),
            OpWidth::W32 => arm!(self.ops ; mov W(reg(dst)), W(reg(src))),
        }
    }
    fn alu_rr(&mut self, op: AluOp, dst: Tmp, src: Tmp, w: OpWidth, sign: Signedness) {
        self.alu(op, dst, dst, Src::Tmp(src), w, sign);
    }
    fn neg(&mut self, dst: Tmp, w: OpWidth) {
        let d = reg(dst);
        match w {
            OpWidth::W64 => arm!(self.ops ; neg X(d), X(d)),
            OpWidth::W32 => arm!(self.ops ; neg W(d), W(d)),
        }
    }

    /// arm is 3 address so no mov / dst == rhs juggling like x86
    fn alu(&mut self, op: AluOp, dst: Tmp, lhs: Tmp, rhs: Src, w: OpWidth, sign: Signedness) {
        if let AluOp::Div | AluOp::Mod = op {
            return self.div_mod(op, dst, lhs, rhs, w, sign);
        }
        let (d, l) = (reg(dst), reg(lhs));
        let r = match rhs {
            Src::Tmp(r) => reg(r),
            // add/sub take a 12 bit unsigned immediate, everything else goes via SCRATCH
            Src::Imm(i) if i < 4096 && matches!(op, AluOp::Add | AluOp::Sub) => {
                match (op, w) {
                    (AluOp::Add, OpWidth::W64) => arm!(self.ops ; add XSP(d), XSP(l), #i as u32),
                    (AluOp::Add, OpWidth::W32) => arm!(self.ops ; add WSP(d), WSP(l), #i as u32),
                    (_, OpWidth::W64) => arm!(self.ops ; sub XSP(d), XSP(l), #i as u32),
                    (_, OpWidth::W32) => arm!(self.ops ; sub WSP(d), WSP(l), #i as u32),
                }
                return;
            }
            Src::Imm(i) => {
                self.load_imm(SCRATCH, i, w);
                SCRATCH
            }
        };
        alu3(&mut self.ops, op, w, d, l, r);
    }
    fn unop(&mut self, op: UnOp, dst: Tmp, src: Src, w: OpWidth) {
        match (op, src) {
            (UnOp::Mov, Src::Tmp(s)) => self.mov_rr(dst, s, w),
            (UnOp::Mov, Src::Imm(i)) => self.mov_imm(dst, i, w),
            (UnOp::Not, Src::Tmp(s)) => match w {
                OpWidth::W64 => arm!(self.ops ; mvn X(reg(dst)), X(reg(s))),
                OpWidth::W32 => arm!(self.ops ; mvn W(reg(dst)), W(reg(s))),
            },
            // figured out at compile time, load_imm masks W32
            (UnOp::Not, Src::Imm(i)) => self.mov_imm(dst, !i, w),
        }
    }

    /// reg form (lslv) masks the amount mod width in hardware, imm is masked here to match, same as x86 shl
    fn lshift(&mut self, dst: Tmp, size: Src, w: OpWidth) {
        let d = reg(dst);
        match (size, w) {
            (Src::Tmp(s), OpWidth::W64) => arm!(self.ops ; lsl X(d), X(d), X(reg(s))),
            (Src::Tmp(s), OpWidth::W32) => arm!(self.ops ; lsl W(d), W(d), W(reg(s))),
            (Src::Imm(i), OpWidth::W64) => arm!(self.ops ; lsl X(d), X(d), #i as u32 & 63),
            (Src::Imm(i), OpWidth::W32) => arm!(self.ops ; lsl W(d), W(d), #i as u32 & 31),
        }
    }
    fn l_rshift(&mut self, dst: Tmp, size: Src, w: OpWidth) {
        let d = reg(dst);
        match (size, w) {
            (Src::Tmp(s), OpWidth::W64) => arm!(self.ops ; lsr X(d), X(d), X(reg(s))),
            (Src::Tmp(s), OpWidth::W32) => arm!(self.ops ; lsr W(d), W(d), W(reg(s))),
            (Src::Imm(i), OpWidth::W64) => arm!(self.ops ; lsr X(d), X(d), #i as u32 & 63),
            (Src::Imm(i), OpWidth::W32) => arm!(self.ops ; lsr W(d), W(d), #i as u32 & 31),
        }
    }
    fn a_rshift(&mut self, dst: Tmp, size: Src, w: OpWidth) {
        let d = reg(dst);
        match (size, w) {
            (Src::Tmp(s), OpWidth::W64) => arm!(self.ops ; asr X(d), X(d), X(reg(s))),
            (Src::Tmp(s), OpWidth::W32) => arm!(self.ops ; asr W(d), W(d), W(reg(s))),
            (Src::Imm(i), OpWidth::W64) => arm!(self.ops ; asr X(d), X(d), #i as u32 & 63),
            (Src::Imm(i), OpWidth::W32) => arm!(self.ops ; asr W(d), W(d), #i as u32 & 31),
        }
    }

    /// keep low `bits`, zero the rest, single ubfx instead of x86's shl + shr
    fn truncate_unsigned(&mut self, dst: Tmp, bits: RegWidth, w: OpWidth) {
        let d = reg(dst);
        let bits = bits as u32;
        match w {
            OpWidth::W64 => {
                debug_assert!(bits <= 64);
                if bits != 0 && bits < 64 {
                    arm!(self.ops ; ubfx X(d), X(d), #0, #bits);
                }
            }
            OpWidth::W32 => {
                debug_assert!(bits <= 32);
                if bits != 0 && bits < 32 {
                    arm!(self.ops ; ubfx W(d), W(d), #0, #bits);
                }
            }
        }
    }
    fn sign_extend(&mut self, dst: Tmp, bits: RegWidth, w: OpWidth) {
        let d = reg(dst);
        let bits = bits as u32;
        match w {
            OpWidth::W64 => {
                debug_assert!(bits <= 64);
                if bits != 0 && bits < 64 {
                    arm!(self.ops ; sbfx X(d), X(d), #0, #bits);
                }
            }
            OpWidth::W32 => {
                debug_assert!(bits <= 32);
                if bits != 0 && bits < 32 {
                    arm!(self.ops ; sbfx W(d), W(d), #0, #bits);
                }
            }
        }
    }

    /// cset writes 0/1 into W, already zero extended so no movzx needed
    fn set_cmp(&mut self, dst: Tmp, cond: CmpCond, lhs: Tmp, rhs: Src, w: OpWidth, s: Signedness) {
        self.cmp_flags(lhs, rhs, w);
        let d = reg(dst);

        use CmpCond::*;
        use Signedness::*;
        match (cond, s) {
            (Eq, _) => arm!(self.ops ; cset W(d), eq),
            (Ne, _) => arm!(self.ops ; cset W(d), ne),
            (Lt, Signed) => arm!(self.ops ; cset W(d), lt),
            (Lt, Unsigned) => arm!(self.ops ; cset W(d), lo),
            (Le, Signed) => arm!(self.ops ; cset W(d), le),
            (Le, Unsigned) => arm!(self.ops ; cset W(d), ls),
            (Gt, Signed) => arm!(self.ops ; cset W(d), gt),
            (Gt, Unsigned) => arm!(self.ops ; cset W(d), hi),
            (Ge, Signed) => arm!(self.ops ; cset W(d), ge),
            (Ge, Unsigned) => arm!(self.ops ; cset W(d), hs),
        }
    }
    fn branch_cmp(
        &mut self,
        cond: CmpCond,
        lhs: Tmp,
        rhs: Src,
        w: OpWidth,
        s: Signedness,
        target: Label,
    ) {
        let st = &mut self.labels[target.0 as usize];
        st.referenced = true;
        let dl = st.dl;
        let l = reg(lhs);

        use CmpCond::*;
        use Signedness::*;
        match (cond, rhs, w) {
            (Eq, Src::Imm(0), OpWidth::W64) => return arm!(self.ops ; cbz X(l), =>dl),
            (Eq, Src::Imm(0), OpWidth::W32) => return arm!(self.ops ; cbz W(l), =>dl),
            (Ne, Src::Imm(0), OpWidth::W64) => return arm!(self.ops ; cbnz X(l), =>dl),
            (Ne, Src::Imm(0), OpWidth::W32) => return arm!(self.ops ; cbnz W(l), =>dl),
            _ => {}
        }

        self.cmp_flags(lhs, rhs, w);
        match (cond, s) {
            (Eq, _) => arm!(self.ops ; b.eq =>dl),
            (Ne, _) => arm!(self.ops ; b.ne =>dl),
            (Lt, Signed) => arm!(self.ops ; b.lt =>dl),
            (Lt, Unsigned) => arm!(self.ops ; b.lo =>dl),
            (Le, Signed) => arm!(self.ops ; b.le =>dl),
            (Le, Unsigned) => arm!(self.ops ; b.ls =>dl),
            (Gt, Signed) => arm!(self.ops ; b.gt =>dl),
            (Gt, Unsigned) => arm!(self.ops ; b.hi =>dl),
            (Ge, Signed) => arm!(self.ops ; b.ge =>dl),
            (Ge, Unsigned) => arm!(self.ops ; b.hs =>dl),
        }
    }
}

/// ALU 3 address Register-Register helper func, d = l op r
fn alu3(ops: &mut VecAssembler<Aarch64Relocation>, op: AluOp, w: OpWidth, d: u8, l: u8, r: u8) {
    match w {
        OpWidth::W64 => match op {
            AluOp::Add => arm!(ops ; add X(d), X(l), X(r)),
            AluOp::Sub => arm!(ops ; sub X(d), X(l), X(r)),
            AluOp::Mul => arm!(ops ; mul X(d), X(l), X(r)),
            AluOp::And => arm!(ops ; and X(d), X(l), X(r)),
            AluOp::Or => arm!(ops ; orr X(d), X(l), X(r)),
            AluOp::Xor => arm!(ops ; eor X(d), X(l), X(r)),
            AluOp::Div | AluOp::Mod => unreachable!("div/mod are emitted by ArmEmitter::div_mod"),
        },
        OpWidth::W32 => match op {
            AluOp::Add => arm!(ops ; add W(d), W(l), W(r)),
            AluOp::Sub => arm!(ops ; sub W(d), W(l), W(r)),
            AluOp::Mul => arm!(ops ; mul W(d), W(l), W(r)),
            AluOp::And => arm!(ops ; and W(d), W(l), W(r)),
            AluOp::Or => arm!(ops ; orr W(d), W(l), W(r)),
            AluOp::Xor => arm!(ops ; eor W(d), W(l), W(r)),
            AluOp::Div | AluOp::Mod => unreachable!("div/mod are emitted by ArmEmitter::div_mod"),
        },
    }
}
