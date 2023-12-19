# Rust Tutorial

Welcome to the Rust Tutorial – your guide to learning Rust from scratch. This tutorial is a continual work-in-progress, evolving alongside my own journey into Rust programming. My aim is to create a concise and helpful resource for beginners looking to dive into Rust programming. 🚀 I primarily use the `cargo` tool for building and running code (though you can also use `rustc` directly to compile files).

## Table of Content

-   [Rust Tutorial](#rust-tutorial)
    -   [Table of Content](#table-of-content)
    -   [Getting Started](#getting-started)
    -   [Build and Run](#build-and-run)
    -   [Using `rustc` to Compile and Run](#using-rustc-to-compile-and-run)
    -   [Contributing and Suggestions](#contributing-and-suggestions)
    -   [References and Additional Reading](#references-and-additional-reading)

## Getting Started

Begin your Rust journey by cloning the [project repository](https://github.com/A-raj468/Rust-Tutorial).

```console
$ git clone https://github.com/A-raj468/Rust-Tutorial.git
```

Once cloned, navigate into the project directory:

```console
$ cd Rust-Tutorial
```

## Build and Run

Move to the desired tutorial (e.g., `04_data_types`) and compile using the `cargo build` command:

```console
$ cd 04_data_types
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
```

To execute the built binary, use:

```console
$ ./target/debug/data_types
Int32: -2
Unsigned Int: 3
Float: 0.12323
Boolean: true
Character: ;
Tuple: (2, -123.988, false, ')
```

To compile and run the file in one go, employ the `cargo run` command:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/data_types`
Int32: -2
Unsigned Int: 3
Float: 0.12323
Boolean: true
Character: ;
Tuple: (2, -123.988, false, ')
```

## Using `rustc` to Compile and Run

For those who prefer using `rustc` directly, compile the file using:

```console
$ rustc <src_file_path>
```

To run the compiled executable, use:

```console
$ <executable_path>
```

## Contributing and Suggestions

Your contributions and suggestions are highly valued! Feel free to contribute, suggest improvements, or report issues to make this tutorial more valuable for fellow Rust enthusiasts. Happy coding! 🦀

## References and Additional Reading

This tutorial is made, in part, by following the [Rust Programming Tutorial](https://www.youtube.com/playlist?list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ) by YouTuber [Tech with Tim](https://www.youtube.com/@TechWithTim).

Here are some recommended resources for further learning and discussion:

-   [Official Rust Documentation](https://doc.rust-lang.org/stable/book/)
-   [Rust Programming Language Forum](https://users.rust-lang.org/)
-   [Rust Reddit Community](https://www.reddit.com/r/rust/)
