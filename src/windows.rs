use winit::keyboard::{NamedKey, PhysicalKey};
use winit::platform::scancode::PhysicalKeyExtScancode;

pub const ALT_STR: &str = "Alt";
pub const LOGO_STR: &str = "Win";
pub const MODIFIERS_ORDER: &str = "csam"; // Ctrl + Shift + Alt + Meta

pub fn os_specific_key_name(key: NamedKey) -> Option<&'static str> {
    match key {
        NamedKey::AltGraph => Some("AltGr"),
        NamedKey::ArrowDown => Some("Down"),
        NamedKey::ArrowLeft => Some("Left"),
        NamedKey::ArrowRight => Some("Right"),
        NamedKey::ArrowUp => Some("Up"),
        NamedKey::Super => Some("Windows"),
        _ => None,
    }
}

pub fn physical_key_name(physical_key: PhysicalKey) -> String {
    match physical_key.to_scancode() {
        Some(sc) => scancode_name(sc as u16),
        None => format!("{physical_key:?}"),
    }
}

fn scancode_name(sc: u16) -> String {
    // This code is based on Frinksy's `keyboard-keynames` crate:
    // https://gitlab.com/Frinksy/keyboard-keynames/-/blob/8b4dff8f2658a15b6ec55290a2c934f6f3819b58/src/platform/windows/key_layout.rs

    // Convert the scancode.
    let mut l_param = sc as cty::c_long;
    l_param <<= 16;

    // Check if 0xE0 escape sequence is present and set extended key flag.
    if (sc & 0x0000FF00) == 0xE000 {
        l_param |= 1 << 24;
    }

    // Allocate a buffer for the UTF-16 encoded key name.
    const BUFFER_SIZE: usize = 32;
    let mut utf16_key_name = vec![0_u16; BUFFER_SIZE];

    // SAFETY: `utf16_key_name` is not borrowed, and `GetKeyNameTextW()` returns
    // 0 if it fails.
    let name_len = unsafe {
        winapi::um::winuser::GetKeyNameTextW(
            l_param,
            utf16_key_name.as_mut_ptr(),
            BUFFER_SIZE as cty::c_int,
        )
    };

    if name_len == 0 {
        return format!("SC{}", sc);
    }

    // Truncate the array to the size of the key name.
    utf16_key_name.truncate(name_len as usize);

    // Decode the UTF-16 string.
    String::from_utf16_lossy(&utf16_key_name)
}
