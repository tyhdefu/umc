# Universal Machine Code

Running the example programs:
- Fibonnaci: `./run.sh progs/fib.umc`
- Pi: `./run.sh progs/pi.umc`

Compiling umc text programs into UMC binary files:
- Fibonnaci: `./assemble.sh progs/fib.umc`
- Pi: `./assemble.sh progs/fib.umc`

To produce a standalone binary, `cargo build --release` will build all packages
- UMC Virtual Machine (bundled with an assembler by default): `target/release/umc-vm`
- UMC Assembler only: `target/release/umcc`

# Cross-compilation to other architectures and testing
The easiest way to test UMC on a different architecture is to use [cross](https://github.com/cross-rs/cross) and podman / docker.

- Install cross: `cargo install --locked cross`
- Ensure you have either podman or docker installed

Cross works just like cargo, using an image to provide libraries so you don't have to install them on your host machine.

One of the best targets for testing is a 32-bit ARM machine `armv7-unknown-linux-gnueabihf`, as this has different endianness and pointer size to x86_64.

Compile for ARM 32-bit (e.g. a Raspberry Pi Zero)
`cargo build --target armv7-unknown-linux-gnueabihf`

Run unit tests on ARM 32-bit (emulation via QEMU):
`cargo test --target armv7-unknown-linux-gnueabihf`

Run a ARM 32-bit binary (emulation via QEMU)
`cargo run --target armv7-unknown-linux-gnueabihf -- progs/fib.umc`

# Building and testing for Web Assembly
Running and testing for Web Assembly is simple.

Testing is best done on `wasm32-wasip1` (which supports stdout/stderr and reading files), which should be installed with rustup: `rustup install wasm32-wasip1`.
You can build via `cargo build --target wasm32-wasip1`.
You can build test binaries which can then be run with a web assembly virtual machine. The binary will be shown in the output:
- `cargo test --target wasm32-wasip1 -p umc-vm`
- `cargo test --target wasm32-wasip1 -p umc-vm`
- `cargo test --target wasm32-wasip1 -p umc-vm`

One suitable virtual machine is `wasmtime`, which can be installed with `cargo install wasmtime --locked`
