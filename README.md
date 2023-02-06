# Rust

[Rust](https://github.com/rust-lang/rust) has been quietly but steadily growing over the past decade. It's been the [most loved language for the past 7 years](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages), and Linus Torvalds (the lead organizer and developer of the OS that the world runs on: Linux) has announced Rust will be an acceptable language for use in the Linux kernel. Reading up on Rust, it seems the main things people like about Rust are:
* Performance: it's as fast as C.
* Security: the Rust compiler checks for unsafe implementations/conditions (which are essentially unavoidable in C) and will help you fix them. 
* Developer Experience: people love Rust's package manager, cargo, and the package management/distribution experience.

 I should finally see for myself.

## Quick Installation

Per the [installation instructions](https://doc.rust-lang.org/book/ch01-01-installation.html) in the canonical Rust-lang book,

```bash
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

When prompted, enter 1

```bash
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
> 1
```

Open a new shell and you're good to go.

## Hello World

Put some code (like that shown below) into a file with extension `.rs`, let's say `main.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```

Then compile that code via

```bash
$ rustc main.rs
```

and run that compiled code via

```bash
$ ./main
"Hello, world!"
```

## Hello Cargo

`cargo` is Rust's package manage and general utility tool. It provides functionality to:

### Create a new project via Cargo

```bash
$ cargo new hello_cargo
```

This will create a project with this structure.

```bash
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

### Building a project via Cargo

Rather than use `rustc`, you can use `cargo` to build rust projects.

```bash
$ cargo build
```

Then you can run the built file via 

```bash
$ ./target/debug/hello_cargo
Hello, world!
```

But yeah, that's much less convenient than running the basic `rustc` build. But `cargo` provides a way to build and run at the same time.

### Running a project

To automatically build and run a project in one command, use

```bash
$ cargo run
```

# Language Facts and Concepts

## Variables

Fact: Variables are immutable 

## Types

### Scalar types
Rust has four primary scalar types: integers, floating point numbers, booleans, and characters

Integers:
* Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (same size as an address on the system: 32 or 64 bits)
* Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (same size as system architecture: 32 or 64 bits)
* Default: `i32`
* Example usage: 
    * `let x: u32 = 25;`
    * `let y = 34;`

Floats: 
* `f32`, `f64`
* Example usage:
    * `let x_f: f64 = -87.619283;`
    * `let y_f = 41.877846;`

Booleans:
* `bool` (`true` or `false`)
* Example usage:
    * `let x_t: bool = true;`
    * `let y_f = false;`

Characters:
* `char` (unicode; 4-byte width)
* Note: define `char`s with single quotes. Double quotes are for strings. 
* Example usage:
    * `let x_c = 'c';`
    * `let y_c: char = 'y';`
    * `let heart_eyed_cat = '😻';`

### Compound Types
These can group different 

Tuple: 
* `(u64, char, bool, ...)` 
* Can hold elements of different types.
* Example usage:
    * `let x_tuple: (i8, char, bool) = (30, 'U', true);`
* Access values via:
    * `x_tuple.<index>`
    * `x_tuple.0`

Array:
* Can only hold elements of the same type.
* Useful when implementing a stack or when the number of elements is known and fixed.
* Example usage:
    * `let arr_i = [0, 1, 2, 3];`
    * `let arr_e: [u16; 4] = [10, 11, 12, 13];`
* Access values via:
    * `arr_i[<index>]`
    * `arr_i[0]`

### Syntax

[Glossary of Rust's syntax](https://doc.rust-lang.org/book/appendix-02-operators.html)