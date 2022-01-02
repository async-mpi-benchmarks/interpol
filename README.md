# Interpol

An interposition library for tracing MPI calls.

## Introduction
The library itself is written in C and calls Rust code to make storing MPI operations and serialization of these easier.

**Interposition library architecture:**
- The `interpol-rs` directory contains the Rust-side of the library,
  implementing the storage of recorded MPI calls (*events*) and their
  serialization.
- The `include` directory contains all the header files necessary for the
  C-side library (Rust bindings missing at the moment).
- The `src` directory contains the C source code for the interposition
  library.
  
We are still not sure which build system we will use for the C library.

***This is still work in progress.***

## Building
Build the Rust backend using Rust's tool, `cargo`:
```
cargo build --release
```
You can also run the test with the `cargo test` command.
