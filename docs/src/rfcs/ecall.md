# External calls / Syscalls in UMC

```
; External calls in UMC, function like exact order
ecall retreg, .LABEL?, u32:1, u32:0, f32:3, i32:1,
```

## System V x86-64 ABI Translation
```
rdi <= u32:1
rsi <= u32:0
rdx <= i32:1

xmm0 <= f32:3

; System V can only return rax, but this is interpreted based on ecall
; e.g. ecall a:0   ; interpret as address
; e.g. ecall u32:0 ; interpret as u32
; e.g. ecall i64:0 ; interpret as i64
ret
```

## UMC External call
```
.LABEL:
  u32:0 <= u32:1
  u32:1 <= u32:0

  f32:0 <= f32:3

  i32:0 <= i32:1

  ret X ; retreg <= X
```
