# UMC Registers
There are a unlimited number of registers in UMC.
Each type (including width) is a separate set of registers.

## Register Types

| Register Set | Syntax | Widths | Description |
| ------------ | ------ | ------ | ----------- |
| Unsigned Int | `u32:0`, `u8:0`, `uX:0` | Any | Unsigned Integers |
| Signed Int   | `i32:0`, `i64:0` | Any | Signed Integers |
| Floats       | `f32:0`, `f64:0` | 32/64 | Floating Point Numbers |
| Memory Address | `m:0`, `m:1` | At least 32-bit | Memory location |
| Instruction Address | `n:0`, `n:1` | At least 32-bit | Instruction location |

## Vector registers
All register types can be extended to a vector register type.

| Register Set | Syntax | Description |
| ------------ | ------ | ----------- |
| Unsigned Int | `u32x4:0` | Vector of 4 Unsigned 32-bit Integers |
| Signed Int   | `i64x4:0` | Vector of 4 Signed 64-bit Integers |
| Float        | `f32x8:1` | Vector of 8, 32-bit Floats |
| Memory Address | `mx8:0` | Vector of 8 memory addresses |
| Instruction Address | `nx8:0` | Vector of 8 instruction addresses |
