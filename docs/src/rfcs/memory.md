# UMC Memory Model
- Four Instructions: `alloc` and `free`, `load` and `store`
- One register type with archecture-dependent width: `a:0`
- `add` and `sub` allowed with one address register and one (un)signed integer "offset"

## Examples

```
alloc a:0, 4
store a:0, u32:1 ; Store 4-byte integer into address register
load u32:1 ; Load it as a 32 bit integer
free a:0 ; Frees the memory allocated earlier. a:0 is now invalid
```

## Rationale
Allocating and de-allocating memory is a common task, which if we are running in the UMC VM,
can easily be delegated to the VM.

Questions:
- How do we allocate a block of memory large enough for a memory address if size is machine-dependent? A constant?
