# Introduction

In this tutorial, we shall learn about some Rust tools such as `cargo` and `rustfmt`.

## Table of Content

-   [Introduction](#introduction)
-   [Table of Content](#table-of-content)
-   [Build using `cargo`](#build-using-cargo)
    -   [Start a new package](#start-a-new-package)
    -   [Compile and Run](#compile-and-run)
-   [Format using `rstfmt`](#format-using-rstfmt)
-   [References](#references)

## Build using `cargo`

Cargo is the Rust package manager. Cargo downloads your Rust package’s dependencies, compiles your packages, makes distributable packages, and uploads them to [crates.io](https://crates.io/), the Rust community’s package registry.

Now we shall learn how to use the `cargo` command line tool.

### Start a new package

Rust projects are often called [packages](https://doc.rust-lang.org/cargo/appendix/glossary.html#package). A package is basically where your source file is located.

To start a new package with Cargo, use `cargo new`:

```console
$ cargo new rust_tools
     Created binary (application) `rust_tools` package
```

It will generate a package called rust_tools where we shall write our programs.

Navigate to the rust_tools directory and see the project structure.

```console
$ cd rust_tools
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

This is our starting point. The Cargo.toml file is called a [manifest](https://doc.rust-lang.org/cargo/appendix/glossary.html#manifest), and it contains all of the metadata that Cargo needs to compile your package.

The `src/main.rs` file is where we will write our code.

It is the same one as in the [previous](https://github.com/A-raj468/Rust-Tutorial/blob/main/01_introduction/src/main.rs) tutorial:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Compile and Run

Now let's compile it:

```console
$ cargo build
   Compiling rust_tools v0.1.0 (/home/overlord468/Rust/Rust-Tutorial/02_rust_tools)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
```

And run it:

```console
$ ./target/debug/rust_tools
Hello, world!
```

Or we can use `cargo run` to compile and then run it, all in one step:

```console
$ cargo run
   Compiling rust_tools v0.1.0 (/home/overlord468/Rust/Rust-Tutorial/02_rust_tools)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/rust_tools`
Hello, world!
```

We can also check for error using the `cargo check` command. This command is faster than `cargo build` and handy for checking errors before compiling.

```console
$ cargo check
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

## Format using `rstfmt`

`rustfmt` formats your files for you. It gives good indentations and does the formating stuff for you.

We will intentionaly make our program look bad(bad formating).

```console
$ cat src/main.rs
    fn main() {

println!("Hello, world!");



}
```

Now we will format it:

```console
$ rustfmt src/main.rs
$ cat src/main.rs
fn main() {
    println!("Hello, world!");
}
```

It automatically corrects the indentations and spacing issues in our file.

## References

-   [Official Rust Cargo Documentation](https://doc.rust-lang.org/cargo/index.html)
