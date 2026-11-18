//! LLVM IR -> UMC translation, scoped to scalar int/float arithmetic for PolyBench/C
//!
//! Every SSA value gets its own UMC register (UMC register indices are unbounded), so there is no register allocation
//! Calling convention:
//!   n:0                      return address (each callee copies it to its own register)
//!   <class><width>:j         argument j, in the register set of its type
//!   <class><width>:0         return value
//!   index >= FIRST_LOCAL     values local to a function
//! bit fo a shortcut: register numbering is global and nothing is spilled so recursion is unsupported
//! direct recursion panics and mutual recursion miscompiles but PolyBench has none so it's fine for these purposes
use std::collections::HashMap;

use llvm_ir::constant::Float;
use llvm_ir::types::FPType;
use llvm_ir::{
    Constant, FPPredicate, Function, Instruction as LI, IntPredicate, Module, Name, Operand,
    Terminator, Type, instruction as li,
};
use umc_model::instructions::*;
use umc_model::reg_model::*;
use umc_model::{Program, RegIndex, RegWidth};

/// Registers below this idx reserved for args or ret values
const FIRST_LOCAL: RegIndex = 16;

/// UMC operand in one of the register classes LLVM values map to
#[derive(Debug, Clone, PartialEq)]
enum V {
    U(RegOrConstant<UnsignedRegT>),
    I(RegOrConstant<SignedRegT>),
    F(RegOrConstant<FloatRegT>),
    M(RegOrConstant<MemRegT>),
}

// macro for generating impl fns for each of the LLVM+UMC Value enums
macro_rules! accessors {
    ($val:ident, $reg:ident, $var:ident, $t:ty) => {
        fn $val(&self) -> RegOrConstant<$t> {
            match self {
                V::$var(x) => x.clone(),
                v => panic!("expected {} operand, got {v:?}", stringify!($var)),
            }
        }
        fn $reg(&self) -> Reg<$t> {
            match self.$val() {
                RegOrConstant::Reg(r) => r,
                c => panic!("expected register, got constant {c:?}"),
            }
        }
    };
}

impl V {
    accessors!(u, ru, U, UnsignedRegT);
    accessors!(i, ri, I, SignedRegT);
    accessors!(f, rf, F, FloatRegT);
    accessors!(m, rm, M, MemRegT);

    /// Register set (class, width) of register operand
    fn class(&self) -> (char, RegWidth) {
        match self {
            V::U(_) => ('u', self.ru().width),
            V::I(_) => ('i', self.ri().width),
            V::F(_) => ('f', self.rf().width),
            V::M(_) => ('m', 0),
        }
    }

    fn is_const(&self) -> bool {
        matches!(
            self,
            V::U(RegOrConstant::Const(_))
                | V::I(RegOrConstant::Const(_))
                | V::F(RegOrConstant::Const(_))
                | V::M(RegOrConstant::Const(_))
        )
    }

    fn any(&self) -> AnySingleReg {
        match self {
            V::U(_) => AnySingleReg::Unsigned(self.ru()),
            V::I(_) => AnySingleReg::Signed(self.ri()),
            V::F(_) => AnySingleReg::Float(self.rf()),
            V::M(_) => AnySingleReg::Mem(self.rm()),
        }
    }
}

// generate UMC reg of specified type & width
fn reg((class, width): (char, RegWidth), index: RegIndex) -> V {
    match class {
        'u' => V::U(RegOrConstant::Reg(Reg { index, width })),
        'i' => V::I(RegOrConstant::Reg(Reg { index, width })),
        'f' => V::F(RegOrConstant::Reg(Reg { index, width })),
        'm' => V::M(RegOrConstant::reg(index)),
        c => unreachable!("register class {c}"),
    }
}

/// LLVM ints are signless: i1 lives in u1 and wider ints are signed regs
fn class_of(ty: &Type) -> (char, RegWidth) {
    match ty {
        Type::IntegerType { bits: 1 } => ('u', 1),
        Type::IntegerType { bits } => ('i', *bits),
        Type::FPType(FPType::Double) => ('f', 64),
        Type::FPType(FPType::Single) => ('f', 32),
        Type::PointerType { .. } => ('m', 0),
        t => unimplemented!("type {t:?}"),
    }
}

// generate zero const for specified LLVM reg type
fn zero(ty: &Type) -> V {
    match class_of(ty).0 {
        'u' => V::U(RegOrConstant::Const(0)),
        'i' => V::I(RegOrConstant::Const(0)),
        'f' => V::F(RegOrConstant::Const(0.0)),
        _ => unimplemented!("null pointer constant"),
    }
}

// get size in bytes of type
// needed for GEP scaling & for allocas if needed (ideally avoid allocas with full SSA)
fn size_of(ty: &Type) -> u64 {
    match ty {
        Type::IntegerType { bits } => (*bits as u64).div_ceil(8),
        Type::FPType(FPType::Double) => 8,
        Type::FPType(FPType::Single) => 4,
        Type::ArrayType {
            element_type,
            num_elements,
        } => size_of(element_type) * *num_elements as u64,
        t => unimplemented!("size of {t:?}"),
    }
}

// convert LLVM constant to UMC immediate value
// LLVM integers converted to the correct length of signed UMC integer and shifted into place
//
// LLVM bools (i1) are converted to UMC u1's
// llvm-ir crate stores Int bits with the raw bits zero-extended, so i32 -1 would be `0xFFFF_FFFF` which wouldn't fit UMC's i64 directly
// So we get around this by doing a left shift and an arithmetic rightshift to sign-extend the constant
// LLVM float singles (f32) are widened to f64 since the UMC float const type is f64 only, `umc_model/src/reg_model.rs`
fn konst(c: &Constant) -> V {
    match c {
        Constant::Int { bits: 1, value } => V::U(RegOrConstant::Const(*value)),
        Constant::Int { bits, value } => {
            let s = 64 - bits;
            V::I(RegOrConstant::Const(((*value << s) as i64) >> s))
        }
        Constant::Float(Float::Double(x)) => V::F(RegOrConstant::Const(*x)),
        Constant::Float(Float::Single(x)) => V::F(RegOrConstant::Const(*x as f64)),
        Constant::Undef(t) | Constant::Poison(t) => zero(t),
        c => unimplemented!("constant {c:?}"),
    }
}

// wrapper to avoid a 5-way `MovParams` match at each mov call
fn mov(d: &V, s: V) -> Instruction {
    Instruction::Mov(match d {
        V::U(_) => MovParams::UnsignedInt(d.ru(), s.u()),
        V::I(_) => MovParams::SignedInt(d.ri(), s.i()),
        V::F(_) => MovParams::Float(d.rf(), s.f()),
        V::M(_) => MovParams::MemAddress(d.rm(), s.m()),
    })
}

// wrapper to avoid `NumOp` match for each call
fn num(d: &V, a: V, b: V) -> AnyConsistentNumOp {
    match d {
        V::U(_) => AnyConsistentNumOp::UnsignedInt(ConsistentOp::Single(d.ru(), a.u(), b.u())),
        V::I(_) => AnyConsistentNumOp::SignedInt(ConsistentOp::Single(d.ri(), a.i(), b.i())),
        V::F(_) => AnyConsistentNumOp::Float(ConsistentOp::Single(d.rf(), a.f(), b.f())),
        V::M(_) => panic!("arithmetic on a pointer"),
    }
}

// add has its own params type, since MemAddresses can be added, need to be handled correctly
fn add(d: &V, a: V, b: V) -> Instruction {
    Instruction::Add(match d {
        // memaddress case
        V::M(_) => AddParams::MemAddress(
            d.rm(),
            a.m(),
            match b {
                V::I(x) => OffsetOp::Signed(x),
                V::U(x) => OffsetOp::Unsigned(x),
                v => panic!("pointer offset {v:?}"),
            },
        ),
        _ => match num(d, a, b) {
            AnyConsistentNumOp::UnsignedInt(o) => AddParams::UnsignedInt(o),
            AnyConsistentNumOp::SignedInt(o) => AddParams::SignedInt(o),
            AnyConsistentNumOp::Float(o) => AddParams::Float(o),
        },
    })
}

// wrapper to avoid Comparison match at each cmp call
fn cmp_args(a: V, b: V) -> ConsistentComparison {
    match a {
        V::U(_) => ConsistentComparison::UnsignedCompare(a.u(), b.u()),
        V::I(_) => ConsistentComparison::SignedCompare(a.i(), b.i()),
        V::F(_) => ConsistentComparison::FloatCompare(a.f(), b.f()),
        V::M(_) => ConsistentComparison::MemAddressCompare(a.m(), b.m()),
    }
}

/// Converts function names into labels with alphanumeric names + `_` for any other characters
/// e.g. "gemm.matrix" -> "GEMM_MATRIX"
fn sanitise(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

// avoids depending on how Name's display prints (i.e. with or without %)
fn name_str(n: &Name) -> String {
    match n {
        Name::Name(s) => s.to_string(),
        Name::Number(n) => n.to_string(),
    }
}

/// Translator context
struct Tx<'m> {
    // the whole module, reference with explicit lifetime tied to module
    module: &'m Module,
    // current instruction list
    out: Vec<Instruction>,
    // labels -> index, becomes Program::instr_labels
    labels: HashMap<String, usize>,
    /// (instruction index, label) for branch targets patched once all labels are known
    fixups: Vec<(usize, String)>,
    // register counter per set (u1, u32, i32, etc.) for SSA
    next_reg: HashMap<(char, RegWidth), RegIndex>,
    // Per-function state
    fname: String,
    vals: HashMap<Name, V>,
    ret_addr: RegIndex,
}

/// Translate whole module into UMC program that runs `main`
pub fn translate(module: &Module) -> Program {
    let mut tx = Tx {
        module,
        out: vec![],
        labels: HashMap::new(),
        fixups: vec![],
        next_reg: HashMap::new(),
        fname: String::new(),
        vals: HashMap::new(),
        ret_addr: 0,
    };

    // Entry stub: give main's argc/argv defined values, call it then halt
    tx.label("ENTRY".into());
    let main = module.get_func_by_name("main").expect("module has no main");
    for (j, p) in main.parameters.iter().enumerate() {
        // generate reg of required class of arg, then zero it / alloc 8 bytes for memregs
        let r = reg(class_of(&p.ty), j as RegIndex);
        let i = match r {
            V::M(_) => Instruction::Alloc(r.rm(), RegOrConstant::Const(8)),
            _ => mov(&r, zero(&p.ty)),
        };
        tx.out.push(i);
    }
    // jal to main function and link ret address to n:0 as per calling convention
    tx.jump("FN_MAIN".into(), |t| {
        Instruction::Jal(t, Reg::from_index(0))
    });
    // jump to END to finish entry stub
    tx.jump("END".into(), Instruction::Jmp);

    // translate functions one by one
    for f in &module.functions {
        tx.function(f);
    }

    // finish with a nop
    tx.label("END".into());
    tx.out.push(Instruction::Nop);

    // for each fixup (forward branches without a known target index) go back through the program and patch up these jumps
    for (at, l) in &tx.fixups {
        // get REAL target index
        let target = *tx
            .labels
            .get(l)
            .unwrap_or_else(|| panic!("undefined label {l}"));
        // set jump target to REAL target index
        match &mut tx.out[*at] {
            Instruction::Jmp(t)
            | Instruction::Jal(t, _)
            | Instruction::Bz(t, _)
            | Instruction::Bnz(t, _) => *t = RegOrConstant::Const(target),
            i => unreachable!("fixup on {i:?}"),
        }
    }
    // generate final UMC program from generated instructions
    Program {
        instr_labels: tx.labels,
        ..Program::from_instrs(tx.out)
    }
}

impl<'m> Tx<'m> {
    // records label as name -> out.len() i.e. idx of the next instruction, and asserts the name is unused
    fn label(&mut self, l: String) {
        let prev = self.labels.insert(l.clone(), self.out.len());
        assert!(prev.is_none(), "duplicate label {l}");
    }

    // generates a block label in the format "FNAME__SANITISEDNAME"
    fn block_label(&self, n: &Name) -> String {
        format!("{}__{}", self.fname, sanitise(&name_str(n)))
    }

    /// emit a branch whose target is patched later, use placeholder target of usize::MAX
    /// put the program idx and label into fixups to be backpatched later
    fn jump(&mut self, label: String, mk: impl FnOnce(RegOrConstant<InstrRegT>) -> Instruction) {
        self.fixups.push((self.out.len(), label));
        self.out.push(mk(RegOrConstant::Const(usize::MAX)));
    }

    // allocates a fresh register, using the next available register for specified set (enforcing SSA)
    fn tmp(&mut self, set: (char, RegWidth)) -> V {
        let n = self.next_reg.entry(set).or_insert(FIRST_LOCAL);
        *n += 1;
        reg(set, *n - 1)
    }

    // converts LLVM val to our representation using enum V
    // consts go throug konst()
    fn val(&self, op: &Operand) -> V {
        match op {
            Operand::LocalOperand { name, .. } => self
                .vals
                .get(name)
                .unwrap_or_else(|| panic!("undefined value {name:?}"))
                .clone(),
            Operand::ConstantOperand(c) => konst(c),
            Operand::MetadataOperand => unimplemented!("metadata operand"),
        }
    }

    // get bits of LLVM operand
    fn bits(&self, op: &Operand) -> u32 {
        match &*self.module.type_of(op) {
            Type::IntegerType { bits } => *bits,
            t => panic!("expected integer, got {t:?}"),
        }
    }

    /// reinterpret integer as unsigned - llvm doesnt have a specific unsigned type
    fn as_u(&mut self, v: V, bits: u32) -> V {
        match v {
            // mask out higher bit in signed integers so that the value is preserved
            V::I(RegOrConstant::Const(c)) => {
                let mask = if bits >= 64 {
                    u64::MAX
                } else {
                    (1 << bits) - 1
                };
                V::U(RegOrConstant::Const(c as u64 & mask))
            }
            // push an ignoresigned cast in the case of this being applied to a register
            V::I(RegOrConstant::Reg(r)) => {
                let t = self.tmp(('u', r.width));
                let cast = IntegerCast::try_create(t.ru(), RegOrConstant::Reg(r)).unwrap();
                self.out
                    .push(Instruction::Cast(SimpleCast::IgnoreSigned(cast)));
                t
            }
            v => v,
        }
    }

    /// Emit an instruction that gives an unsigned result for `d` via a u temp + `AddSign` when `d` is signed
    /// since this takes an FnOnce closure we can use this to cover udiv, urem, zext, and fptoui, and if d is already in U we just apply the closure directly to it
    fn via_u(&mut self, d: &V, mk: impl FnOnce(&V) -> Instruction) {
        if let V::U(_) = d {
            self.out.push(mk(d));
            return;
        }
        let t = self.tmp(('u', d.class().1));
        self.out.push(mk(&t));
        let cast = IntegerCast::try_create(d.ri(), t.u()).unwrap();
        self.out.push(Instruction::Cast(SimpleCast::AddSign(cast)));
    }

    // makes sure an operand is in a register, copying a constant in if needed
    // needed for store's value since UMC doesn't do stores for arbitrary width constants
    fn in_reg(&mut self, op: &Operand) -> V {
        let v = self.val(op);
        if !v.is_const() {
            return v;
        }
        let t = self.tmp(class_of(&self.module.type_of(op)));
        self.out.push(mov(&t, v));
        t
    }

    // UMC doesn't have a conditional move, so we use the following format:
    //     mov d, f
    //     bz <here+2>, c
    //     mov d, t
    // This is also used for smax and smin
    fn select(&mut self, d: &V, c: RegOrConstant<UnsignedRegT>, t: V, f: V) {
        self.out.push(mov(d, f));
        let skip = self.out.len() + 2;
        self.out.push(Instruction::Bz(
            RegOrConstant::Const(skip),
            CompareToZero::Unsigned(c),
        ));
        self.out.push(mov(d, t));
    }

    /// Translate a whole function:
    /// 1. Reset per-function state and emit label FN_<NAME>
    /// 2. Assert < 16 parameters, arg registers must be below FIRST_LOCAL
    /// 3. Param prologue: mov <fresh>, <class>:j for each param, record fresh register in vals, local copy is needed as nested calls would overwrite the arg registers
    /// 4. Save the return address `mov n:<fresh>, n:0`, increment instr reg counter
    /// 5. Pre-allocate result regs across all function blocks, try_get_result() returns None for instrs with no result
    /// 6. Emit each block: label, then each instruction through instr, then its terminator through term passing in next blocks name for fall-through
    fn function(&mut self, f: &Function) {
        self.fname = sanitise(&f.name);
        self.vals.clear();
        self.label(format!("FN_{}", self.fname));

        // make sure < 16 params
        assert!(
            f.parameters.len() < FIRST_LOCAL as usize,
            "too many parameters in {}",
            f.name
        );
        // mov in function params
        for (j, p) in f.parameters.iter().enumerate() {
            let set = class_of(&p.ty);
            let r = self.tmp(set);
            self.out.push(mov(&r, reg(set, j as RegIndex)));
            self.vals.insert(p.name.clone(), r);
        }
        // save return address in next available n register
        let n = self.next_reg.entry(('n', 0)).or_insert(FIRST_LOCAL);
        self.ret_addr = *n;
        *n += 1;
        self.out.push(Instruction::Mov(MovParams::InstrAddress(
            Reg::from_index(self.ret_addr),
            RegOrConstant::reg(0),
        )));

        // Allocate every result up front: phi copies and non-layout-order uses need them early
        for bb in &f.basic_blocks {
            for inst in &bb.instrs {
                if let Some(n) = inst.try_get_result() {
                    // add result values of each basic block to fresh registers
                    let r = self.tmp(class_of(&self.module.type_of(inst)));
                    self.vals.insert(n.clone(), r);
                }
            }
        }

        for (k, bb) in f.basic_blocks.iter().enumerate() {
            let next = f.basic_blocks.get(k + 1).map(|b| &b.name);
            // generate BB label with sanitised format
            self.label(self.block_label(&bb.name));
            for inst in &bb.instrs {
                // emit instructions
                self.instr(f, inst);
            }
            // emit terminator
            self.term(f, &bb.name, &bb.term, next);
        }
    }

    // Terminators
    // ret: mov to <class>:0 if theres a return value, then jmp n:<ret_addr>
    // br: phi copies for current edge, then jmp unless the target is the next block (just fallthrough in that case)
    // Conditional branch:
    //    The condition must be unsigned for bnz
    //    has_phis() decides if a trampoline is needed, checks only first expression since LLVM groups phis at top of block
    //    Trampoline is needed when an edge's source has multiple successors and its target has multiple prececessors
    //    e.g. br i1 %c, label %loop, label %exit - the copies for loop cant go before the branch because they also would before the exit branch
    //    so we'll use a trampoline section which will act as a false target for one of the possible branch targets, and split up the phi allocation
    //      - without trampoline: bnz TRUE, false-edge copies, then jmp FALSE unless thats next block
    //      - with trampoline: jmp FALSE is always emitted, because trampoline comes next not false block
    //    unreachable emits nothing
    //    anythign else is currently unimplmented
    fn term(&mut self, f: &Function, from: &Name, term: &Terminator, next: Option<&Name>) {
        match term {
            Terminator::Ret(r) => {
                // if we have ret value, mov it into <class>:0
                if let Some(op) = &r.return_operand {
                    let v = self.val(op);
                    let ret = reg(class_of(&self.module.type_of(op)), 0);
                    self.out.push(mov(&ret, v));
                }
                // jump to n:<ret_addr>
                self.out
                    .push(Instruction::Jmp(RegOrConstant::reg(self.ret_addr)));
            }
            Terminator::Br(b) => {
                self.phi_copies(f, from, &b.dest);
                // jump to next block if it isn't the following block written in program (just fall through)
                if Some(&b.dest) != next {
                    self.jump(self.block_label(&b.dest), Instruction::Jmp);
                }
            }
            Terminator::CondBr(b) => {
                let c = self.val(&b.condition).u();
                // Critical edge: the true edge's phi copies go in trampoline after false path
                let tramp = has_phis(f, &b.true_dest);
                let tramp_label = format!("{}_T", self.block_label(from));
                let target = if tramp {
                    tramp_label.clone()
                } else {
                    self.block_label(&b.true_dest)
                };
                self.jump(target, |t| Instruction::Bnz(t, CompareToZero::Unsigned(c)));

                // implement phi copy movs for false block
                self.phi_copies(f, from, &b.false_dest);
                // jump to false dest if it isnt next block
                if tramp || Some(&b.false_dest) != next {
                    self.jump(self.block_label(&b.false_dest), Instruction::Jmp);
                }
                // implement phi copy movs for true dest and jump if it isnt next block
                if tramp {
                    self.label(tramp_label);
                    self.phi_copies(f, from, &b.true_dest);
                    if Some(&b.true_dest) != next {
                        self.jump(self.block_label(&b.true_dest), Instruction::Jmp);
                    }
                }
            }
            Terminator::Unreachable(_) => {}
            t => unimplemented!("terminator {t:?}"),
        }
    }

    /// Movs implementing phis of `to` for edge `from -> to`
    fn phi_copies(&mut self, f: &Function, from: &Name, to: &Name) {
        let bb = f.get_bb_by_name(to).unwrap();
        // collect phi sources from LI::Phi instructions in this basic block
        let copies: Vec<(V, V)> = bb
            .instrs
            .iter()
            // find phi nodes
            .filter_map(|i| match i {
                LI::Phi(p) => Some(p),
                _ => None,
            })
            // find phi nodes which have `from` block as a source
            .map(|p| {
                let (src, _) = p.incoming_values.iter().find(|(_, b)| b == from).unwrap();
                (self.vals[&p.dest].clone(), self.val(src))
            })
            .collect();

        // Phi reading another phi of the same block must see its old value: stage through temps
        // avoid a p = q, q = p situation ending up with both = q
        let clash = copies
            .iter()
            .any(|(_, s)| copies.iter().any(|(d, _)| d == s));
        if !clash {
            for (d, s) in copies {
                self.out.push(mov(&d, s));
            }
            return;
        }
        // if we have clashes, use tmp registers as a stage to avoid aliasing
        let staged: Vec<(V, V)> = copies
            .into_iter()
            .map(|(d, s)| {
                let t = self.tmp(d.class());
                self.out.push(mov(&t, s));
                (d, t)
            })
            .collect();
        for (d, t) in staged {
            self.out.push(mov(&d, t));
        }
    }

    // massive match statement for templating LLVM instructions to UMC instructions
    fn instr(&mut self, f: &Function, inst: &LI) {
        // op! macro for "dest = operand0 OP operand1" pattern with numerical operations
        macro_rules! op {
            ($s:ident, $x:expr, $ctor:path) => {{
                let (d, a, b) = (
                    $s.vals[&$x.dest].clone(),
                    $s.val(&$x.operand0),
                    $s.val(&$x.operand1),
                );
                $s.out.push($ctor(num(&d, a, b)));
            }};
        }
        // dst closure gets an already allocated dest register but keeps &Self borrow short to appease borrow checker
        let dst = |s: &Self, n: &Name| s.vals[n].clone();

        match inst {
            LI::Add(x) => {
                let (d, a, b) = (
                    dst(self, &x.dest),
                    self.val(&x.operand0),
                    self.val(&x.operand1),
                );
                self.out.push(add(&d, a, b));
            }
            LI::FAdd(x) => {
                let (d, a, b) = (
                    dst(self, &x.dest),
                    self.val(&x.operand0),
                    self.val(&x.operand1),
                );
                self.out.push(add(&d, a, b));
            }
            LI::Sub(x) => op!(self, x, Instruction::Sub),
            LI::FSub(x) => op!(self, x, Instruction::Sub),
            LI::Mul(x) => op!(self, x, Instruction::Mul),
            LI::FMul(x) => op!(self, x, Instruction::Mul),
            LI::SDiv(x) => op!(self, x, Instruction::Div),
            LI::FDiv(x) => op!(self, x, Instruction::Div),
            LI::SRem(x) => op!(self, x, Instruction::Mod),
            LI::FRem(x) => op!(self, x, Instruction::Mod),
            LI::And(x) => op!(self, x, Instruction::And),
            LI::Or(x) => op!(self, x, Instruction::Or),
            LI::Xor(x) => op!(self, x, Instruction::Xor),
            LI::UDiv(x) => self.unsigned_op(&x.dest, &x.operand0, &x.operand1, Instruction::Div),
            LI::URem(x) => self.unsigned_op(&x.dest, &x.operand0, &x.operand1, Instruction::Mod),
            LI::Shl(x) => self.shift(&x.dest, &x.operand0, &x.operand1, ShiftType::LeftShift),
            LI::LShr(x) => self.shift(
                &x.dest,
                &x.operand0,
                &x.operand1,
                ShiftType::RightLogicalShift,
            ),
            LI::AShr(x) => self.shift(
                &x.dest,
                &x.operand0,
                &x.operand1,
                ShiftType::RightArithmeticShift,
            ),
            LI::FNeg(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                self.out.push(Instruction::Mul(num(
                    &d,
                    a,
                    V::F(RegOrConstant::Const(-1.0)),
                )));
            }

            // Comparison arms
            LI::ICmp(x) => {
                use IntPredicate::*;
                let (d, mut a, mut b) = (
                    dst(self, &x.dest).ru(),
                    self.val(&x.operand0),
                    self.val(&x.operand1),
                );
                let cond = match x.predicate {
                    EQ => BinaryCondition::Equal,
                    NE => BinaryCondition::NotEqual,
                    UGT | SGT => BinaryCondition::GreaterThan,
                    UGE | SGE => BinaryCondition::GreaterThanOrEqualTo,
                    ULT | SLT => BinaryCondition::LessThan,
                    ULE | SLE => BinaryCondition::LessThanOrEqualTo,
                };
                if matches!(x.predicate, UGT | UGE | ULT | ULE) {
                    let bits = self.bits(&x.operand0);
                    a = self.as_u(a, bits);
                    b = self.as_u(b, bits);
                }
                self.out.push(Instruction::Compare {
                    cond,
                    params: CompareParams {
                        dst: d,
                        args: cmp_args(a, b),
                    },
                });
            }
            LI::FCmp(x) => {
                use FPPredicate::*;
                let (d, a, b) = (
                    dst(self, &x.dest).ru(),
                    self.val(&x.operand0),
                    self.val(&x.operand1),
                );
                // shortcut just for polybench: ordered and unordered predicates treated the same
                let cond = match x.predicate {
                    OEQ | UEQ => BinaryCondition::Equal,
                    ONE | UNE => BinaryCondition::NotEqual,
                    OGT | UGT => BinaryCondition::GreaterThan,
                    OGE | UGE => BinaryCondition::GreaterThanOrEqualTo,
                    OLT | ULT => BinaryCondition::LessThan,
                    OLE | ULE => BinaryCondition::LessThanOrEqualTo,
                    p => unimplemented!("fcmp {p:?}"),
                };
                self.out.push(Instruction::Compare {
                    cond,
                    params: CompareParams {
                        dst: d,
                        args: cmp_args(a, b),
                    },
                });
            }
            LI::Select(x) => {
                let d = dst(self, &x.dest);
                let (c, t, f) = (
                    self.val(&x.condition).u(),
                    self.val(&x.true_value),
                    self.val(&x.false_value),
                );
                self.select(&d, c, t, f);
            }
            LI::Phi(_) => {} // lowered to copies in predecessors

            LI::Trunc(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                let cast = match d {
                    V::U(_) => {
                        let a = self.as_u(a, self.bits(&x.operand));
                        ResizeCast::Unsigned(d.ru(), a.u())
                    }
                    _ => ResizeCast::Signed(d.ri(), a.i()),
                };
                self.out.push(Instruction::Cast(SimpleCast::Resize(cast)));
            }
            LI::ZExt(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                let a = self.as_u(a, self.bits(&x.operand));
                self.via_u(&d, |t| {
                    Instruction::Cast(SimpleCast::Resize(ResizeCast::Unsigned(t.ru(), a.u())))
                });
            }
            LI::SExt(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                self.out
                    .push(Instruction::Cast(SimpleCast::Resize(ResizeCast::Signed(
                        d.ri(),
                        a.i(),
                    ))));
            }
            LI::FPExt(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                self.out
                    .push(Instruction::Cast(SimpleCast::Resize(ResizeCast::Float(
                        d.rf(),
                        a.f(),
                    ))));
            }
            LI::FPTrunc(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                self.out
                    .push(Instruction::Cast(SimpleCast::Resize(ResizeCast::Float(
                        d.rf(),
                        a.f(),
                    ))));
            }
            LI::SIToFP(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                let cast = IntegerFloatCast::try_create(d.rf(), a.i()).unwrap();
                self.out.push(Instruction::Cast(SimpleCast::IBuoy(cast)));
            }
            LI::UIToFP(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                let a = self.as_u(a, self.bits(&x.operand));
                let cast = IntegerFloatCast::try_create(d.rf(), a.u()).unwrap();
                self.out.push(Instruction::Cast(SimpleCast::UBuoy(cast)));
            }
            LI::FPToSI(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                let cast = IntegerFloatCast::try_create(d.ri(), a.f()).unwrap();
                self.out.push(Instruction::Cast(SimpleCast::ISink(cast)));
            }
            LI::FPToUI(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                self.via_u(&d, |t| {
                    Instruction::Cast(SimpleCast::USink(
                        IntegerFloatCast::try_create(t.ru(), a.f()).unwrap(),
                    ))
                });
            }
            // used to stop propagation of undef or posion values, so just mov since there is no poison at runtime
            LI::Freeze(x) => {
                let (d, a) = (dst(self, &x.dest), self.val(&x.operand));
                self.out.push(mov(&d, a));
            }

            // Memory arms
            LI::Alloca(x) => {
                // shortcut: allocas are heap-allocated and never freed so fine while they stay out of loops (-O2)
                // most will be removed with -O2 optimisation
                let n = match self.val(&x.num_elements) {
                    V::I(RegOrConstant::Const(n)) => n as u64,
                    v => unimplemented!("dynamic alloca {v:?}"),
                };
                let d = dst(self, &x.dest);
                let size = RegOrConstant::Const(size_of(&x.allocated_type) * n);
                self.out.push(Instruction::Alloc(d.rm(), size));
            }
            LI::Load(x) => {
                // shortcut: loads from globals (e.g. `@stderr`) are skipped and leave the value undefined,
                // valid only while it just feeds a printf, real global data needs UMC mem labels
                if let Operand::ConstantOperand(c) = &x.address
                    && matches!(**c, Constant::GlobalReference { .. })
                {
                    return;
                }
                let (d, a) = (dst(self, &x.dest), self.val(&x.address));
                self.out.push(Instruction::Load(d.any(), a.m()));
            }
            LI::Store(x) => {
                let (a, v) = (self.val(&x.address), self.in_reg(&x.value));
                self.out.push(Instruction::Store(a.m(), v.any()));
            }
            LI::GetElementPtr(x) => self.gep(x),
            LI::Call(x) => self.call(f, x),
            i => unimplemented!("instruction {i:?}"),
        }
    }

    // convert regs to unsigned and convert to instruction
    fn unsigned_op(
        &mut self,
        dest: &Name,
        a: &Operand,
        b: &Operand,
        ctor: fn(AnyConsistentNumOp) -> Instruction,
    ) {
        let (d, bits) = (self.vals[dest].clone(), self.bits(a));
        let (a, b) = (self.val(a), self.val(b));
        let (a, b) = (self.as_u(a, bits), self.as_u(b, bits));
        self.via_u(&d, |t| ctor(num(t, a, b)));
    }

    // wrapper for shifts of target size & type
    fn shift(&mut self, dest: &Name, a: &Operand, b: &Operand, kind: ShiftType) {
        let (d, bits) = (self.vals[dest].clone(), self.bits(a));
        let (a, b) = (self.val(a), self.val(b));
        let size = self.as_u(b, bits).u();
        self.out.push(mov(&d, a));
        self.out
            .push(Instruction::Shift(ShiftParams { dst: d.any(), size }, kind));
    }

    // Followign LLVM's definition of GEP
    fn gep(&mut self, x: &li::GetElementPtr) {
        let d = self.vals[&x.dest].clone();
        let mut base = self.val(&x.address); // base address
        let mut ty = x.source_element_type.clone(); // source element type
        let mut off: i64 = 0; // constant part
        for (k, idx) in x.indices.iter().enumerate() {
            if k > 0 {
                // go 1 level into array or vector
                ty = match &*ty {
                    Type::ArrayType { element_type, .. }
                    | Type::VectorType { element_type, .. } => element_type.clone(),
                    t => unimplemented!("GEP into {t:?}"),
                };
            }
            let scale = size_of(&ty) as i64;
            match self.val(idx) {
                V::I(RegOrConstant::Const(c)) => off += c * scale, // compile-time if constant index
                V::I(RegOrConstant::Reg(r)) => {
                    let mut ix = V::I(RegOrConstant::Reg(r.clone()));
                    // if narrower than 64bit resize to 64bit
                    if r.width != 64 {
                        let t = self.tmp(('i', 64));
                        self.out
                            .push(Instruction::Cast(SimpleCast::Resize(ResizeCast::Signed(
                                t.ri(),
                                ix.i(),
                            ))));
                        ix = t;
                    }
                    // scale index if != 1
                    if scale != 1 {
                        let t = self.tmp(('i', 64));
                        self.out.push(Instruction::Mul(num(
                            &t,
                            ix,
                            V::I(RegOrConstant::Const(scale)),
                        )));
                        ix = t;
                    }
                    // pointer add
                    self.out.push(add(&d, base, ix));
                    // later terms add onto base = d
                    base = d.clone();
                }
                v => unimplemented!("GEP index {v:?}"),
            }
        }
        if off != 0 {
            // add onto scaled base
            self.out
                .push(add(&d, base, V::I(RegOrConstant::Const(off))));
        } else if base != d {
            // all-constant zero offsets is just a pure copy
            self.out.push(mov(&d, base));
        }
    }

    fn call(&mut self, f: &Function, x: &li::Call) {
        // get callee register from Either<InlineAssembly, Function>
        let name = match x.function.as_ref().right() {
            // only direct calls for now
            Some(Operand::ConstantOperand(c)) => match &**c {
                Constant::GlobalReference { name, .. } => name_str(name),
                c => unimplemented!("call to {c:?}"),
            },
            _ => unimplemented!("indirect call"),
        };
        if name == "printf" || name == "fprintf" {
            // shortcut for polybench: only scalar register args are printed
            // polybench doesn't use any vector arithmetic etc so we'll just avoid it for this dissertation scope
            // handled before collecting args since the format string is a global (`@.str`) that konst() can't translate
            for (op, _) in &x.arguments {
                if let Operand::LocalOperand { .. } = op {
                    let a = self.val(op);
                    if !a.is_const() && !matches!(a, V::M(_)) {
                        self.out.push(Instruction::Dbg(AnyReg::Single(a.any())));
                    }
                }
            }
            return;
        }
        // collect args and ret val destination
        let args: Vec<V> = x.arguments.iter().map(|(a, _)| self.val(a)).collect();
        let d = x.dest.as_ref().map(|n| self.vals[n].clone());
        let n = name.as_str();

        // nothing for now for these
        if n.starts_with("llvm.lifetime")
            || n.starts_with("llvm.dbg")
            || n.starts_with("polybench_timer")
        {
            // fused mul and add -> just replace with a mul then add
        } else if n.starts_with("llvm.fmuladd") {
            let d = d.unwrap();
            let [a, b, c] = args.try_into().unwrap();
            self.out.push(Instruction::Mul(num(&d, a, b)));
            self.out.push(add(&d, d.clone(), c));
        } else if n.starts_with("llvm.sqrt") {
            let d = d.unwrap();
            // mov into dst reg, then call native UMC sqrt
            self.out.push(mov(&d, args[0].clone()));
            self.out.push(Instruction::Sqrt(d.any()));
        } else if n.starts_with("llvm.smax") || n.starts_with("llvm.smin") {
            // return the larger / smaller value while comparing as signed values
            let d = d.unwrap();
            let [a, b] = args.try_into().unwrap();
            let c = self.tmp(('u', 1));
            // select correct comparison condition
            let cond = if n.starts_with("llvm.smax") {
                BinaryCondition::GreaterThan
            } else {
                BinaryCondition::LessThan
            };
            let args = cmp_args(a.clone(), b.clone());
            self.out.push(Instruction::Compare {
                cond,
                params: CompareParams { dst: c.ru(), args },
            });
            // select correct value
            self.select(&d, c.u(), a, b);
        } else if n == "malloc" {
            // use UMC alloc
            let size = self.as_u(args[0].clone(), 64).u();
            self.out.push(Instruction::Alloc(d.unwrap().rm(), size));
        } else if n == "polybench_alloc_data" {
            // polybench_alloc_data(n, elt_size) allocates n * elt_size bytes
            // defined in polybench_repo/utilities/polybench.c 557-569
            //
            // By recognising this by name it means we dont have to link polybench.c into the llvm output for the polybench benchmarks
            let [count, elt] = args.try_into().unwrap();
            let elt = match elt {
                // if non 64-bit reg is provided resize to 64 in tmp register
                V::I(RegOrConstant::Reg(r)) if r.width != 64 => {
                    let t = self.tmp(('i', 64));
                    self.out
                        .push(Instruction::Cast(SimpleCast::Resize(ResizeCast::Signed(
                            t.ri(),
                            RegOrConstant::Reg(r),
                        ))));
                    t
                }
                e => e,
            };
            let bytes = self.tmp(('i', 64));
            self.out.push(Instruction::Mul(num(&bytes, count, elt)));
            let size = self.as_u(bytes, 64).u();
            self.out.push(Instruction::Alloc(d.unwrap().rm(), size));
        } else if n == "free" {
            // very simple - just free target mem reg
            self.out.push(Instruction::Free(args[0].rm()));
        } else if self.module.get_func_by_name(n).is_some() {
            // avoid recursion for now because of the shortcuts we've taken in callee/caller register assignment
            // luckily polybench doesn't utilise recursion, so again just avoid for this dissertation scope
            assert_ne!(n, f.name, "recursion is unsupported");
            // mov arguments for call into correct regs
            for (j, ((op, _), a)) in x.arguments.iter().zip(args).enumerate() {
                let arg = reg(class_of(&self.module.type_of(op)), j as RegIndex);
                self.out.push(mov(&arg, a));
            }
            // jump to function
            self.jump(format!("FN_{}", sanitise(n)), |t| {
                Instruction::Jal(t, Reg::from_index(0))
            });
            // return
            if let Some(d) = d {
                let ret = reg(d.class(), 0);
                self.out.push(mov(&d, ret));
            }
        } else {
            // will likely need to implement exp and pow later for certain benchmarks
            unimplemented!("call to external function {n}");
        }
    }
}

// check if basic block has phi nodes
fn has_phis(f: &Function, block: &Name) -> bool {
    let bb = f.get_bb_by_name(block).unwrap();
    matches!(bb.instrs.first(), Some(LI::Phi(_)))
}
