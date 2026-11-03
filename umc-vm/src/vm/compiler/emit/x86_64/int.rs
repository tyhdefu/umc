use super::*;

impl X86Emitter {
    fn cmp_flags(&mut self, lhs: Tmp, rhs: Src, w: OpWidth) -> Result<(), IcedError> {
        match rhs {
            Src::Tmp(s) => match w {
                OpWidth::W64 => self.asm.cmp(r64(lhs), r64(s)),
                OpWidth::W32 => self.asm.cmp(r32(lhs), r32(s)),
            },
            Src::Imm(i) => match w {
                // avoid truncation by copying to scratch register if needed
                OpWidth::W64 => match i32::try_from(i as i64) {
                    Ok(j) => self.asm.cmp(r64(lhs), j),
                    Err(_) => {
                        self.asm.mov(SCRATCH64, i)?;
                        self.asm.cmp(r64(lhs), SCRATCH64)
                    }
                },
                OpWidth::W32 => self.asm.cmp(r32(lhs), i as i32),
            },
        }
    }

    /// unsigned dst = lhs / rhs or lhs % rhs
    /// div divides rdx:rax by its operand, quotient -> rax, remainder -> rdx, so T0 and T2 are clobbered
    fn div_mod(&mut self, op: AluOp, dst: Tmp, lhs: Tmp, rhs: Src, w: OpWidth, sign: Signedness) {
        match sign {
            Signedness::Unsigned => {
                // divisor goes in scratch first so writing rax / rdx below can't overwrite it
                let r = match rhs {
                    Src::Tmp(s) => self.asm.mov(SCRATCH64, r64(s)),
                    Src::Imm(imm) => self.asm.mov(SCRATCH64, imm),
                };
                record(&mut self.error, r);
                self.mov_rr(Tmp::T0, lhs, w);
                // high half of dividend must be zero, otherwise quotient is wrong or faults on overflow
                record(&mut self.error, self.asm.xor(edx, edx));
                let r = match w {
                    OpWidth::W64 => self.asm.div(SCRATCH64),
                    OpWidth::W32 => self.asm.div(SCRATCH32),
                };
                record(&mut self.error, r);
                let result = if op == AluOp::Mod { Tmp::T2 } else { Tmp::T0 };
                self.mov_rr(dst, result, w);
            }
            Signedness::Signed => {
                // divisor goes in scratch first so writing rax / rdx below can't overwrite it
                let r = match rhs {
                    Src::Tmp(s) => self.asm.mov(SCRATCH64, r64(s)),
                    Src::Imm(imm) => self.asm.mov(SCRATCH64, imm),
                };
                record(&mut self.error, r);
                self.mov_rr(Tmp::T0, lhs, w);
                // high half of dividend must be sign extension of dividend, otherwise quotient is wrong or faults on overflow
                match w {
                    OpWidth::W32 => record(&mut self.error, self.asm.cdq()),
                    OpWidth::W64 => record(&mut self.error, self.asm.cqo()),
                }
                let r = match w {
                    OpWidth::W64 => self.asm.idiv(SCRATCH64),
                    OpWidth::W32 => self.asm.idiv(SCRATCH32),
                };
                record(&mut self.error, r);
                let result = if op == AluOp::Mod { Tmp::T2 } else { Tmp::T0 };
                self.mov_rr(dst, result, w);
            }
        }
    }
}

impl IntEmitter for X86Emitter {
    fn load_slot(&mut self, dst: Tmp, slot: FrameSlot, w: OpWidth) {
        let m = FRAME_BASE + slot.byte_offset();
        let r = match w {
            OpWidth::W64 => self.asm.mov(r64(dst), qword_ptr(m)),
            OpWidth::W32 => self.asm.mov(r32(dst), dword_ptr(m)),
        };
        record(&mut self.error, r)
    }
    fn store_slot(&mut self, slot: FrameSlot, src: Tmp, w: OpWidth) {
        let m = FRAME_BASE + slot.byte_offset();
        let r = match w {
            OpWidth::W64 => self.asm.mov(qword_ptr(m), r64(src)),
            OpWidth::W32 => self.asm.mov(dword_ptr(m), r32(src)),
        };
        record(&mut self.error, r)
    }

    // register-immediate move
    fn mov_imm(&mut self, dst: Tmp, imm: u64, w: OpWidth) {
        let r = match w {
            OpWidth::W64 => {
                if let Ok(u) = u32::try_from(imm) {
                    self.asm.mov(r32(dst), u) // if can be directly converted from u32 do it
                } else if let Ok(s) = i32::try_from(imm as i64) {
                    // if not, sign-extended imm32 into 64 bit reg
                    Instruction::with2(Code::Mov_rm64_imm32, Register::from(r64(dst)), s)
                        .and_then(|i| self.asm.add_instruction(i))
                } else {
                    self.asm.mov(r64(dst), imm) // or stick to 64 bit reg directly
                }
            }
            OpWidth::W32 => {
                // W32 keeps the low half (matches aarch64); signed consts arrive sign-extended
                self.asm.mov(r32(dst), imm as u32)
            }
        };
        record(&mut self.error, r);
    }
    // register register move
    fn mov_rr(&mut self, dst: Tmp, src: Tmp, w: OpWidth) {
        if dst == src {
            return;
        }

        let r = match w {
            OpWidth::W64 => self.asm.mov(r64(dst), r64(src)),
            OpWidth::W32 => self.asm.mov(r32(dst), r32(src)),
        };
        record(&mut self.error, r);
    }
    // register register ALU op
    fn alu_rr(&mut self, op: AluOp, dst: Tmp, src: Tmp, w: OpWidth, signed: Signedness) {
        if let AluOp::Div | AluOp::Mod = op {
            return self.div_mod(op, dst, dst, Src::Tmp(src), w, signed);
        }
        let r = match w {
            OpWidth::W64 => alu_rr64(&mut self.asm, op, r64(dst), r64(src)),
            OpWidth::W32 => alu_rr32(&mut self.asm, op, r32(dst), r32(src)),
        };
        record(&mut self.error, r);
    }
    // negate register
    fn neg(&mut self, dst: Tmp, w: OpWidth) {
        let r = match w {
            OpWidth::W64 => self.asm.neg(r64(dst)),
            OpWidth::W32 => self.asm.neg(r32(dst)),
        };
        record(&mut self.error, r);
    }

    /// ALU is 3 address but x86 is 2 address so we need to translate dst = lhs op rhs -> dst = dst op rhs
    /// so now ALU ops become mov dst, lhs then op dst, rhs
    /// but need to account for dst == rhs and dst != lhs as the mov would override rhs before its read
    fn alu(&mut self, op: AluOp, dst: Tmp, lhs: Tmp, rhs: Src, w: OpWidth, sign: Signedness) {
        // Handle Div / Mod Static register calling convention
        match op {
            AluOp::Div | AluOp::Mod => self.div_mod(op, dst, lhs, rhs, w, sign),
            _ => {
                // handle other ops
                match rhs {
                    Src::Tmp(r) if r == dst && dst != lhs => match op {
                        AluOp::Sub => {
                            // case: turn dst = lhs - rhs where dst == rhs
                            // into: dst = -rhs -> dst + lhs
                            // i.e. lhs - rhs
                            self.neg(dst, w);
                            self.alu_rr(AluOp::Add, dst, lhs, w, sign);
                        }
                        _ => self.alu_rr(op, dst, lhs, w, sign), // commutative so just do as normal
                    },
                    Src::Tmp(r) => {
                        // just do normal move into dst then do op
                        self.mov_rr(dst, lhs, w);
                        self.alu_rr(op, dst, r, w, sign);
                    }
                    Src::Imm(imm) => {
                        self.mov_rr(dst, lhs, w);
                        let r = match w {
                            OpWidth::W32 => alu_ri32(&mut self.asm, op, r32(dst), imm as i32),
                            OpWidth::W64 => match i32::try_from(imm as i64) {
                                // if imm fits in i32 use register-immediate op
                                Ok(s) => alu_ri64(&mut self.asm, op, r64(dst), s),
                                Err(_) => {
                                    // too wide for imm32, go r11 and do a register-register op
                                    record(&mut self.error, self.asm.mov(SCRATCH64, imm));
                                    alu_rr64(&mut self.asm, op, r64(dst), SCRATCH64)
                                }
                            },
                        };
                        record(&mut self.error, r);
                    }
                }
            }
        }
    }
    /// unop is a mov or not
    fn unop(&mut self, op: UnOp, dst: Tmp, src: Src, w: OpWidth) {
        match (op, src) {
            (UnOp::Mov, Src::Tmp(s)) => self.mov_rr(dst, s, w),
            (UnOp::Mov, Src::Imm(i)) => self.mov_imm(dst, i, w),
            (UnOp::Not, Src::Tmp(s)) => {
                self.mov_rr(dst, s, w);
                let r = match w {
                    OpWidth::W32 => self.asm.not(r32(dst)),
                    OpWidth::W64 => self.asm.not(r64(dst)),
                };
                record(&mut self.error, r);
            }
            (UnOp::Not, Src::Imm(i)) => {
                // figured out at compile time
                let nw = match w {
                    OpWidth::W32 => !i & 0xFFFF_FFFF,
                    OpWidth::W64 => !i,
                };
                self.mov_imm(dst, nw, w);
            }
        }
    }

    /// UMC wrap at declared width - noop when bits already natural for withd
    fn truncate_unsigned(&mut self, dst: Tmp, bits: RegWidth, w: OpWidth) {
        let r = match w {
            OpWidth::W64 => {
                debug_assert!(bits <= 64);
                if bits > 64 {
                    return;
                }
                if bits == 64 || bits == 0 {
                    self.asm.nop()
                } else {
                    record(&mut self.error, self.asm.shl(r64(dst), 64 - bits));
                    self.asm.shr(r64(dst), 64 - bits)
                }
            }
            OpWidth::W32 => {
                debug_assert!(bits <= 32);
                if bits > 32 {
                    return;
                }
                if bits == 32 || bits == 0 {
                    self.asm.nop()
                } else {
                    record(&mut self.error, self.asm.shl(r32(dst), 32 - bits));
                    self.asm.shr(r32(dst), 32 - bits)
                }
            }
        };
        record(&mut self.error, r);
    }
    fn sign_extend(&mut self, dst: Tmp, bits: RegWidth, w: OpWidth) {
        let r = match w {
            OpWidth::W64 => {
                debug_assert!(bits <= 64);
                if bits > 64 {
                    return;
                }
                if bits == 64 || bits == 0 {
                    self.asm.nop()
                } else {
                    record(&mut self.error, self.asm.shl(r64(dst), 64 - bits));
                    self.asm.sar(r64(dst), 64 - bits)
                }
            }
            OpWidth::W32 => {
                debug_assert!(bits <= 32);
                if bits > 32 {
                    return;
                }
                if bits == 32 || bits == 0 {
                    self.asm.nop()
                } else {
                    record(&mut self.error, self.asm.shl(r32(dst), 32 - bits));
                    self.asm.sar(r32(dst), 32 - bits)
                }
            }
        };
        record(&mut self.error, r);
    }

    fn set_cmp(&mut self, dst: Tmp, cond: CmpCond, lhs: Tmp, rhs: Src, w: OpWidth, s: Signedness) {
        let r = self.cmp_flags(lhs, rhs, w);
        record(&mut self.error, r);

        let a = &mut self.asm;
        let d = reg8(dst);

        use CmpCond::*;
        use Signedness::*;
        let r = match (cond, s) {
            (Eq, _) => a.sete(d),
            (Ne, _) => a.setne(d),
            (Lt, Signed) => a.setl(d),
            (Lt, Unsigned) => a.setb(d),
            (Le, Signed) => a.setle(d),
            (Le, Unsigned) => a.setbe(d),
            (Gt, Signed) => a.setg(d),
            (Gt, Unsigned) => a.seta(d),
            (Ge, Signed) => a.setge(d),
            (Ge, Unsigned) => a.setae(d),
        };
        record(&mut self.error, r);
        record(&mut self.error, a.movzx(r32(dst), d));
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
        let r = self.cmp_flags(lhs, rhs, w);
        record(&mut self.error, r);

        let st = &mut self.labels[target.0 as usize];
        st.referenced = true;
        let l = st.iced;

        let a = &mut self.asm;

        use CmpCond::*;
        use Signedness::*;
        let r = match (cond, s) {
            (Eq, _) => a.je(l),
            (Ne, _) => a.jne(l),
            (Lt, Signed) => a.jl(l),
            (Lt, Unsigned) => a.jb(l),
            (Le, Signed) => a.jle(l),
            (Le, Unsigned) => a.jbe(l),
            (Gt, Signed) => a.jg(l),
            (Gt, Unsigned) => a.ja(l),
            (Ge, Signed) => a.jge(l),
            (Ge, Unsigned) => a.jae(l),
        };
        record(&mut self.error, r);
    }
}

/// ALU 64bit Register-Register helper func
fn alu_rr64(
    a: &mut CodeAssembler,
    op: AluOp,
    d: AsmRegister64,
    s: AsmRegister64,
) -> Result<(), IcedError> {
    match op {
        AluOp::Add => a.add(d, s),
        AluOp::Sub => a.sub(d, s),
        AluOp::Mul => a.imul_2(d, s),
        AluOp::And => a.and(d, s),
        AluOp::Or => a.or(d, s),
        AluOp::Xor => a.xor(d, s),
        AluOp::Div | AluOp::Mod => unreachable!("div/mod are emitted by X86Emitter::div_mod"),
    }
}

/// ALU 32bit Register-Register helper func
fn alu_rr32(
    a: &mut CodeAssembler,
    op: AluOp,
    d: AsmRegister32,
    s: AsmRegister32,
) -> Result<(), IcedError> {
    match op {
        AluOp::Add => a.add(d, s),
        AluOp::Sub => a.sub(d, s),
        AluOp::Mul => a.imul_2(d, s),
        AluOp::And => a.and(d, s),
        AluOp::Or => a.or(d, s),
        AluOp::Xor => a.xor(d, s),
        AluOp::Div | AluOp::Mod => unreachable!("div/mod are emitted by X86Emitter::div_mod"),
    }
}

/// ALU 64bit Register-Immediate helper func
fn alu_ri64(a: &mut CodeAssembler, op: AluOp, d: AsmRegister64, imm: i32) -> Result<(), IcedError> {
    match op {
        AluOp::Add => a.add(d, imm),
        AluOp::Sub => a.sub(d, imm),
        AluOp::Mul => a.imul_3(d, d, imm),
        AluOp::And => a.and(d, imm),
        AluOp::Or => a.or(d, imm),
        AluOp::Xor => a.xor(d, imm),
        AluOp::Div | AluOp::Mod => unreachable!("div/mod are emitted by X86Emitter::div_mod"),
    }
}

/// ALU 32bit Register-Immediate helper func
fn alu_ri32(a: &mut CodeAssembler, op: AluOp, d: AsmRegister32, imm: i32) -> Result<(), IcedError> {
    match op {
        AluOp::Add => a.add(d, imm),
        AluOp::Sub => a.sub(d, imm),
        AluOp::Mul => a.imul_3(d, d, imm),
        AluOp::And => a.and(d, imm),
        AluOp::Or => a.or(d, imm),
        AluOp::Xor => a.xor(d, imm),
        AluOp::Div | AluOp::Mod => unreachable!("div/mod are emitted by X86Emitter::div_mod"),
    }
}
