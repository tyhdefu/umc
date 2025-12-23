# Explicit and Implicit Casting

tl;dr:
Widening is allowed, but all other casts must be explicit.
Calculations are always done in the domain of the domain of the destination register.

## Examples

Examples of what is allowed:
```
add u32:0, u32:0, u32:0 ; Great
add u64:0, u32:0, u32:0 ; Implicit widening cast

; Alowed, and recommended for large multiplications
; Would translate as multiply low and high
mul u64:0, u32:0, u32:0
mul u128:0, u64:0, u64:0

; Comparisons can use any width unsigned register
gt u1:0, u32:0, u32:0
gt u1:0, u64:0, u32:0 ; 64-bit comparison

; Use explicit casts where necessary

; Do 32-bit subtraction
mov u32:1, u64:0 ; Require u64:0 to fit in 32 bits
sub u32:2, u32:1, u32:0
; Do 64-bit subtraction, expect the result to fit in 32 bits
sub u64:2, u64:1, u32:0
mov u32:2, u64:2

mov u32:0, i32:0
gt u1:0, u32:0, u32:1 ; Performs unsigned comparison: Weird if i32:0 is negative

mov i32:0, i32:0
gt u1:0, i32:0, i32:0 ; Performs signed comparison: Weird u32:0 uses the top bit
```

What is not allowed
```
; This is not allowed, result depends on cast before vs after, confusing
sub u16:0, u32:0, u32:0

; Confusing comparisons
gt u1:0, u32:0, i32:0 ; This is actually Undefined behaviour in C when i32:0 is negative
; E.g. 0b1000, 0b0001 => Signed comparison interprets u32:0 as smallest negative
; E.g. 0b0000, 0b1000 => Unsigned comparison interprets smallest negative as bigger than 0
```

## Types of casts

What operations do we need to do?
- Narrowing cast (signed, unsigned or float)
- Unsigned <-> Signed (bit or value-based?)
- Unsigned <-> Float (Value-based)
- Signed <-> Float   (Value-based)

`bcast` - Binary / Bitwise cast - for twos complement conversions/narrowing
`vcast` - Value cast - Best effort at encoding the same value with the new datatype

```
bcast u32:0, u64:0 ; Truncation
bcast i32:0, i64:0 ; Truncation

bcast u32:0, i32:0 ; Reinterpret two's complement i32:0 as a u32:0.
bcast i32:0, u32:0 ; Reinterpret unsigned as two's complement - if top bit is set then it signed overflow occurs

vcast f64:0, u32:0 ; Take the unsigned integer and convert it into its closest floating point representation
vcast f64:0, i32:0 ; Take the signed integer and convert it into its closest floating point representation

vcast f32:0, f64:0 ; Take the value of the f64 and re-encode it as a f32 (precision lost)

; Explicit vcast with specific behaviour?
abs u32:0, i32:0 ; Take the signed integer, make it positive so it definitely fits correctly (no vcast for u32 and i32?)

; Maybe just a fcast with specific behaviour too?
fcast f64:0, u32:0 ; Re-encode as a float
fcast f64:0, i32:0 ; Re-encode as a float
fcast f32:0, f64:0 ; Re-encode as a different precision float (not a straightforward bit-cast)

brdc u32x8:0, u32:0 ; Broadcast u32 to vector of u32s.
; Does it make sense to allow every instruction to operate on vector elements?
; Or just allow mov to put/pick (Leaning towards this, otherwise it signficantly complicates the instruction set)
```

## Rationale
Many instructions need to specify whether to operate in signed or unsigned form, such as comparisons.
The top bit of an unsigned integer can be confused with a signed negative number.

Doing something like:
```
sub u32:0, u64:0, u32:1
```

Does not equate to a single instruction in x86, RISC-V or ARM.
It must do a 32-bit or 64-bit operation, with a narrow/widen before/after

## Problems with using extra registers
The idea is that the casting instruction will be eliminated during JIT compilation.
However, the cast causes there to potentially be 2 registers used for the same value (one of which may outlive the other).

For example, if the following code were to be JIT compiled:
```
mov u32:1, u64:0
sub u32:2, u32:1, u32:0
```
it might require one register to store the 64 bit value, and another to store the 32-bit value (one of which may live much longer than the other).
In more complex examples, with many more registers, this might cause registers to overflow, even though u32:1 is really not needed.
However, if necessary this can be solved by clearing the register in UMC:
```
mov u32:1, u64:0
sub u32:2, u32:1, u32:0
mov u64:1, #0 ; If the u64:0 value is never needed
```
Now, only one register is needed if the instruction set supports accessing 64-bit registers as 32-bit.
The UMC JIT can set u64:0 to #0 without an output register, as it knows this is guaranteed.
