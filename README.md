# Interpol

An interposition library for tracing MPI calls.

## Introduction
The library itself is written in C and calls Rust code to make storing MPI operations and serialization of these easier.

***This is still work in progress.***

## Building
Build the Rust backend using Rust's tool, `cargo`:
```
cargo build --release
```
You can also run the test with the `cargo test` command.
