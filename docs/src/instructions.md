# UMC Instructions

## Misc
| Mneumonic | Operand 1 | Operand 2 | Operand 3 | Description |
| --------- | --------- | --------- | --------- | ----------- |
| `mov`     | Reg       | Any       |           | Sets op1 = op2 |
| `cast`    | Reg       | Any       |           | Sets op1 = op2, casting |
| `bcast`   | Integer Reg | Reg     |           | Bit / Reinterpret cast |
| `abs`     | Numeric   | Signed Numeric | | Gets positive value |



## Arithmetic
| Mneumonic | Operand 1   | Operand 2 | Operand 3 | Description |
| --------- | ----------- | --------- | --------- | ----------- |
| `add`     | Numeric Reg | Numeric   | Numeric   | op1 = op2 + op3 |
| `sub`     | Numeric Reg | Numeric   | Numeric   | op1 = op2 - op3 |
| `mul`     | Numeric Reg | Numeric   | Numeric   | op1 = op2 * op3 |
| `div`     | Numeric Reg | Numeric   | Numeric   | op1 = op2 / op3 |
| `mod`     | Numeric Reg | Numeric   | Numeric   | op1 = op2 `mod` op3

## Jumps, Calls and Conditional Branches
| Mneumonic | Operand 1 | Operand 2 | Description |
| --------- | --------- | --------- | ----------- |
| `jmp`     | Label     |           | Unconditional jump to the given label |
| `bz`      | Label     | Numeric   | Jump to label if op2 is zero |
| `bnz`     | Label     | Numeric   | Jump to label if op2 is not zero |

## Comparison Operations
Comparison operations take an unsigned destination register (minimum width 1),
and sets the destination to 1 if true, otherwise sets to 0.
Comparisons are only allowed between the same register sets.

| Mneumonic | Operand 1    | Operand 2 | Operand 3 | Description |
| --------- | ------------ | --------- | --------- | ----------- |
| `eq`      | Unsigned Reg | Numeric   | Numeric   | \\(op2 = op3\\) |
| `gt`      | Unsigned Reg | Numeric   | Numeric   | \\(op2 \gt op1\\) |
| `gte`     | Unsigned Reg | Numeric   | Numeric   | \\(op2 \ge op3\\) |
| `lt`      | Unsigned Reg | Numeric   | Numeric   | \\(op2 \lt op3\\) |
| `lte`     | Unsigned Reg | Numeric   | Numeric   | \\(op2 \le op3\\) |

## Bitwise Operations
All (signed and unsigned) integers are defined to be stored in twos-complement representation.
The following bitwise operations work only between integer types.

| Mneumonic | Operand 1   | Operand 2 | Operand 3 | Description |
| --------- | ----------- | --------- | --------- | ----------- |
| `and`     | Integer Reg | Integer   | Integer   | Bitwise AND |
| `or`      | Integer Reg | Integer   | Integer   | Bitwise OR  |
| `xor`     | Integer Reg | Integer   | Integer   | Bitwise XOR |
| `not`     | Integer Reg | Integer   | Integer   | Bitwise NOT |


## Memory
Blocks of memory can be allocated with the `alloc` instruction.

| Mneumonic | Operand 1   | Operand 2 | Description |
| --------- | ----------- | --------- | ----------- |
| `alloc`   | Address Reg | Unsigned  | Allocates a continguous block of memory X bytes long |
| `free`    | Address Reg |           | De-allocates a block of memory |
