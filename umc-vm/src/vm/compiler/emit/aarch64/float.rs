use super::*;

impl ArmEmitter {
    fn fcmp_flags(&mut self, lhs: FTmp, rhs: FTmp, w: OpWidth) {
        let (l, r) = (freg(lhs), freg(rhs));
        match w {
            OpWidth::W64 => arm!(self.ops ; fcmp D(l), D(r)),
            OpWidth::W32 => arm!(self.ops ; fcmp S(l), S(r)),
        }
    }
}

impl FloatEmitter for ArmEmitter {
    // ldr/str imm offset is unsigned, scaled by access size, so reaches 4095 * size, past that index via SCRATCH2
    fn fload_slot(&mut self, dst: FTmp, slot: FrameSlot, w: OpWidth) {
        let off = slot.byte_offset() as u32;
        let d = freg(dst);
        match w {
            OpWidth::W64 if off <= 4095 * 8 => arm!(self.ops ; ldr D(d), [X(FRAME_BASE), #off]),
            OpWidth::W32 if off <= 4095 * 4 => arm!(self.ops ; ldr S(d), [X(FRAME_BASE), #off]),
            _ => {
                mov_imm64(&mut self.ops, SCRATCH2, off as u64);
                match w {
                    OpWidth::W64 => arm!(self.ops ; ldr D(d), [X(FRAME_BASE), X(SCRATCH2)]),
                    OpWidth::W32 => arm!(self.ops ; ldr S(d), [X(FRAME_BASE), X(SCRATCH2)]),
                }
            }
        }
    }
    fn fstore_slot(&mut self, slot: FrameSlot, src: FTmp, w: OpWidth) {
        let off = slot.byte_offset() as u32;
        let s = freg(src);
        match w {
            OpWidth::W64 if off <= 4095 * 8 => arm!(self.ops ; str D(s), [X(FRAME_BASE), #off]),
            OpWidth::W32 if off <= 4095 * 4 => arm!(self.ops ; str S(s), [X(FRAME_BASE), #off]),
            _ => {
                mov_imm64(&mut self.ops, SCRATCH2, off as u64);
                match w {
                    OpWidth::W64 => arm!(self.ops ; str D(s), [X(FRAME_BASE), X(SCRATCH2)]),
                    OpWidth::W32 => arm!(self.ops ; str S(s), [X(FRAME_BASE), X(SCRATCH2)]),
                }
            }
        }
    }

    // shortcut: always movz + movk, negatives cost up to 4 instrs where movn would be 1, upgrade if hot
    fn fmov_imm(&mut self, dst: FTmp, bits: u64, w: OpWidth) {
        let d = freg(dst);
        mov_imm64(&mut self.ops, SCRATCH, bits);
        match w {
            OpWidth::W64 => arm!(self.ops ; fmov D(d), X(SCRATCH)),
            OpWidth::W32 => arm!(self.ops ; fmov S(d), W(SCRATCH)),
        }
    }

    fn fsqrt(&mut self, dst: FTmp, w: OpWidth) {
        let d = freg(dst);
        match w {
            OpWidth::W64 => arm!(self.ops ; fsqrt D(d), D(d)),
            OpWidth::W32 => arm!(self.ops ; fsqrt S(d), S(d)),
        }
    }

    /// arm is 3 address so no mov / dst == rhs juggling like x86
    fn falu(&mut self, op: FAluOp, dst: FTmp, rhs: FTmp, w: OpWidth) {
        let (d, r) = (freg(dst), freg(rhs));
        match w {
            OpWidth::W64 => match op {
                FAluOp::Add => arm!(self.ops ; fadd D(d), D(d), D(r)),
                FAluOp::Sub => arm!(self.ops ; fsub D(d), D(d), D(r)),
                FAluOp::Mul => arm!(self.ops ; fmul D(d), D(d), D(r)),
                FAluOp::Div => arm!(self.ops ; fdiv D(d), D(d), D(r)),
            },
            OpWidth::W32 => match op {
                FAluOp::Add => arm!(self.ops ; fadd S(d), S(d), S(r)),
                FAluOp::Sub => arm!(self.ops ; fsub S(d), S(d), S(r)),
                FAluOp::Mul => arm!(self.ops ; fmul S(d), S(d), S(r)),
                FAluOp::Div => arm!(self.ops ; fdiv S(d), S(d), S(r)),
            },
        }
    }
    /// cset writes 0/1 into W, already zero extended so no movzx needed
    fn fset_cmp(&mut self, dst: Tmp, cond: CmpCond, lhs: FTmp, rhs: FTmp, w: OpWidth) {
        self.fcmp_flags(lhs, rhs, w);
        let d = reg(dst);

        use CmpCond::*;
        match cond {
            Eq => arm!(self.ops ; cset X(d), eq),
            Ne => arm!(self.ops ; cset X(d), ne),
            Lt => arm!(self.ops ; cset X(d), lo),
            Le => arm!(self.ops ; cset X(d), ls),
            Gt => arm!(self.ops ; cset X(d), hi),
            Ge => arm!(self.ops ; cset X(d), hs),
        }
    }
}
