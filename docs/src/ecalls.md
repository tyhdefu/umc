# Environment Calls
Environment calls are like system calls in UMC.
An environment call requests the the VM perform privilleged, (likely platform specific) action on its behalf.

## UMC External Call Table

| Name    | id    | Details           | Short Description |
| ------- | ----- | ----------------- | ----------------- |
| `exit`  | `0x0` | [exit](#exit)     | Halt the UMC VM with the given exit code |
| `open`  | `0x1` | [open](#open)     | Open a file |
| `close` | `0x2` | [close](#close)   | Close a file |
| `read`  | `0x3` | [read](#read)     | Read bytes from a file |
| `write` | `0x4` | [write](#write)   | Write bytes to a file |
| `getarg`| `0x10`| [getarg](#getarg) | Retrieve an arbitrary bytes argument passed to the UMC VM by index |

## External Call List

### exit
Exits with the given exit code (should this be kept?)

Example:
```
ecall u1:0, 0x0, #1 ; Exit the VM with exit code 1
```

### open
```
ecall m:1, 0x1, m:0, #10 ; Filename of length 10
```

### close
Close the given file
```
ecall u1:0, 0x2, m:1
```

### read
Read up to the given number of bytes from the file
```
ecall u64:1, 0x3, m:1, m:2, u64:0
```

### write
Write the given number of bytes from a memory address into a file
```
ecall u1:0, 0x4, m:1, m:2, u64:0
```

### getarg
Retrieve an user-provided argument given the UMC Program at runtime.

The first argument is always a memory address of the null-terminated name of the program.
The remaining arguments are provided by the user when the UMC VM is started.

All values are copies of the original arguments, and the same arguments can be retrieved multiple times in the program.
Memory addresses are also "copies", but values may be stored in them - this can allow returning parameters to the caller.

UMC programs that use this environment call should document their required parameters.
Calling `getarg` with type or argument number that doesn't match the provided arguments will cause the program to terminate.

Examples:
```
ecall m:0,   0x10, #0 ; Get 0th argument - null terminated name of the program
ecall m:1,   0x10, #1 ; Get first argument as a memory address
ecall u64:0, 0x10, #1 ; Get first argument as a 64-bit unsigned integer
ecall i64:1, 0x10, #2 ; Get second argument as a 64-bit signed integer
ecall f32:0, 0x10, #3 ; Get third argument as a 32-bit float
```

