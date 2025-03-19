# key-names

[![Latest version](https://img.shields.io/crates/v/key-names.svg)](https://crates.io/crates/key-names)
[![Documentation](https://docs.rs/key-names/badge.svg)](https://docs.rs/key-names)

Platform-aware keyboard key names for Rust applications. This crate is a fork of [keyboard-keynames](https://crates.io/crates/keyboard-keynames) that adds a few extra features including [winit](https://crates.io/crates/winit) integration.

Features:

- Generates user-friendly strings for modifier keys that respect platform conventions
- Generates user-friendly strings for keys based on scancode and current keyboard layout
- Supports Windows, Linux (X11 and Wayland), macOS, and web

This crate currently queries the keyboard layout only on Windows and Linux (X11 and Wayland). On macOS and web, there is instead a hard-coded table based on the US QWERTY layout. I would love a PR that adds support for querying the layout on either of these.

Run `cargo run --example all_keys` to see the key names produced by this library.

## Modifier names and order

| Platform | Modifier names and order          |
| -------- | --------------------------------- |
| Windows  | Ctrl + Shift + Alt + Win + ...    |
| Linux    | Ctrl + Shift + Alt + Super + ...  |
| macOS    | Ctrl + Option + Shift + Cmd + ... |
| Web      | Ctrl + Shift + Alt + Super + ...  |

## Contributing

Bugfix PRs welcome! Before investing time & effort into a new feature, it might be good to open an issue to discuss. I made this crate initially for the needs of [Hyperspeedcube](https://github.com/HactarCE/Hyperspeedcube), and I'm open to adding more features if people would find it useful and it's possible to support cross-platform.
