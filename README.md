# lmdb-rust

LMDB library translated to unsafe Rust by C2Rust.

Even though internal tests (mtest*) are passing, an attempt to replace lmdb-sys in lmdb-rs crate caused a test failure with memory alignment error. 

# Goals

- [ ] Make it compatible with other architectures 32-bit, big-endian, wasm32, etc.)
- [ ] Translate to safe Rust
- [ ] Move C interface into another crate.

> [!WARNING]  
> This is an experiment. Use at your own risk.
