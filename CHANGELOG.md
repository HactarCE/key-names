# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[@ArvinSKushwaha]: https://github.com/ArvinSKushwaha

## [3.0.0] - 2025-02-05

- **BREAKING:** Removed keycodes dependency
- **BREAKING:** Updated winit from 0.27 to 0.30.8 and added it as a non-optional dependency
- **BREAKING:** Removed `sc_to_key()`, `key_to_sc()`, and `key_to_winit_vkey()`
- **BREAKING:** Replaced `scancode_name(sc: u16)` with `physical_key_name(physical_key: winit::keyboard::PhysicalKey)`
- **BREAKING:** Replaced `key_name(key: keycode::KeyMappingCode)` with `key_name(key: winit::keyboard::Key)`
- Changed some key names

## [2.0.0] - 2025-02-05

- **BREAKING:** Updated wayland-client from 0.29 to 0.31.8
- **BREAKING:** Updated winit from 0.27 to 0.30.8
- **BREAKING:** Updated xcb from 0.9.0 to 1.5.0
- **BREAKING:** Updated xkbcommon from 0.4.1 to 0.8.0
- Changed to use X11 fallback when using XWayland ([@ArvinSKushwaha] [#8](https://github.com/HactarCE/key-names/pull/8))
- Fixed incorrect virtual key code conversion on Linux

## [1.5.1] - 2023-01-21

- Fixed incorrect key names on web

## [1.5.0] - 2023-01-18

- Added web support
  - `web::winit_vkey_to_arbitrary_scancode()`
  - `web::arbitrary_scancode_to_winit_vkey()`
  - `web::ascii_to_keycode()`
- Upgraded `keycode` from `0.3` to `0.4`
- Upgraded `winit` from `0.26` to `0.27`

## [1.4.0] - 2022-12-02

- Fixed incorrect key names on Linux ([#5](https://github.com/HactarCE/key-names/issues/5))

## [1.3.0] - 2022-09-03

- Switched from XKB scancodes to evdev

## [1.2.0] - 2022-08-13

- Upgraded `xcb` from `^0.8.0` to `^0.9.0`
- Upgraded `xkbcommon` from `^0.4` to `^0.4.1`

## [1.1.0] - 2022-08-12

- Added `MODIFIERS_ORDER` constant

## [1.0.1] - 2022-07-29

- Reverted `xcb` dependency to `^0.8.0` to Fix Linux build

## [1.0.0] - 2022-07-29

- Initial release

[unreleased]: https://github.com/HactarCE/key-names/compare/v3.0.0...HEAD
[3.0.0]: https://github.com/HactarCE/key-names/compare/v2.0.0...v3.0.0
[2.0.0]: https://github.com/HactarCE/key-names/compare/v1.5.1...v2.0.0
[1.5.1]: https://github.com/HactarCE/key-names/compare/v1.5.0...v1.5.1
[1.5.0]: https://github.com/HactarCE/key-names/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/HactarCE/key-names/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/HactarCE/key-names/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/HactarCE/key-names/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/HactarCE/key-names/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/HactarCE/key-names/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/HactarCE/key-names/releases/tag/v1.0.0
