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

Note: If you want a deeper understanding of the command before running it, jump to the [Rust installation command decoder](rust-installation-command-decoder) section.

When prompted, enter 1

```bash
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
> 1
```

Then either open a new shell (to reload your PATH variable which now includes paths to Rust executables), or enter this in your current shell to activate that Rust env

```bash
$ source "$HOME/.cargo/env"
```

and check that Rust is installed via

```bash
$ rustup -V
```

which should print the versions of rustup and of the Rust compiler.

```bash
rustup 1.25.2 (17db695f1 2023-02-01)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.67.0 (fc594f156 2023-01-24)`
```

And that should be all you have to do to install Rust on Linux or Mac OS.



## Optional: Looking closer 

### Installation with outputs

Enter this command to get and run the rustup install script.

```bash
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```



```bash

info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  $MY_HOME_DIR/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  $MY_HOME_DIR/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  $MY_HOME_DIR/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  $MY_HOME_DIR/.profile
  $MY_HOME_DIR/.bashrc

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
```

To that prompt, I enter 1.

```bash
>1

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2023-01-26, rust version 1.67.0 (fc594f156 2023-01-24)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 19.3 MiB /  19.3 MiB (100 %)  12.1 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 29.3 MiB /  29.3 MiB (100 %)  15.2 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 67.7 MiB /  67.7 MiB (100 %)  17.7 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.67.0 (fc594f156 2023-01-24)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

Then enter that command to get Rust commands into the namespace.


#### Rust installation command decoder
Nearly developer or engineer who installs Rust will look at these installation instructions, so I'm not really worried about malicious code. Still, I just don't like to just run commands without understanding what they do.

* `curl`: a tool for transferring data from or to servers via some transfer protocol.
* `--proto '=https'`:
    * the `--proto` option tells `curl` which protocols it can use in the transfer. 
    * the `=` in `=https` means "permit **only** this protocol", and `https` is, well, the `https` protocol.
* `--tlsv1.3`:
    * the `--tlsv1.3` option forces `curl` to use TLS (Transport Layer Security) version 1.3 or higher when connecting to a remote server.
* `https://sh.rustup.rs`:
    * this is the URL of the remote server.
* `-sSf`:
    * `-s`: silent (surpress outputs), `-S`: show errors, `-sS` together: supress outputs except for errors.
    * `-f`: fail silently.
    * I'm not 100% on how `-sS` and `-f` work together, but I think the goal is tp prevent piping errors to `sh`
* `|`: pipe
* `sh`: basic shell

So this command sends a request to the server listening to https://sh.rustup.rs, requesting the server to send any response back using transfer protocol HTTPS (which is short for HyperText Transfer Protocol Secure), and then that response is piped to `sh` to run.

Entering https://sh.rustup.rs into a browser will trigger the download of a shell script named `rustup-init.sh`, and I've included a `curl` command to `-o`utput that shell script to a file in the `assets/` directory of this project.

```bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -o assets/rustup-init.sh
```

Looking through that script, there's more than I'm going to fully decode, but scanning through, it looks like most of the code is focused on determining the system architecture (ie OS, chip instruction set, endianness, etc) and on validating the transported file.