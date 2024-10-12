# Rust Setup For Neovim wit Tests

Follows [Rust Setup For Neovim (ft BashBunni)](https://www.youtube.com/watch?v=Mccy6wuq3JE&)

```shell

# Install rust LSP
rustup component add rust-analyzer

# Create new project
cargo new a1-tests 

# run project
cd a1-tests
cargo run

# Tests
# Install tool to monitor for file changes and then do tests
cargo test
cargo install cargo-watch
# Tell cargo watch to run tests on source changes
cargo watch -x test 
```
