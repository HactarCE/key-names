# key-names

Platform-aware keyboard key name handling for Rust applications. This crate is roughly a fork of [keyboard-keynames](https://crates.io/crates/keyboard-keynames), with a few extra features as well as integration with the [keycode](https://docs.rs/keycode/0.3.0/keycode/index.html) crate and [`winit::event::VirtualKeyCode`](https://docs.rs/winit/0.26.1/winit/event/enum.VirtualKeyCode.html) (requires `winit` feature).

Features:

- Generates user-friendly strings for modifier keys that respect platform conventions
- Generates user-friendly strings for keys based on scancode and current keyboard layout
- Supports Windows, Linux (X11 and Wayland), macOS, and web

Note that on macOS there does not appear to be an API for getting key names from scancodes, so there is instead a hard-coded table based on the US QWERTY layout. This isn't ideal, but at least it's something.

Run `cargo run --example all_keys` to see the key names produced by this library.

## Web support

On web, there's no standardized scancodes so this crate makes yet another set of arbitrary scancodes. Compiling to web requires the `winit` feature, since it's specifically made to help build a compatibility layer to work with winit on the web.

On web, winit 0.27 switches the virtual key code and scancode of each key input event, so this crate provides the functions `web::winit_vkey_to_arbitrary_scancode()` and `web::ascii_to_keycode()` to switch them back.

## Modifier names and order

| Platform | Modifier names and order          |
| -------- | --------------------------------- |
| Windows  | Ctrl + Shift + Alt + Win + ...    |
| Linux    | Ctrl + Shift + Alt + Super + ...  |
| macOS    | Ctrl + Option + Shift + Cmd + ... |
| Web      | Ctrl + Shift + Alt + Logo + ...   |

## Contributing

Bugfix PRs welcome! Before investing time & effort into a new feature, it might be good to open an issue to discuss. I made this crate initially for the needs of [Hyperspeedcube](https://github.com/HactarCE/Hyperspeedcube), and I'm open to adding more features if people would find it useful and it's possible to support cross-platform.

Also, if anyone knows how to set up `Cargo.toml` to force-enable the `winit` feature on web, please submit a PR!
