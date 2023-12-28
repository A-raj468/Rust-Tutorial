# Introduction

This tutorial is all about variables(and constants). We will learn about variables, constants and shadowing.

## Table of Content

-   [Introduction](#introduction)
    -   [Table of Content](#table-of-content)
    -   [Variables](#variables)
    -   [Shadowing](#shadowing)
    -   [Constants](#constants)
    -   [References](#references)

## Variables

Variables are where we store stuff(could be integers, floats, functions/closures, etc.). In Rust, valriables are declared using the `let` keyword.

Rust is a strongly and strictly typed language, i.e., any variable in rust is going to be given a type(either explicitly or implicitly) and it can not cahnge its type during its scope.

Implicit type declaration means one leaves the type inference to the compiler, while explicit type declaration means assigning types to variable manually.

Both of the above lines of code are equivalent(except for the variable name).

```rust
let x: i32 = 100; // explicit type declaration
let y = 100; // implicit type declaration

// The following line will produce a compiler error, as we are trying to assign an integer(x) to a variable of type String.
// let text: String = x;
// To assign value of one type to another, we need to explicitly convert it.
let text: String = x.to_string();
```

We will discuss about datatypes in more details in a later chapter.

## Shadowing

## Constants

## References
