# Rust Tests

Tests with Rust language

## Prerequisites

- [Rust Tools](https://doc.rust-lang.org/book/ch01-01-installation.html)

Optional for language support:

- Install [rust-analyzer](https://rust-analyzer.github.io/manual.html#installation)
  - For example in VSCode, install the rust-analyzer extension or install with rustup `rustup component add rust-analyzer`

## Usage

### Using Cargo

``` shell

  # Go into relevant project directory
  cd <project>
  # Run project
  cargo run

```

### Using Makefile

- Can be used if make is installed on Linux, MacOS, or Windows with git bash and make installed

``` shell

  # Install rust-analyzer
  make install

  # See available programs to run
  make

  # Choose program to run
  make <run...>

```
