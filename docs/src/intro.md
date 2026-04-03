# Introduction

## What is Universal Machine Code?
Universal Machine Code is an attempt to provide a portable bytecode, similar to WebAssembly.
It uses a register-based machine to achieve fast execution, and the reference virtual machine is on par with existing, fast WebAssembly interpreters such as `wasmi`.
The syntax and semantics of UMC have been explicitly designed to make JIT-compilation to common ISAs such as x86, Arm and RISC-V simple and fast.

Below is an example of iterative fibonacci in UMC.
```umc
.FIB:
  mov u64:0, #30
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
```
