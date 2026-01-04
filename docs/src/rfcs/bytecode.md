# UMC Bytecode Format

In UMC we have not only "unlimited" registers, but also "unlimited" register types.
These need to be represented in a fixed amount of space, which requires a variable-length encoding of instructions.

Constraints:
- Unlimited registers and register types
- Minimum 64 instruction opcodes, ideally more or extensible

Register-based bytecodes are generally faster than stack-based ones, but spend significantly longer
on "instruction-fetch".
So we want to use an efficient encoding to mitigate this, especially since we have unbounded size.

## Register Types
- Unsigned Int (+ width)
- Signed Int (+ width)
- Float (+ width)
- Memory Address
- Instruction Address

Plus vector types with a "length"
This could fit in 4 bits, first bit as a flag for a vector.
However we might also want a flag for a constant, at which point we'd need 8 bits (a full byte).

## Basic/Naive encoding

Example of 1.5 byte type and 1 byte index
```
u32:1 | 0001 00010000 00000001 | 2.5 bytes
        ^^^^ ^^^^^^^^ ^^^^^^^^
    unsigned   |      |
              32      index 1
```

Example of vectorised register
```
u32x8:1 | 1001 00010000 0100 00000001
          ^^^^ ^^^^^^^^ ^^^^ ^^^^^^^^
 unsigned vector  |      |      |
                  32     8    index
```

Example of a full u32-u32 instruction (8.5 bytes):
```
add     | 00000010 <- add opcode
u32:1,  | 0001 00010000 00000001
u32:2,  | 0001 00010000 00000010
u32:3   | 0001 00010000 00000011
```

Some instructions have only one valid register type, or a constant, so they don't necessarily need a type:
```
jmp | 00001000 <- jump opcode
n:0 | 00000000 <- jmp to register 0 value

jmp    | 00001000 <- Jump-constant instruction ideally?
.LABEL | 00001000
```

A lot of instructions are likely to be "uniform", using the same types (for the registers that aren't forced),
so we could consider an encoding like (5.5 bytes):
```
add     | 00000010 <- add opcode
u32     | 0001 00010000
:1      | 0000 0001
:2      | 0000 0010
:3      | 0000 0011
```

However, using a full byte just to store the 32/64-part of u32/u64 is pretty wasteful, especially considering they are probably the most common type.
Unfortunately the full range of u1-u32 is allowed, so we can't simply store log2(width).

## Huffman encoding of register types
Ultimately, the most common register types will be used massively more than the less common types.

I propose a Huffman encoding of types (with their widths / lengths!).

In the following example, a program uses 32-bit registers most often, plus some others used in UMC.
```
 entry    | register
--------------------
0000 0000 | u32
0000 0001 | i32
0000 0010 | u64
0000 0011 | i64
0000 0100 | f32
0000 0101 | f64
0000 0111 | u1 (used as a boolean in UMC)
0000 1001 | n (Instruction Address)
0000 1011 | m (Memory Address)
0000 1111 | u32x8 (example of vector registers, probably only a handful used)
... a few more application specific, maybe u128/i128, u8 etc.

0000 0000 0000 0001 | If necessary, for rare types, a 2-byte or 1.5-byte encoding can be used
```

Instruction and Memory address registers will only really be used in add/sub for offsetting, so won't be too high, but should still fit in the table.

It might also be a good idea to 'fix' the first few entries of the table for types we know will be used, like `u32`, `u64`, `i32`, `i64`, `u1`, `n` and `m` to
reduce how much binaries change and make things like combining two umc binary files easier.

Instructions now for non-uniform types (7 bytes):
```
add     | 0000 0010 <- add opcode
u32x8:1 | 0000 1111 0000 0001
u32x8:2 | 0000 1111 0000 0010
u32:2   | 0000 0000 0000 0010
```
Covers any 3 operand types with a range of 256 different registers.

Only 5 bytes needed for a uniform instruction
```
add     | 0100 0010 <- add opcode
u32     | 0000 0000 <- u32 index
:1      | 0000 0001
:2      | 0000 0010
:3      | 0000 0011
```

Re-indexing with the most common as lower numbers can also be used to ensure the most common register indexes are packed tightly,
without changing the result of the program.
1 byte is probably the minimum for a register index.
Another advantage is that registers can be stored in an implementation-dependent way without decoding penalty, and we can just store a list of register operands.

Can reserve one or two bits in the instruction opcode for 2-byte encoding where rare types or high indices are used.
Leading 1 on operand type could indicate a inline constant

### Huffman encoding registers
We could take this even further for a much greater space saving, by encoding the exact registers (type + index).

```
   entry  | register
---------------------
0000 0000 | u32:0
0000 0001 | u32:1
0000 0010 | u32:2
0000 0011 | u32:3
0000 0100 | u64:0
0000 0101 | u64:1
0000 0110 | i32:0
0000 0111 | i32:1
0000 1000 | i32:2
0000 1001 | f32:0
0000 1010 | f32:1
0000 1011 | u32x8:0
```

This causes an 4-byte encoding of:
```
add    | 0000 0010 <- add opcode
u32:2  | 0000 0010
u32:1  | 0000 0001
u32:0  | 0000 0000
```

The cost of non-uniform encodings is no longer an issue (Still 4 bytes):
```
add   | 0000 0010 <- add opcode
u64:0 | 0000 0100
u32:1 | 0000 0001
u32:2 | 0000 0010
```

However this would require a decently large table at the start of the file, and decoding logic would also be more complex,
if you wanted to store values of the same type contiguously.

