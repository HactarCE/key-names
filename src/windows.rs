use super::*;

const INVALID: u16 = 0x0000;

#[derive(Debug, Default, Copy, Clone)]
pub struct WindowsKeymap;
impl OsKeymap for WindowsKeymap {
    fn logo_str(&self) -> &'static str {
        "Win"
    }

    fn decode_scancode(&self, sc: u16) -> Option<KeyMappingCode> {
        KeyMap::try_from(KeyMapping::Win(sc as u16)).ok()?.code
    }
    fn encode_scancode(&self, key: KeyMappingCode) -> Option<u16> {
        Some(KeyMap::try_from(KeyMapping::Code(Some(key))).ok()?.win as u16)
            .filter(|&sc| sc != INVALID)
    }

    fn scancode_name(&self, sc: u16) -> String {
        // This code is based on Frinksy's `keyboard-keynames` crate:
        // https://gitlab.com/Frinksy/keyboard-keynames/-/blob/master/src/platform/windows/key_layout.rs

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

        // SAFETY: `utf16_key_name` is not borrowed, and `GetKeyNameTextW()`
        // returns 0 if it fails.
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
}
