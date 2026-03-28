# Binary Format

This RFC defines the binary format for UMC.

All UMC binary files begin with the following 16-byte magic data:

```
Magic: 0x7F 0x55 0x4D 0x43 0x20 0x42 0x79 0x74 0x65 0x64 0x6F 0x64 0x65 0x00 0x00 0x00
```
This translates to the following ascii string: `\x7FUMC Bytecode\0\0\0`

All UMC binary files are then immediately followed by two bytes describing the UMC binary format version:
```
Major Version: 1 byte
Minor Version: 1 byte
```
If a UMC Virtual Machine encounters an incompatible major version, it should refuse to execute the program.
If a UMC Virtual Machine encounters a minor version higher than what is supported for that major version by the Virtual Machine, then it should refuse to execute the program.
Lower minor versions than are supported by the UMC VM for a particular major version will work correctly on VMs that correctly follow the UMC Specification for a higher minor version.

## Little Endian Base 128 (LEB128)
The UMC Binary File format extensively uses LEB128 for integers.

## Version 0.3

### Type Header Table
The first header found in a 1.0 UMC Binary file contains the register type header, which maps type indices to types

This table first starts with the number of entries:
```
Entry Count: Unsigned LEB128
```

Followed by each entry in series:
```
Control Byte:
  - Length Flag:   1 bit
  - Constant Flag: 1 bit
  - Reserved (0):  5 bits
  - Register Type: 3 bits
Width: Unsigned LEB128
Length: Unsigned LEB128 (Only if indicated in the control byte)
```
Duplicate entries are currently not permitted, as they may be significant in future versions.
~~Duplicate entries are permitted for register entries (constant flag not set).
A instruction references **a different register** if it has a different type index, even if the register index is the same.
This allows using encoding more registers of the same type in the same number of bytes.
For example, it allows 256 unique register operands to be encoded instead of 128 in two bytes.~~

The register types are encoded as follows:
| Register Type    | Encoding |
| ---------------- | -------- |
| Unsigned Integer | 0b000    |
| Signed Integer   | 0b001    |
| Float            | 0b010    |
| Memory Address   | 0b011    |
| Instruction Address | 0b100 |

### Pre-initialised memory Table
UMC supports pre-initialised memory regions which can be referenced by a memory label, which is encoded as a memory constant.
The pre-initialised memory table immediately follows the RT Header, and begins with the number of entries:
```
Entry Count: Unsigned LEB128
```

Then for each entry is stored contiguously:
```
Data Length: Unsigned LEB128
Data: Variable number of bytes
```

### Instruction Encoding
Immediately after the type header table, are all instructions, one after the other.

The first byte of each instruction is the opcode:
```
Op Code: 1 byte
```

| Op Code | Value |
| ------- | ----- |
| NOP     | 0b000000 | 
| MOV     | 0b000001 | 
| ADD     | 0b000010 |
| SUB     | 0b000011 |
| MUL     | 0b000100 |
| DIV     | 0b000101 |
| MOD     | 0b000110 |
| JMP     | 0b001000 |
| JAL     | 0b001001 |
| BZ      | 0b001010 |
| BNZ     | 0b001011 |
| EQ      | 0b001100 |
| GT      | 0b001101 |
| GTE     | 0b001110 |
| AND     | 0b010000 |
| OR      | 0b010001 |
| XOR     | 0b010010 |
| NOT     | 0b010011 |
| ALLOC   | 0b100000 |
| FREE    | 0b100001 |
| LOAD    | 0b100010 |
| STORE   | 0b100011 |
| SIZE    | 0b100100 |
| CAST    | 0b110001 |
| ECALL   | 0b110100 |
| DBG     | 0b111111 |

#### Fixed Instruction Encoding
The majority of instructions are encoded using this format, since for a given op code they have fixed number of arguments.
Each operand begins with its type:
```
Operand Type: Unsigned LEB128
```
This operand type is an index in the Type Header Table.
Depending on this entry, one of the following values are then decoded:

Register operands are encoded as:
```
Register Index: Unsigned LEB128
```

Unsigned integer constants are encoded as:
```
Constant Value: Unsigned LEB128
```

Signed integer constants are encoded as:
```
Constant Value: Signed LEB128
```

Float constants are encoded as:
```
Constant Value: 4 or 8 bytes
```
The number of bytes for the constant value is dictated by the width's bytes, rounded up.
The only supported float constant types are f32 and f64, which use 4 and 8 byte constants respectively.

Memory Label Constants are encoded as:
```
Constant Value: Unsigned LEB128
```
These refer to an index in the pre-initialised memory table.

Instruction Label Constants are encoded as:
```
Constant Value: Unsigned LEB128
```

#### Variable Instruction Encoding
This encoding is used for the `ECALL` opcode.
The variable instruction encoding first starts with the operand count:
```
Operand Count: Unsigned LEB128
```

And then the encoding is the same as the fixed instruction encoding.

#### Sizeof Instruction Encoding
`msize` and `isize` are encoded like a fixed instruction with two operands:
- Register Operand, but with no index
- Unsigned Register Operand
