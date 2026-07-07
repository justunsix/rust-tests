# Rust Tests

Tests with Rust language

## Prerequisites

- [Rust Tools](https://doc.rust-lang.org/book/ch01-01-installation.html)

Optional for language support:

- Install [rust-analyzer](https://rust-analyzer.github.io/manual.html#installation)
  - For example in VSCode, install the rust-analyzer extension or install
    with rustup `rustup component add rust-analyzer`

### Other Options

- Develop in browser [![Open in Ona (formerly Gitpod)](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/justunsix/rust-tests)
- Use `nix` and install all tools with `nix develop` with flake in this repository

## Usage

### Using Cargo

```shell

  # Go into relevant project directory
  cd <project>
  # Run project
  cargo run

```

### Using Makefile

- Can be used if make is installed on Linux, MacOS, or Windows
  with git bash and make installed

```shell

  # Install rust-analyzer
  make install

  # See available tasks to run like run, build
  make

```
