//! End to end tests for the UMC VM
use std::io::Cursor;

use umc_compiler::error_display::assemble_prog;
use umc_model::binary::{decode, encode};

use crate::vm::types::uint::ArbitraryUnsignedInt;

use super::*;

fn compile_and_run(s: &str) -> VirtualMachine {
    let prog = assemble_prog(s).unwrap();
    let mut vm = VirtualMachine::create(prog, VMOptions::vm_debug()).unwrap();
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

        sgt u32:3, u32:0, u32:1
        dbg u32:3

        sle u1:0, u32:5, u32:5
        dbg u1:0

        slt u1:1, #10, u32:0
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
        ; abuse the default value for now
        add u32x6:0, u32x6:0, #10
        ; Double
        add u32x6:0, u32x6:0, u32x6:0
    ";
    let vm = compile_and_run(PROG);
    assert_eq!(
        vec![20u32; 6],
        vm.inspect_uint_vec(0, u32::BITS, 6).unwrap()
    );
}

#[test]
fn basic_load_store_u32() {
    const PROG: &str = "
        alloc m:0, #4
        mov u32:5, #121
        store m:0, u32:5
        load u32:0, m:0
        free m:0

        dbg u32:0
    ";
    let vm = compile_and_run(PROG);
    assert_eq!(121u32, vm.inspect_uint(0, u32::BITS));
}

#[test]
fn nat_numbers_array() {
    const PROG: &str = "
        mul u64:0, #4, #10
        alloc m:0, u64:0
        mov m:1, m:0

        ; Counter
        mov u32:0, #0
.FILL_LOOP:
        dbg m:1
        store m:1, u32:0
        add m:1, m:1, #4
        add u32:0, u32:0, #1
        dbg u32:0
        sge u1:0, u32:0, #10
        bz .FILL_LOOP, u1:0

        ; Get index 5
        mul i32:0, #5, #4
        add m:1, m:0, i32:0
        dbg m:1
        load u32:2, m:1

        dbg u32:2

        free m:0
    ";

    let vm = compile_and_run(PROG);
    assert_eq!(5u32, vm.inspect_uint(2, u32::BITS));
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

    let mut vm = VirtualMachine::create(prog.clone(), VMOptions::vm_debug()).unwrap();

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

    let mut vm = VirtualMachine::create(decoded_prog, VMOptions::vm_debug()).unwrap();
    vm.execute();
    let u64_1 = vm.inspect_uint(1, u64::BITS);
    assert_eq!(FIB_7, u64_1);
}

#[test]
fn jump_and_link() {
    const PROG: &str = "
        mov u32:1, #300
        jal .FUNC, n:0 ; If this doesn't return, the add won't run
        add u32:1, u32:1, #50
        jmp .END

        .FUNC:
            sub u32:1, u32:1, #100
            jmp n:0
        .END:
            dbg u32:1
    ";

    let vm = compile_and_run(PROG);
    let v: u32 = vm.inspect_uint(1, u32::BITS);

    assert_eq!(250, v);
}

#[test]
fn cast_integers() {
    const PROG: &str = "
        mov i32:0, #100
        cast u32:0, i32:0

        mov i32:1, #-1
        cast u32:1, i32:1

        cast i32:3, 0xFFFFFFFF
    ";
    let vm = compile_and_run(PROG);
    let u32_0: u32 = vm.inspect_uint(0, u32::BITS);
    let u32_1: u32 = vm.inspect_uint(1, u32::BITS);
    let i32_3: i32 = vm.inspect_int(3, i32::BITS);

    assert_eq!(u32_0, 100);
    assert_eq!(u32_1, u32::MAX);
    assert_eq!(i32_3, -1);
}

#[test]
fn pre_init_addresses() {
    const PROG: &str = "
        &CONST_A: [0x01]
        &CONST_B: [0x12]

        load u8:0, &CONST_A
        load u8:1, &CONST_B

        dbg u8:0
        dbg u8:1

        seq u1:0, &CONST_A, &CONST_B
        dbg u1:0
    ";
    let vm = compile_and_run(PROG);
    let u8_0: u32 = vm.inspect_uint(0, u8::BITS);
    let u8_1: u32 = vm.inspect_uint(1, u8::BITS);
    let u1_0: u32 = vm.inspect_uint(0, 1);

    assert_eq!(0x01, u8_0);
    assert_eq!(0x12, u8_1);
    assert_eq!(false as u32, u1_0);
}

#[test]
fn test_compare_uint_constant() {
    const PROG: &str = "
        mov u32:0, 0x1
        slt u1:0, u32:0, 0x800000000
        dbg u1:0
    ";
    let vm = compile_and_run(PROG);
    let u1_0: u32 = vm.inspect_uint(0, 1);
    assert_eq!(true as u32, u1_0);
}

#[test]
fn test_store_iaddress() {
    const PROG: &str = "
        ; Find out how much room an instruction address takes up
        nsize u32:0
        dbg u32:0
        ; Allocate that amount of memory
        alloc m:0, u32:0

        ; Store a label
        mov n:1, .DUMMY_LABEL_1
        mov n:2, .DUMMY_LABEL_2

        .DUMMY_LABEL_1:
            nop
        .DUMMY_LABEL_2:
            nop

        ; Store then load it back from memory
        store m:0, n:2
        load n:3, m:0
        dbg n:3

        ; Should be true
        seq u1:0, n:3, .DUMMY_LABEL_2
        ; Should be false
        seq u1:1, n:3, .DUMMY_LABEL_1

        dbg u1:0
        dbg u1:1
    ";

    let vm = compile_and_run(PROG);
    let u1_0: bool = vm.inspect_bool(0);
    let u1_1: bool = vm.inspect_bool(1);

    assert_eq!(u1_0, true, ".DUMMY_LABEL_2 == .DUMMY_LABEL_2 (u1:0)");
    assert_eq!(u1_1, false, ".DUMMY_LABEL_2 != .DUMMY_LABEL_1 (u1:1)");
}

#[test]
fn test_store_mem_addr() {
    const PROG: &str = "
        msize u32:0
        alloc m:0, u32:0
        alloc m:1, #4
        ; Store the memory address m:1 at m:0
        store m:0, m:1

        ; m:0 and m:1 should be different
        seq u1:0, m:0, m:1
        ; But the value at m:0 should be m:1
        load m:2, m:0
        seq u1:1, m:1, m:2

        dbg m:0
        dbg m:1
        dbg m:2

        dbg u1:0
        dbg u1:1
    ";

    let vm = compile_and_run(PROG);
    let u1_0 = vm.inspect_bool(0);
    let u1_1 = vm.inspect_bool(1);

    assert_eq!(u1_0, false, "m:0 != m:1");
    assert_eq!(u1_1, true, "m:1 == m:2");
}
