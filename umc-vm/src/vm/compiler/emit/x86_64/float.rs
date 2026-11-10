use super::*;

impl FloatEmitter for X86Emitter {
    fn fload_slot(&mut self, dst: FTmp, slot: FrameSlot, w: OpWidth) {
        let m = FRAME_BASE + slot.byte_offset();
        let r = match w {
            OpWidth::W64 => self.asm.movsd_2(xmm(dst), qword_ptr(m)),
            OpWidth::W32 => self.asm.movss(xmm(dst), dword_ptr(m)),
        };
        record(&mut self.error, r)
    }
    fn fstore_slot(&mut self, slot: FrameSlot, src: FTmp, w: OpWidth) {
        let m = FRAME_BASE + slot.byte_offset();
        let r = match w {
            OpWidth::W64 => self.asm.movsd_2(qword_ptr(m), xmm(src)),
            OpWidth::W32 => self.asm.movss(dword_ptr(m), xmm(src)),
        };
        record(&mut self.error, r)
    }

    // register-immediate move
    fn fmov_imm(&mut self, dst: FTmp, bits: u64, w: OpWidth) {
        let r = match w {
            OpWidth::W64 => {
                record(&mut self.error, self.asm.mov(r11, bits));
                self.asm.movq(xmm(dst), r11)
            }
            OpWidth::W32 => {
                record(&mut self.error, self.asm.mov(r11d, bits as u32));
                self.asm.movd(xmm(dst), r11d)
            }
        };
        record(&mut self.error, r);
    }

    fn fsqrt(&mut self, dst: FTmp, w: OpWidth) {
        let r = match w {
            OpWidth::W64 => self.asm.sqrtsd(xmm(dst), xmm(dst)),
            OpWidth::W32 => self.asm.sqrtss(xmm(dst), xmm(dst)),
        };
        record(&mut self.error, r);
    }

    fn falu(&mut self, op: FAluOp, dst: FTmp, rhs: FTmp, w: OpWidth) {
        match w {
            OpWidth::W64 => {
                record(
                    &mut self.error,
                    falu_64(&mut self.asm, op, xmm(dst), xmm(rhs)),
                );
            }
            OpWidth::W32 => {
                record(
                    &mut self.error,
                    falu_32(&mut self.asm, op, xmm(dst), xmm(rhs)),
                );
            }
        };
    }

    fn fset_cmp(&mut self, dst: Tmp, cond: CmpCond, lhs: FTmp, rhs: FTmp, w: OpWidth) {
        use CmpCond::*;
        // ucomis sets ZF=PF=CF=1 when unordered, so only "above" style checks are false on NaN
        let (a, b) = match cond {
            Lt | Le => (rhs, lhs), // a < b <=> b > a
            _ => (lhs, rhs),
        };
        let r = match w {
            OpWidth::W32 => self.asm.ucomiss(xmm(a), xmm(b)),
            OpWidth::W64 => self.asm.ucomisd(xmm(a), xmm(b)),
        };
        record(&mut self.error, r);
        let d = reg8(dst);
        let r = match cond {
            Gt | Lt => self.asm.seta(d),
            Ge | Le => self.asm.setae(d),
            // equal and ordered
            Eq => self
                .asm
                .sete(d)
                .and_then(|_| self.asm.setnp(r11b))
                .and_then(|_| self.asm.and(d, r11b)),
            // not equal or unordered
            Ne => self
                .asm
                .setne(d)
                .and_then(|_| self.asm.setp(r11b))
                .and_then(|_| self.asm.or(d, r11b)),
        };
        record(&mut self.error, r);
        record(&mut self.error, self.asm.movzx(r32(dst), d));
    }
}

/// ALU 64bit float helper func
fn falu_64(
    a: &mut CodeAssembler,
    op: FAluOp,
    l: AsmRegisterXmm,
    r: AsmRegisterXmm,
) -> Result<(), IcedError> {
    match op {
        FAluOp::Add => a.addsd(l, r),
        FAluOp::Sub => a.subsd(l, r),
        FAluOp::Mul => a.mulsd(l, r),
        FAluOp::Div => a.divsd(l, r),
    }
}

/// ALU 32bit float helper func
fn falu_32(
    a: &mut CodeAssembler,
    op: FAluOp,
    l: AsmRegisterXmm,
    r: AsmRegisterXmm,
) -> Result<(), IcedError> {
    match op {
        FAluOp::Add => a.addss(l, r),
        FAluOp::Sub => a.subss(l, r),
        FAluOp::Mul => a.mulss(l, r),
        FAluOp::Div => a.divss(l, r),
    }
}
