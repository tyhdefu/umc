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

One of the best targets for testing is a 32-bit ARM machine `armv7-unknown-linux-gnueabihf`, as this has pointer size and architecture to x86_64.

Compile for ARM 32-bit (e.g. a Raspberry Pi Zero)
`cross build --target armv7-unknown-linux-gnueabihf`

Run unit tests on ARM 32-bit (emulation via QEMU):
`cross test --target armv7-unknown-linux-gnueabihf`

Run a ARM 32-bit binary (emulation via QEMU)
`cross run -p umc-vm --target armv7-unknown-linux-gnueabihf -- progs/fib.umc`

## AArch64 on Windows (WSL2 + QEMU user-mode)
Compiles natively in WSL and runs only the resulting binary under `qemu-aarch64`, so no full VM is needed.

One-time setup inside WSL (Ubuntu):
- `sudo apt install -y build-essential gcc-aarch64-linux-gnu qemu-user`
- `rustup target add aarch64-unknown-linux-gnu`
- Add the following to `~/.cargo/config.toml`:
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
# -L points qemu at the aarch64 libc; use "qemu-aarch64 -cpu max -L ..." for newer extensions (e.g. LSE)
runner = "qemu-aarch64 -L /usr/aarch64-linux-gnu"
```

Work from a copy in the Linux filesystem (building from `/mnt/c/...` is slow), e.g. `cp -r /mnt/c/.../umc ~/umc && rm -rf ~/umc/target`, then:
- Run unit tests: `cargo test -p umc-vm --target aarch64-unknown-linux-gnu`
- Run a program: `cargo run -p umc-vm --target aarch64-unknown-linux-gnu -- --verbose progs/fib.umc`

To limit WSL's RAM/CPU, put the following in `C:\Users\<you>\.wslconfig` and run `wsl --shutdown`:
```ini
[wsl2]
memory=2GB
processors=2
```

# Building and testing for Web Assembly
Running and testing for Web Assembly is simple.

Testing is best done on `wasm32-wasip1` (which supports stdout/stderr and reading files), which should be installed with rustup: `rustup install wasm32-wasip1`.
You can build via `cargo build --target wasm32-wasip1`.
You can build test binaries which can then be run with a web assembly virtual machine. The binary will be shown in the output:
- `cargo test --target wasm32-wasip1 -p umc-vm`
- `cargo test --target wasm32-wasip1 -p umc-vm`
- `cargo test --target wasm32-wasip1 -p umc-vm`

One suitable virtual machine is `wasmtime`, which can be installed with `cargo install wasmtime --locked`

Tip: to make `cargo test` and `cargo run` work directly if you put the following in your `$CARGO_HOME/config.toml`:
```toml
[target.wasm32-wasip1]
# Run wasm32-wasip1 targets with the wasmtime VM, with access to the current directory
runner = "wasmtime --dir ."
```
