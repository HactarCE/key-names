//! Platform-aware keyboard key name handling for Rust applications.

#![warn(missing_docs)]

pub use keycode::*;

mod common;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_arch = "wasm32")]
pub mod web;
#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
use linux as os;
#[cfg(target_os = "macos")]
use macos as os;
#[cfg(target_arch = "wasm32")]
use web as os;
#[cfg(windows)]
use windows as os;

/// OS's conventional modifiers order, represented as an ASCII string containing
/// the characters `csam` for `CTRL`, `SHIFT`, `ALT`, and `META`/`LOGO`
/// respectively in some order.
pub const MODIFIERS_ORDER: &str = os::MODIFIERS_ORDER;

/// OS's conventional name for the <key>Ctrl</key> modifier.
pub const CTRL_STR: &str = "Ctrl";
/// OS's conventional name for the <key>Shift</key> modifier.
pub const SHIFT_STR: &str = "Shift";
/// OS's conventional name for the <key>Alt</key> modifier.
pub const ALT_STR: &str = os::ALT_STR;
/// OS's conventional name for the logo modifier.
pub const LOGO_STR: &str = os::LOGO_STR;

/// Returns a string representing modifiers using the OS's conventional names
/// and ordering. For example, on Windows this function might produce "Ctrl +
/// Shift + Alt + Win + " while on macOS it might produce "Ctrl + Option + Shift
/// + Cmd + ".
pub fn mods_prefix_string(shift: bool, ctrl: bool, alt: bool, logo: bool) -> String {
    let mut ret = String::new();
    for ch in MODIFIERS_ORDER.chars() {
        match ch {
            's' if shift => {
                ret += SHIFT_STR;
                ret += " + ";
            }
            'c' if ctrl => {
                ret += CTRL_STR;
                ret += " + ";
            }
            'a' if alt => {
                ret += ALT_STR;
                ret += " + ";
            }
            'm' if logo => {
                ret += LOGO_STR;
                ret += " + ";
            }
            _ => (),
        }
    }
    ret
}

/// Converts the OS-specific scancode to an OS-independent key mapping code.
pub fn sc_to_key(sc: u16) -> Option<KeyMappingCode> {
    let key_mapping = os::SC_TO_KEY_MAPPING(sc);
    KeyMap::try_from(key_mapping).ok()?.code
}
/// Converts the OS-independent key mapping code back into an OS-specific
/// scancode. This is not guaranteed to produce the original scancode.
pub fn key_to_sc(key: KeyMappingCode) -> Option<u16> {
    let key_map = KeyMap::try_from(KeyMapping::Code(Some(key))).ok()?;
    let sc = os::KEY_MAP_TO_SC(key_map);
    Some(sc).filter(|&sc| sc != os::SC_INVALID)
}

/// Uses the operarting system's API to return a name for the scancode.
pub fn scancode_name(sc: u16) -> String {
    os::scancode_name(sc)
}
/// Uses the operating system's API to return a name for the key.
pub fn key_name(key: KeyMappingCode) -> String {
    match key_to_sc(key) {
        Some(sc) => scancode_name(sc),
        None => format!("{:?}", key),
    }
}

/// Converts the key mapping code to a virtual keycode
#[cfg(feature = "winit")]
pub fn key_to_winit_vkey(key: KeyMappingCode) -> Option<winit::event::VirtualKeyCode> {
    let key_map = KeyMap::try_from(KeyMapping::Code(Some(key))).ok()?;
    os::key_map_to_winit_vkey(key_map)
}
