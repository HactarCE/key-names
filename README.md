# scancode-convert-rs

Platform-aware keyboard scancode handling for Rust applications

Features:

- Generates user-friendly strings for modifier keys that respect platform conventions
- Generates user-friendly strings for keys based on scancode and current keyboard layout
- Supports Windows, Linux (X11 and Wayland), and macOS

Note that on macOS there does not appear to be an API for getting key names from scancodes, so there is instead a hard-coded table based on the US QWERTY layout. This isn't ideal, but at least it's something.

## Modifier names and order

| OS      | Modifier names and order          |
| ------- | --------------------------------- |
| Windows | Ctrl + Shift + Alt + Win + ...    |
| Linux   | Ctrl + Shift + Alt + Super + ...  |
| macOS   | Ctrl + Option + Shift + Cmd + ... |
