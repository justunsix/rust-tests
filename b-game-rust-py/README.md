# Maturin User Guide - Tutorial

Follows [Maturin's Tutorial](https://www.maturin.rs/tutorial) to use a library in Rust in Python

```shell
# Init
cargo new --lib --edition 2021 guessing-game
# Update Cargo.toml with rand and pyo3 dependencies

# Create Python Virtualenv
# And install maturin
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# Update lib.rs as needed and build
maturin develop

# Test Rust library in Python
python guessing-game-test.py
```
