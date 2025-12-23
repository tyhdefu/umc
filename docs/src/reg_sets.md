# UMC Register Sets
The following register sets are available in UMC.

## Single Registers
These registers hold a single value of given type / width.

| Register Set | Syntax | Widths | Description |
| ------------ | ------ | ------ | ----------- |
| Unsigned Int | `u32:0`, `u8:0`, `uX:0` | Any | Unsigned Integers |
| Signed Int   | `i32:0`, `i64:0` | Any | Signed Integers |
| Floats       | `f32:0`, `f64:0` | 32/64 | Floating Point Numbers |
| Memory Address | `a:0`, `a:1` | At least 32-bit | Memory location |
| Instruction Address | `n:0`, `n:1` | At least 32-bit | Instruction location |

## Vector registers
