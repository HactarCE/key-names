//! Platform-aware keyboard key name handling for Rust applications.

#![warn(missing_docs)]

use winit::keyboard::{Key, NativeKey, NativeKeyCode, PhysicalKey};

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

/// Returns a human-friendly name for a physical key using the operating
/// system's API when possible.
///
/// On Windows and Linux, this queries the user's keyboard layout. On macOS and
/// web, hard-coded key names are used.
pub fn physical_key_name(physical_key: PhysicalKey) -> String {
    os::try_physical_key_name(physical_key).unwrap_or_else(|| match physical_key {
        PhysicalKey::Code(key_code) => format!("{key_code:?}"),
        PhysicalKey::Unidentified(native_key_code) => match native_key_code {
            NativeKeyCode::Unidentified => "<unknown>".to_string(),
            NativeKeyCode::Android(sc) => format!("SC{sc}"),
            NativeKeyCode::MacOS(sc) => format!("SC{sc}"),
            NativeKeyCode::Windows(sc) => format!("SC{sc}"),
            NativeKeyCode::Xkb(sc) => format!("SC{sc}"),
        },
    })
}

/// Returns a human-friendly name for a virtual key.
///
/// Letters are uppercased and some special keys are given OS-specific names
/// (such as "Win" on Windows vs. "Super" on Linux vs. "Command" on macOS).
pub fn key_name(key: Key) -> String {
    match key {
        Key::Named(named_key) => match os::os_specific_key_name(named_key) {
            Some(name) => name.to_string(),
            None => format!("{named_key:?}"),
        },
        Key::Character(c) => c.to_ascii_uppercase(),
        Key::Unidentified(native_key) => match native_key {
            NativeKey::Unidentified => "<unknown>".to_string(),
            NativeKey::Android(sc) => format!("SC{sc}"),
            NativeKey::MacOS(sc) => format!("SC{sc}"),
            NativeKey::Windows(sc) => format!("SC{sc}"),
            NativeKey::Xkb(sc) => format!("SC{sc}"),
            NativeKey::Web(smol_str) => format!("{smol_str}"),
        },
        Key::Dead(None) => "<unknown>".to_string(),
        Key::Dead(Some(c)) => c.into(),
    }
}
