pub use keycode::*;
use std::fmt;
use thiserror::Error;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(windows)]
mod windows;

#[cfg(windows)]
type Keymap = windows::WindowsKeymap;
#[cfg(target_os = "linux")]
type Keymap = linux::LinuxKeymap;
#[cfg(target_os = "macos")]
type Keymap = macos::MacosKeymap;

pub fn get_keymap() -> Result<Keymap, KeymapError> {
    #[cfg(windows)]
    return Ok(windows::WindowsKeymap);
    #[cfg(target_os = "linux")]
    return linux::LinuxKeymap::new();
    #[cfg(target_os = "macos")]
    return Ok(macos::MacosKeymap);
}

#[derive(Error, Debug)]
pub enum KeymapError {
    #[error("unable to connect to X server")]
    X11Connect,

    #[error("unable to connect to wayland")]
    WaylandConnect,
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("wl_seat not found in available interfaces")]
    MissingWlSeat,
    #[error("wl_seat does not have keyboard capability")]
    MissingKeyboardCapability,
    #[error("failed to create keymap")]
    FailedToCreateKeymap,
}

pub trait OsKeymap: fmt::Debug {
    /// Returns the OS's conventional name for the <key>Ctrl</key> modifier.
    fn ctrl_str(&self) -> &'static str {
        "Ctrl"
    }
    /// Returns the OS's conventional name for the <key>Shift</key> modifier.
    fn shift_str(&self) -> &'static str {
        "Shift"
    }
    /// Returns the OS's conventional name for the <key>Alt</key> modifier.
    fn alt_str(&self) -> &'static str {
        "Alt"
    }
    /// Returns the OS's conventional name for the logo modifier.
    fn logo_str(&self) -> &'static str;

    /// Returns a string representing modifiers using the OS's conventional
    /// names and ordering. For example, on Windows this function might produce
    /// "Ctrl + Shift + Alt + Win + " while on macOS it might produce "Ctrl +
    /// Option + Shift + Cmd + ".
    fn display_mods(&self, shift: bool, ctrl: bool, alt: bool, logo: bool) -> String {
        let mut ret = String::new();
        if ctrl {
            ret += self.ctrl_str();
            ret += " + ";
        }
        if shift {
            ret += self.shift_str();
            ret += " + ";
        }
        if alt {
            ret += self.alt_str();
            ret += " + ";
        }
        if logo {
            ret += self.logo_str();
            ret += " + ";
        }
        ret
    }

    /// Converts the OS-specific scancode to an OS-independent key mapping code.
    fn decode_scancode(&self, sc: u16) -> Option<KeyMappingCode>;
    /// Converts the OS-independent key mapping code back into an OS-specific
    /// scancode. This is not guaranteed to produce the original scancode.
    fn encode_scancode(&self, key: KeyMappingCode) -> Option<u16>;

    /// Uses the operarting system's API to return a name for the scancode.
    fn scancode_name(&self, sc: u16) -> String;
    /// Uses the operating system's API to return a name for the key.
    fn key_name(&self, key: KeyMappingCode) -> String {
        match self.encode_scancode(key) {
            Some(sc) => self.scancode_name(sc),
            None => format!("{:?}", key),
        }
    }
}
