# Embryon

A programming language designed for embedded systems.

> [!WARNING]
> This project is still under active development.

For examples of the syntax, see the [examples](./embryon-cli/examples/)

## Dependencies

- [Cargo](https://doc.rust-lang.org/cargo/), to build and run the project.
- [LLVM 17](https://releases.llvm.org/17.0.1/docs/index.html), used to simplify code generation and for portability (eventually).

## Usage

For a simple overview, check out the [examples](./embryon-cli/examples/).

To build the compiler, run `cargo build`.
This will place a binary (`embryon-cli`) into `./target/debug/embryon-cli`.
You can then use this to compile an Embryon file to LLVM IR:

```sh
embryon-cli path/to/code.embryon
```

## Features

Embryon is still in very early stages, so many features are missing.
Generally, features are inspired by the [Rust programming language](https://www.rust-lang.org/).
Currently implemented are:

- Integers
- Integer Arithmetic
- Variables (both mutable and immutable)
- Constants
- Blocks as values
- Infinite loops (`loop`)

