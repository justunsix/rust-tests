# Key Mapper

Listens for numpad key presses (1-9) and remaps them to Super+number
keyboard shortcuts. Useful for desktop environments where workspace
switching is bound to Super+number such as Windows and GNOME.

## Build

```
cargo build --release
```

## Run

```
cargo run --release
```

Press Ctrl+C to stop.

## Dependencies

- `enigo` - simulates keyboard input
- `rdev` - listens for operating system key events

## Notes

- Requires permission to listen to keyboard events. On Linux, the process
  must run as root (not recommended) or as a member of the `input` group or `plugdev` group per
  ([rdev documentation](https://crates.io/crates/rdev). If in doubt, add user to both groups.
- Only numpad keys without NumLock are mapped (Keypad number 1 to 9).
- Each keypress triggers once; holding a key does not repeat.

## About Crates Used

- [rdev - Listen and send keyboard and mouse events on Windows, Linux and MacOS](https://lib.rs/crates/rdev)
- [enigo - Cross-platform (Linux, Windows, macOS & BSD) library to simulate keyboard and mouse events](https://lib.rs/crates/enigo)
