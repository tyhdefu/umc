# Basic Architecture
The original proposition of UMC is to create a register-based VM bytecode (as opposed to a stack-based VM).
LuaJIT successfull implemented a register-based bytecode with significant performance improvements [\[1\]](https://www.lua.org/doc/jucs05.pdf)

## Syntax Proposal - Typed Register Sets (Accepted)
```
;;;;; FULLY TYPED REGISTER SETS PROTOTYPE

; Pros:
; No implicit casting since independent register sets
;  - Vector registers can't be reshaped
;  - No undefined behaviour
;  - Easy to implement registers as vecs

; Cons:
; Very verbose
;  - Can be mitigated by implicit register type
;    add u32:0, :0, :1


fib:
  ; i32:0 (n)
  b i32:0 < $0, .L4
  mov u32:0, $0 ; int cur = 0
  mov u32:1, $1 ; int next = 1
  mov u32:2, $0 ; int i = 0
.L3:
  ; u32:3 next
  mov u32:3, u32:2 ; int next2 = next
  add u32:3, u32:3, u32:0 ; next2 += cur
  mov u32:0, u32:1 ; cur = next
  mov u32:2, u32:3 ; next = next2
  add u32:2, u32:2, $1
  b u32:2 != n, .L3
.L4:
  mov i32:0, u32:0

sum:
  ; a:0   - Array of integers
  ; u32:0 - Length of the array
  b i32:0 == 0, .L8    ; If empty, terminate
  mov i32:1, $0        ; int total = 0
  shl u32:0, u32:0, $2 ; Multiply by 4 to get length in bytes
  add a:1, a:0, u0       ; end address = start address + length
.L8:
  load u32:2 a:0
  add i32:1, i32:1, u32:2
  add a:0, a:0, $4
  b   a:0 != a:1, .L8
.L9:
  mov i32:0, $0
  ret

;;;;;;;;;;; IMPLICIT MULTIPLE OPERANDS ;;;;;;;;;;;;

fib:
  ; i32:0 (n)
  b i32:0 < $0, .L4
  mov u32:0, $0 ; int cur = 0
  mov u32:1, $1 ; int next = 1
  mov u32:2, $0 ; int i = 0
.L3:
  ; u32:3 next
  mov u32:3, :2 ; int next2 = next
  add u32:3, :3, u32:0 ; next2 += cur
  mov u32:0, :1 ; cur = next
  mov u32:2, :3 ; next = next2
  add u32:2, :2, $1
  b u32:2 != n, .L3
.L4:
  mov i32:0, u32:0

sum:
  ; a:0   - Array of integers
  ; u32:0 - Length of the array
  b i32:0 == 0, .L8    ; If empty, terminate
  mov i32:1, $0        ; int total = 0
  shl u32:0, :0, $2 ; Multiply by 4 to get length in bytes
  add a:1, a:0, u32:0       ; end address = start address + length
.L8:
  load u32:2 a:0
  add i32:1, i32:1, u32:2
  add a:0, a:0, $4
  b   a:0 != a:1, .L8
.L9:
  mov i32:0, $0
  ret

;;;;;;;;; VECTOR INSTRUCTIONS ;;;;;;;;;;;;;

doublearray:
  load  u32x8:0, a:0
  mul   u32x8:0, u32x8:0, $2
  store u32x8:0, a:0
```

## Original Syntax Proposal - Types don't include widths
```
;;;;; Instructions

; Based off RISC-V basic instruction set
; https://projectf.io/posts/riscv-cheat-sheet/#arithmetic


; Pros:
; Large reduction in instructions achieved
; - Lots of instructions have a signed/unsigned variant - add, div, mul, even bit shift, cond. jump
; - Instructions overloaded, or only work on some types of registers
; - Simple idea with different applications
; - Register names (not numbers) describe their capabilities
; Specifying width for same registers matches hardware
;  -  Can be simulated if not big enough
; Vector registers having different name, to avoid implicit conversions
; Destination register gives type of operation

; Cons:
; Undefined behaviour to reshape vector registers?
; - Semantic checker for different acceses?
; Lots of implicit sign casting
; Lots of repeating the same widths?
; - Could use implicit from destination like:
;   add u0:32 u0: u1:

; Arithmetic
add u0:32, $100 ; Immediate
add i0:32, i1:32, i2:32  ; Registers, all same, ints
add f0:32, f1:32, f3:32  ; Registers, all same, floats
add i0:64, i1:64, u1:32  ; Mix of types and widths

cast i2:64, u1:32
add [i64] %0, %1, ~~~u1:32~~~

neg i1:32                ; cast to i1, then negate
sub i0:32, u1:32, u2:32  ; Can subtract unsigned numbers (can the result be signed?)

mul u1:32, u2:32, u3:32  ; Multiplication of several signed numbers
mul u1:64, u2:32, u3:32  ; Multiplication (completely safe)
mul i1:64, i2:32, u3:32  ; Signed Multiplication

div u3:32, u2:32, u1:32  ; Unsigned division
div i3:32, u2:32, i1:32  ; Signed division

; Bitwise logic
and i3:32, u2:32, i1:32  ; Just works on bits. Only width needs to match.
not i0:32, i0:32         ; same again
or  i0:32, i0:32         ; same again
xor i0:32, i0:32         ; same again

; Shifts

shl i0:32, i0:32, $2 ; Arithmetic left shift (x2)
shr u0:32, i0:32, $2 ; Logical shift (determined by destination)

shl i0:32, i1:32, i2:32 ; Arithmetic left shift
shr u0:32, i1:32, i2:32 ; Logical right shift (determined by destination)

; This is quite good, saves a few instructions

; Memory accessing
load u0:32, 0xDEADBEEF ; Loads a 32 bit value into the register
load u0:8,  0xDEADBEEF ; Loads a 8 bit value into the register

mov u0:8, 0xFF
stor 0xDEADBEEF, u0:8
stor 0xDEADBEEF, u0:32 ; Store 32 bits even though its 8 bit

; Immediate offset in load / store?
load u0:32, 4(0xDEADBEEF)

; Jumps

j    0xBEEF     ; Jump to immediate address
j    .LABEL     ; Jump to label (labels have type aX)
jal  a0, 0xBEEF ; Jump to immediate address, store next instruction address in a0
jalr a0, a1, $4 ; Jump to a1+4, store next instruction address in a0 (Should immediate be multiplied by instruction size?)

call .LABEL ; Jump to 
ret         ;


; Conditional branches
beq u0:32, i0:32, .LABEL ; Branch if the two are equal
bne u0:32, i0:32, .LABEL ; Branch if the two are not equal

bgt u0:32, i0:32, .LABEL ; Branch if greater than
bge u0:32, i0:32, .LABEL ; Branch if greater than or equal to

; Allow aX instead of label above?

;;;; zero-register, like ARM64 and RISC-V? Partly an encoding concern

seq u0:1, i0:32, i0:8 ; Set u0 to 1 if equal
sne u0:1, 

; Alternative?
add %0:u32, %1:i32, %2:i32

; Vector registers?
; iv, uv, fv, av avoids

add iv0:32x8 iv0:32x8, iv0:32x8

add av0:x8, av0:x8, $4

;;;; Fibonacci example ;;;;

; Problem - how are parameters passed? As u0,u1,.. by default?
fib:
  ; i0:32 (n)
  b i0:32 < $0, .L4
  mov u0:32, $0 ; int cur = 0
  mov u1:32, $1 ; int next = 1
  mov u2:32, $0 ; int i = 0
.L3:
  ; u3: next
  mov u3:32, u2:32 ; int next2 = next
  add u3:32, u3:32, u0:32 ; next2 += cur
  mov u0:32, u1:32 ; cur = next
  mov u2:32, u3:32 ; next = next2
  add u2:32, u2:32, $1
  b u2:32 != n, .L3
.L4:
  mov i0:32, u0:32
  ret

;;;; Sum array ;;;;
sum:
  ; a0 - Array of integers
  ; u0 - Length of the array
  b u0:32 == 0, .L8    ; If empty, terminate
  mov i1:32, $0        ; int total = 0
  shl u0:32, u0:32, $2 ; Multiply by 4 to get length in bytes
  add a1, a0, u0       ; end address = start address
.L8:
  load u2:32, a0
  add  i1:32, i1:32, u2:32
  add  a0, a0, $4
  b    a0 != a1, .L8
.L9:
  mov i0:32, $0
  ret

;;;;; Vector instructions - Double array
doublearray:
  ; Assume multiple of 4
  load  uv0:32x8, a0
  mul   uv0:32x8, uv0:32x8, $2
  store uv0:32x8, a0

; Pick operations
pick u1:32, uv1:32x4, 0
pick u2:32, uv1:32x4, 1
pick u3:32, uv1:32x4, 2
pick u4:32, uv1:32x4, 3

pick uv1:32x4, uv0:32x8, 0 ; Take first 4
pick uv2:32x4, uv0:32x8, 4 ; Take second 4

; Put (reverse pick) operations
pick uv1:32x4, u1:32, 0

; Pick alternative - similar to ARM
mov u0:32, uv1:32x4[0]
mov u1:32, uv1:32x4[1]
mov u2:32, uv1:32x4[2]
mov u3:32, uv1:32x4[3]

mov uv2:32x2, uv1:32x4[2]

; Broadcast
brdc uv1:32x4, 0


; MUL V0.4S, V2.4S, V3.S[2] multiplies each of the four 32-bit elements in V2 by the 32-bit scalar value in lane 2 of V3, storing the result vector in V0.
; https://developer.arm.com/documentation/102474/0100/Fundamentals-of-Armv8-Neon-technology/Registers--vectors--lanes-and-elements

; is there any point having a stack pointer register?
; special case of a-type register

; How to detect overflow??
; add u3:32, u2:32, u1:32 : $overflow > u4:1
```

## Alternative Syntax Proposal - No types
```
add  r0:32, $100
adds r0:32, r1:32, r2:32 $-100

neg r0:32, r0:32
sub r3:32, r2:32, r1:32 # r3 = r2 - r1, signed/unsigned doesn't matter

mul  r3:32, r2:32, r1:32
muls r3:32, r2:32, r1:32
mul  r3:64, r2:32, r2:32
muls r3:64, r2:32, r2:32

div  u3:32, u2:32, u1:32
divs u3:32, u2:32, u1:32

; Bitwise - Sign not relevant
and r3:32, r2:32, r1:32
not r3:32, r2:32, r1:32
or  r3:32, r2:32, r1:32
xor r3:32, r2:32, r1:32

; Shifts

sll r3:32, r2:32, r1:32 ; Logical left shift
slr r3:32, r2:32, r1:32 ; Logical right shift

sra r3:32, r2:32, r1:32 ; Arithmetic left shift
; No arithmetic right shift

load  r3:32, 0xDEADBEEF ; Load 32 bits into register
store 0xDEADBEEF, r3:32 ; Store 32 bits into memory
```

## Alternative Syntax Proposal - Typed Instructions
```
;;;; TYPED INSTRUCTION PROTOTYPE

; Pros:
; No implicit casting since independent register sets
;  - Vector registers can't be reshaped
;  - No undefined behaviour

; Cons:
; %1 and %1 in different instructions, makes it hard to read
;  - This code is mostly going to be generated anyway
; Some instructions are supposed to have different types:
;  - load [u32] a0
;  - vector broadcast

; Memory leak of registers?

fib:
  ; i0:32 (n)
  b   [i32] %0 < #0, .L4
  mov [u32] %0, #0 ; int cur = 0
  mov [u32] %1, #1 ; int next = 1
  mov [u32] %2, #0 ; int i = 0
.L3:
  ; u3: next
  mov [u32] %3, %2     ; int next2 = next
  add [u32] %3, %3, #0 ; next2 += cur
  mov [u32] %0, #1     ; cur = next
  mov [u32] %2, %3     ; next = next2
  add [u32] %2, %2, #1 ; i++
.L4:
  mov [i32] %0, {u0:32}
  ret


sum:
  ; a0 - Array of integers
  ; u0 - Length of the array
  b    [u32] %0 == 0, .L8 ; If empty, terminate
  mov  [i32] %1, #0       ; int total = 0
  shl  [u32] %0, %0, #2   ; Multiply by 4 to get the length in bytes
  add  [a]   %1, %0, {u0:32} ; end address = start address + length
.L8:
  load [u32] %2, [a0] ; Load the memory address
  add  [i32] %1, %1, {u2:32}
  add  [a]   %1, %0, {u0:32}
  b    [a]   %0 == %1, .L8
.L9:
  mov [i32]  %0, #0
  ret
```

