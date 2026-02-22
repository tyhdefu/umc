 # Initialised data

In NASM, its possible to define pre-initialised data in a file, which gets put in the .data segment:

```nasm
x db "hello\0"
```

This is especially useful for defining static strings, and it would be nice to be able to do this in UMC.

A simple syntax would be:
```
.x: "hello\0"
```

However this would then be confusing / mixing memory addresses versus instruction addresses.

Instead, we could introduce a memory label:
```
&x: "hello\0" ; A null terminated string constant

&y: [0x01, 0x02, 0x03] ; Arbitrary bytes
```

And this could only be used as a memory address constant:
```
load u8:0, &x
```
