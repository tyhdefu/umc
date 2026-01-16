//! End to end tests for the UMC VM
use std::io::Cursor;

use umc_compiler::error_display::assemble_prog;
use umc_model::binary::{decode, encode};

use super::*;

fn compile_and_run(s: &str) -> VirtualMachine {
    let prog = assemble_prog(s).unwrap();
    let mut vm = VirtualMachine::new(prog);
    vm.execute();
    vm
}

#[test]
fn test_add() {
    const PROG: &str = "
        mov u32:0, #100
        dbg u32:0
        add u32:0, u32:0, #10
        dbg u32:0
    ";
    let vm = compile_and_run(PROG);
    let v: u32 = vm.inspect_uint(0, u32::BITS);
    assert_eq!(v, 110);
}

#[test]
fn expand_not() {
    const PROG: &str = "
        not u64:0, #1
        dbg u64:0

        mov u32:0, #1
        not u64:1, u32:0
        dbg u64:0
    ";
    let vm = compile_and_run(PROG);
    let v: u64 = vm.inspect_uint(0, u64::BITS);
    assert_eq!(v, u64::MAX - 1);
}

#[test]
fn non_native_int() {
    const PROG: &str = "
        mov u5:1, #2
        add u5:1, u5:1, #3
        dbg u5:1
    ";
    let vm = compile_and_run(PROG);
    let v: u32 = vm.inspect_uint(1, 5);
    assert_eq!(v, 5);
}

#[test]
fn signed_sub() {
    const PROG: &str = "
        sub i32:0, #0, #5
        dbg i32:0

        sub i32:1, #100, i32:0
        dbg i32:1
    ";
    let vm = compile_and_run(PROG);
    let i32_0: i32 = vm.inspect_int(0, i32::BITS);
    assert_eq!(i32_0, -5);
    let i32_1: i32 = vm.inspect_int(1, i32::BITS);
    assert_eq!(i32_1, 105);
}

#[test]
fn overflow_uint() {
    const PROG: &str = "
        not u32:0, #0
        add u32:0, u32:0, #1
        dbg u32:0

        not u64:0, #0
        add u64:0, u64:0, #4
        dbg u64:0

        add u3:0, #5, #5
        dbg u3:0

        add u5:0, 0b11101, 0b00010
        dbg u5:0
    ";
    let vm = compile_and_run(PROG);
    let u64_0: u64 = vm.inspect_uint(0, u64::BITS);
    assert_eq!(u64_0, 3);
    let u3_0: u32 = vm.inspect_uint(0, 3);
    assert_eq!(u3_0, (5 + 5) & 0b111);
    let u5_0: u32 = vm.inspect_uint(0, 5);
    assert_eq!(u5_0, u32::pow(2, 5) - 1);
}

#[test]
fn test_jmp() {
    const PROG: &str = "
        mov u32:0, #100
        jmp .END
        mov u32:0, #3 ; should be skipped

        .END:
            dbg u32:0
    ";
    let vm = compile_and_run(PROG);
    let u32_0: u32 = vm.inspect_uint(0, u32::BITS);
    assert_eq!(u32_0, 100);
}

#[test]
fn test_indirect_jmp() {
    const PROG: &str = "
        mov u32:0, #100
        mov n:0, .END
        jmp n:0
        mov u32:0, #3 ; should be skipped

        .END:
        	dbg u32:0
    ";
    let vm = compile_and_run(PROG);
    let u32_0: u32 = vm.inspect_uint(0, u32::BITS);
    assert_eq!(u32_0, 100);
}

#[test]
fn test_huge_int() {
    const PROG: &str = "
        not u64:0, #0
        add u65:0, u64:0, u64:0
        dbg u65:0
    ";
    let vm = compile_and_run(PROG);
    let v: ArbitraryUnsignedInt = vm.inspect_uint(0, 65);
    let s = format!("{}", v);
    assert_eq!(s, format!("0x1{:X}", u64::MAX - 1))
}

#[test]
fn widening_add() {
    const PROG: &str = "
        mov u32:0, #10
        mov u64:0, #1
        add u64:1, u64:0, u32:0
        dbg u64:1
    ";
    let vm = compile_and_run(PROG);
    let v: u64 = vm.inspect_uint(1, u64::BITS);
    assert_eq!(v, 11);
}

#[test]
fn compare_uints() {
    const PROG: &str = "
        mov u32:0, #5
        mov u32:1, #0

        gt u32:3, u32:0, u32:1
        dbg u32:3

        le u1:0, u32:5, u32:5
        dbg u1:0

        lt u1:1, #10, u32:0
        dbg u1:1
    ";
    let vm = compile_and_run(PROG);
    let u32_3: u32 = vm.inspect_uint(3, u32::BITS);
    assert_eq!(u32_3, 1);

    let u1_0: u32 = vm.inspect_uint(0, 1);
    assert_eq!(u1_0, 1);

    let u1_1: u32 = vm.inspect_uint(1, 1);
    assert_eq!(u1_1, 0)
}

#[test]
fn add_floats() {
    const PROG: &str = "
        mov f64:0, #0.5
        dbg f64:0
        mov f64:1, #1.5
        dbg f64:1

        add f64:2, f64:0, f64:1
        dbg f64:2
    ";
    let vm = compile_and_run(PROG);
    let f64_2: f64 = vm.inspect_float(2, 64);
    assert_eq!(2.0f64, f64_2);
}

#[test]
fn unsigned_vector() {
    const PROG: &str = "
        dbg u32x6:0
    ";
    let _ = compile_and_run(PROG);
}

#[test]
fn fib_encode_and_decode() {
    // 1, 1, 2, 3, 5, 8, 13
    const PROG: &str = "
.FIB:
  mov u64:0, #7
  bz .END, u64:0

  mov u64:1, #0 ; int cur = 0
  mov u64:2, #1 ; int next = 1
  mov u64:3, #0 ; int i = 0

.LOOP: ; u64:3 next
  mov u64:4, u64:2        ; int next2 = next
  add u64:4, u64:4, u64:1 ; next2 += cur
  mov u64:1, u64:2        ; cur = next
  mov u64:2, u64:4        ; next = next2
  add u64:3, u64:3, #1

  dbg u64:3
  dbg u64:0

  xor u64:8, u64:3, u64:0 ; Compare :3 and :0
  bnz .LOOP, u64:8
.END:
  dbg u64:1
";
    const FIB_7: u64 = 13;
    let prog = assemble_prog(PROG).expect("Failed to assemble program");

    let mut vm = VirtualMachine::new(prog.clone());

    vm.execute();
    let u64_1 = vm.inspect_uint(1, u64::BITS);
    assert_eq!(FIB_7, u64_1, "7th fibonacci number");

    let mut buffer = vec![];
    encode(&prog, &mut buffer).expect("Failed to encode fibonacci program");

    let mut cursor = Cursor::new(buffer);
    let decoded_prog = decode(&mut cursor).expect("Failed to decode program");

    assert_eq!(
        prog.instructions, decoded_prog.instructions,
        "Decoded fibonacci did not match encoded program\n{}\n-----\n{}",
        prog, decoded_prog
    );

    let mut vm = VirtualMachine::new(decoded_prog);
    vm.execute();
    let u64_1 = vm.inspect_uint(1, u64::BITS);
    assert_eq!(FIB_7, u64_1);
}
