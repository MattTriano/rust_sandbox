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