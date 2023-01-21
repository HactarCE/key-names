# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

- Fixed incorrect key names on Linux (#5)

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
