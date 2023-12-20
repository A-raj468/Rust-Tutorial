# Introduction

Introduction to `Rust` programming language.

Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It is a statically and strongly typed language.

## Table of Content

-   [Introduction](#introduction)
    -   [Table of Content](#table-of-content)
    -   [Installation](#installation)
        -   [Windows](#windows)
        -   [Linux/Mac/WSL](#linuxmacwsl)
        -   [Successful Intallation](#successful-intallation)
        -   [Updating and Uninstalling](#updating-and-uninstalling)
    -   [The First Program](#the-first-program)
    -   [References](#references)

## Installation

Follow the following steps to install Rust on your system.

### Windows

On windows, go [here](https://www.rust-lang.org/tools/install) and download the graphical installer. Follow the instructions to install `rust`.

You will also need MSVC build tools for Visual Studio 2013 or later. To acquire the build tools, you’ll need to install [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/). When asked which workloads to install, include:

-   Desktop Development with C++
-   The Windows 10 or 11 SDK
-   The English language pack component, along with any other language pack of your choosing

This should install `Rust` in your windows system.

### Linux/Mac/WSL

Open a terminal and enter the following command:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Follow the onscreen intructions. If the install is successful, the following line will appear:

```console
Rust is installed now. Great!
```

You should also install a C compiler as it will typically include a linker(Rust needs it) and also because some common Rust packages depend on C code and will need a C compiler.

### Successful Intallation

To check if it is installed correctly use:

```console
$ rustc --version
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

### Updating and Uninstalling

Once Rust is installed via `rustup`, updating to a newly released version is easy. From your shell, run the following update script:

```console
$ rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your shell:

```console
$ rustup self uninstall
```

## The First Program

Like every other tutorial on every other language, our first program would be the `hello_world`.

Copy the content of the file `src/main.rs` into a new file, let's call it `hello_world.rs`.

The code should look like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

Compile the code using the `rustc` compiler and execute the generated binary.

```console
$ rustc hello_world.rs
$ ./hello_world
Hello, world!
```

We used `rustc` compiler here directly, but going forward we would use the `cargo` tool provided by Rust for building and running our projects.

Use the `cargo` tool to build and run the `src/main.rs` file.

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/introduction`
Hello, world!
```

Rust files end in the `.rs` extention and are conventionaly named in `snake_case`.

We should also follow the `snake_case` naming conventions when writing our code.

## References

-   [Wikipedia](<https://en.wikipedia.org/wiki/Rust_(programming_language)>)
-   [Official Rust Installation Guide](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
