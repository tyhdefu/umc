# Memory Model

UMC uses little-endian byte-ordering, and the smallest unit of memory is the byte.

A given number of bytes can be allocated with the `alloc` instruction:
```
alloc m:0, #100
```
This gives a memory address in `m:0`, which can be used for load and store operations:
```
mov u32:0, #7
store m:0, u32:0
load u32:1, m:0
```

This should be free'd once it is no longer needed:
```
free m:0
```

Memory addresses can be offset using add:
```
add m:0, #10
```
but it is the programmer's responsibility to ensure that they address is incremented enough for the given data type.

Memory and instruction addresses have an opaque memory representation.
It is undefined behaviour to do anything other than load a memory address from where a memory address is stored, and the same with an instruction address.
However, the `msize` and `nsize` instructions allow you to know how much room an address will take up:
```
; Allocate enough for a memory address to be stored
msize u32:0
alloc m:0, u32:0
```

Pre-allocated strings and arbitrary bytes are supported through memory labels:
```
&HELLO: "hello"

; Load the first character of "hello" into u8:0
load u8:0, &HELLO
```
