# key-names

Platform-aware keyboard key name handling for Rust applications. This crate is roughly a fork of [keyboard-keynames](https://crates.io/crates/keyboard-keynames), with a few extra features as well as integration with the [keycode](https://docs.rs/keycode/0.3.0/keycode/index.html) crate and [`winit::event::VirtualKeyCode`](https://docs.rs/winit/0.26.1/winit/event/enum.VirtualKeyCode.html) (requires `winit` feature).

Features:

- Generates user-friendly strings for modifier keys that respect platform conventions
- Generates user-friendly strings for keys based on scancode and current keyboard layout
- Supports Windows, Linux (X11 and Wayland), and macOS

Note that on macOS there does not appear to be an API for getting key names from scancodes, so there is instead a hard-coded table based on the US QWERTY layout. This isn't ideal, but at least it's something.

Run `cargo run --example all_keys` to see the key names produced by this library.

## Modifier names and order

| OS      | Modifier names and order          |
| ------- | --------------------------------- |
| Windows | Ctrl + Shift + Alt + Win + ...    |
| Linux   | Ctrl + Shift + Alt + Super + ...  |
| macOS   | Ctrl + Option + Shift + Cmd + ... |

## Contributing

WASM support is highly desirable. PRs welcome!
