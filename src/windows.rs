use super::*;

pub const ALT_STR: &str = "Alt";
pub const LOGO_STR: &str = "Win";
pub const MODIFIERS_ORDER: &str = "csam"; // Ctrl + Shift + Alt + Meta

pub const SC_INVALID: u16 = 0x0000;
pub const SC_TO_KEY_MAPPING: fn(u16) -> KeyMapping = KeyMapping::Win;
pub const KEY_MAP_TO_SC: fn(KeyMap) -> u16 = |k| k.win;

pub fn scancode_name(sc: u16) -> String {
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

/// Stolen shamelessly from winit:
/// https://github.com/rust-windowing/winit/blob/bcd76d47186b074e536ca5ab9714953931796243/src/platform_impl/windows/event.rs#L186-L361
#[cfg(feature = "winit")]
pub fn key_map_to_winit_vkey(key: KeyMap) -> Option<winit::event::VirtualKeyCode> {
    use winapi::um::winuser::*;
    use winit::event::VirtualKeyCode;
    let vkey = key.win as _;
    // VK_* codes are documented here https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
    match vkey {
        //VK_LBUTTON => Some(VirtualKeyCode::Lbutton),
        //VK_RBUTTON => Some(VirtualKeyCode::Rbutton),
        //VK_CANCEL => Some(VirtualKeyCode::Cancel),
        //VK_MBUTTON => Some(VirtualKeyCode::Mbutton),
        //VK_XBUTTON1 => Some(VirtualKeyCode::Xbutton1),
        //VK_XBUTTON2 => Some(VirtualKeyCode::Xbutton2),
        VK_BACK => Some(VirtualKeyCode::Back),
        VK_TAB => Some(VirtualKeyCode::Tab),
        //VK_CLEAR => Some(VirtualKeyCode::Clear),
        VK_RETURN => Some(VirtualKeyCode::Return),
        VK_LSHIFT => Some(VirtualKeyCode::LShift),
        VK_RSHIFT => Some(VirtualKeyCode::RShift),
        VK_LCONTROL => Some(VirtualKeyCode::LControl),
        VK_RCONTROL => Some(VirtualKeyCode::RControl),
        VK_LMENU => Some(VirtualKeyCode::LAlt),
        VK_RMENU => Some(VirtualKeyCode::RAlt),
        VK_PAUSE => Some(VirtualKeyCode::Pause),
        VK_CAPITAL => Some(VirtualKeyCode::Capital),
        VK_KANA => Some(VirtualKeyCode::Kana),
        //VK_HANGUEL => Some(VirtualKeyCode::Hanguel),
        //VK_HANGUL => Some(VirtualKeyCode::Hangul),
        //VK_JUNJA => Some(VirtualKeyCode::Junja),
        //VK_FINAL => Some(VirtualKeyCode::Final),
        //VK_HANJA => Some(VirtualKeyCode::Hanja),
        VK_KANJI => Some(VirtualKeyCode::Kanji),
        VK_ESCAPE => Some(VirtualKeyCode::Escape),
        VK_CONVERT => Some(VirtualKeyCode::Convert),
        VK_NONCONVERT => Some(VirtualKeyCode::NoConvert),
        //VK_ACCEPT => Some(VirtualKeyCode::Accept),
        //VK_MODECHANGE => Some(VirtualKeyCode::Modechange),
        VK_SPACE => Some(VirtualKeyCode::Space),
        VK_PRIOR => Some(VirtualKeyCode::PageUp),
        VK_NEXT => Some(VirtualKeyCode::PageDown),
        VK_END => Some(VirtualKeyCode::End),
        VK_HOME => Some(VirtualKeyCode::Home),
        VK_LEFT => Some(VirtualKeyCode::Left),
        VK_UP => Some(VirtualKeyCode::Up),
        VK_RIGHT => Some(VirtualKeyCode::Right),
        VK_DOWN => Some(VirtualKeyCode::Down),
        //VK_SELECT => Some(VirtualKeyCode::Select),
        //VK_PRINT => Some(VirtualKeyCode::Print),
        //VK_EXECUTE => Some(VirtualKeyCode::Execute),
        VK_SNAPSHOT => Some(VirtualKeyCode::Snapshot),
        VK_INSERT => Some(VirtualKeyCode::Insert),
        VK_DELETE => Some(VirtualKeyCode::Delete),
        //VK_HELP => Some(VirtualKeyCode::Help),
        48 => Some(VirtualKeyCode::Key0),
        49 => Some(VirtualKeyCode::Key1),
        50 => Some(VirtualKeyCode::Key2),
        51 => Some(VirtualKeyCode::Key3),
        52 => Some(VirtualKeyCode::Key4),
        53 => Some(VirtualKeyCode::Key5),
        54 => Some(VirtualKeyCode::Key6),
        55 => Some(VirtualKeyCode::Key7),
        56 => Some(VirtualKeyCode::Key8),
        57 => Some(VirtualKeyCode::Key9),
        65 => Some(VirtualKeyCode::A),
        66 => Some(VirtualKeyCode::B),
        67 => Some(VirtualKeyCode::C),
        68 => Some(VirtualKeyCode::D),
        69 => Some(VirtualKeyCode::E),
        70 => Some(VirtualKeyCode::F),
        71 => Some(VirtualKeyCode::G),
        72 => Some(VirtualKeyCode::H),
        73 => Some(VirtualKeyCode::I),
        74 => Some(VirtualKeyCode::J),
        75 => Some(VirtualKeyCode::K),
        76 => Some(VirtualKeyCode::L),
        77 => Some(VirtualKeyCode::M),
        78 => Some(VirtualKeyCode::N),
        79 => Some(VirtualKeyCode::O),
        80 => Some(VirtualKeyCode::P),
        81 => Some(VirtualKeyCode::Q),
        82 => Some(VirtualKeyCode::R),
        83 => Some(VirtualKeyCode::S),
        84 => Some(VirtualKeyCode::T),
        85 => Some(VirtualKeyCode::U),
        86 => Some(VirtualKeyCode::V),
        87 => Some(VirtualKeyCode::W),
        88 => Some(VirtualKeyCode::X),
        89 => Some(VirtualKeyCode::Y),
        90 => Some(VirtualKeyCode::Z),
        VK_LWIN => Some(VirtualKeyCode::LWin),
        VK_RWIN => Some(VirtualKeyCode::RWin),
        VK_APPS => Some(VirtualKeyCode::Apps),
        VK_SLEEP => Some(VirtualKeyCode::Sleep),
        VK_NUMPAD0 => Some(VirtualKeyCode::Numpad0),
        VK_NUMPAD1 => Some(VirtualKeyCode::Numpad1),
        VK_NUMPAD2 => Some(VirtualKeyCode::Numpad2),
        VK_NUMPAD3 => Some(VirtualKeyCode::Numpad3),
        VK_NUMPAD4 => Some(VirtualKeyCode::Numpad4),
        VK_NUMPAD5 => Some(VirtualKeyCode::Numpad5),
        VK_NUMPAD6 => Some(VirtualKeyCode::Numpad6),
        VK_NUMPAD7 => Some(VirtualKeyCode::Numpad7),
        VK_NUMPAD8 => Some(VirtualKeyCode::Numpad8),
        VK_NUMPAD9 => Some(VirtualKeyCode::Numpad9),
        VK_MULTIPLY => Some(VirtualKeyCode::NumpadMultiply),
        VK_ADD => Some(VirtualKeyCode::NumpadAdd),
        //VK_SEPARATOR => Some(VirtualKeyCode::Separator),
        VK_SUBTRACT => Some(VirtualKeyCode::NumpadSubtract),
        VK_DECIMAL => Some(VirtualKeyCode::NumpadDecimal),
        VK_DIVIDE => Some(VirtualKeyCode::NumpadDivide),
        VK_F1 => Some(VirtualKeyCode::F1),
        VK_F2 => Some(VirtualKeyCode::F2),
        VK_F3 => Some(VirtualKeyCode::F3),
        VK_F4 => Some(VirtualKeyCode::F4),
        VK_F5 => Some(VirtualKeyCode::F5),
        VK_F6 => Some(VirtualKeyCode::F6),
        VK_F7 => Some(VirtualKeyCode::F7),
        VK_F8 => Some(VirtualKeyCode::F8),
        VK_F9 => Some(VirtualKeyCode::F9),
        VK_F10 => Some(VirtualKeyCode::F10),
        VK_F11 => Some(VirtualKeyCode::F11),
        VK_F12 => Some(VirtualKeyCode::F12),
        VK_F13 => Some(VirtualKeyCode::F13),
        VK_F14 => Some(VirtualKeyCode::F14),
        VK_F15 => Some(VirtualKeyCode::F15),
        VK_F16 => Some(VirtualKeyCode::F16),
        VK_F17 => Some(VirtualKeyCode::F17),
        VK_F18 => Some(VirtualKeyCode::F18),
        VK_F19 => Some(VirtualKeyCode::F19),
        VK_F20 => Some(VirtualKeyCode::F20),
        VK_F21 => Some(VirtualKeyCode::F21),
        VK_F22 => Some(VirtualKeyCode::F22),
        VK_F23 => Some(VirtualKeyCode::F23),
        VK_F24 => Some(VirtualKeyCode::F24),
        VK_NUMLOCK => Some(VirtualKeyCode::Numlock),
        VK_SCROLL => Some(VirtualKeyCode::Scroll),
        VK_BROWSER_BACK => Some(VirtualKeyCode::NavigateBackward),
        VK_BROWSER_FORWARD => Some(VirtualKeyCode::NavigateForward),
        VK_BROWSER_REFRESH => Some(VirtualKeyCode::WebRefresh),
        VK_BROWSER_STOP => Some(VirtualKeyCode::WebStop),
        VK_BROWSER_SEARCH => Some(VirtualKeyCode::WebSearch),
        VK_BROWSER_FAVORITES => Some(VirtualKeyCode::WebFavorites),
        VK_BROWSER_HOME => Some(VirtualKeyCode::WebHome),
        VK_VOLUME_MUTE => Some(VirtualKeyCode::Mute),
        VK_VOLUME_DOWN => Some(VirtualKeyCode::VolumeDown),
        VK_VOLUME_UP => Some(VirtualKeyCode::VolumeUp),
        VK_MEDIA_NEXT_TRACK => Some(VirtualKeyCode::NextTrack),
        VK_MEDIA_PREV_TRACK => Some(VirtualKeyCode::PrevTrack),
        VK_MEDIA_STOP => Some(VirtualKeyCode::MediaStop),
        VK_MEDIA_PLAY_PAUSE => Some(VirtualKeyCode::PlayPause),
        VK_LAUNCH_MAIL => Some(VirtualKeyCode::Mail),
        VK_LAUNCH_MEDIA_SELECT => Some(VirtualKeyCode::MediaSelect),
        /*VK_LAUNCH_APP1 => Some(VirtualKeyCode::Launch_app1),
        VK_LAUNCH_APP2 => Some(VirtualKeyCode::Launch_app2),*/
        VK_OEM_PLUS => Some(VirtualKeyCode::Equals),
        VK_OEM_COMMA => Some(VirtualKeyCode::Comma),
        VK_OEM_MINUS => Some(VirtualKeyCode::Minus),
        VK_OEM_PERIOD => Some(VirtualKeyCode::Period),
        VK_OEM_1 => map_text_keys(vkey as _),
        VK_OEM_2 => map_text_keys(vkey as _),
        VK_OEM_3 => map_text_keys(vkey as _),
        VK_OEM_4 => map_text_keys(vkey as _),
        VK_OEM_5 => map_text_keys(vkey as _),
        VK_OEM_6 => map_text_keys(vkey as _),
        VK_OEM_7 => map_text_keys(vkey as _),
        /* VK_OEM_8 => Some(VirtualKeyCode::Oem_8), */
        VK_OEM_102 => Some(VirtualKeyCode::OEM102),
        /*VK_PROCESSKEY => Some(VirtualKeyCode::Processkey),
        VK_PACKET => Some(VirtualKeyCode::Packet),
        VK_ATTN => Some(VirtualKeyCode::Attn),
        VK_CRSEL => Some(VirtualKeyCode::Crsel),
        VK_EXSEL => Some(VirtualKeyCode::Exsel),
        VK_EREOF => Some(VirtualKeyCode::Ereof),
        VK_PLAY => Some(VirtualKeyCode::Play),
        VK_ZOOM => Some(VirtualKeyCode::Zoom),
        VK_NONAME => Some(VirtualKeyCode::Noname),
        VK_PA1 => Some(VirtualKeyCode::Pa1),
        VK_OEM_CLEAR => Some(VirtualKeyCode::Oem_clear),*/
        _ => None,
    }
}
// This is needed as windows doesn't properly distinguish
// some virtual key codes for different keyboard layouts
#[cfg(feature = "winit")]
fn map_text_keys(
    win_virtual_key: winapi::shared::minwindef::UINT,
) -> Option<winit::event::VirtualKeyCode> {
    use winit::event::VirtualKeyCode;
    let char_key = unsafe {
        winapi::um::winuser::MapVirtualKeyA(win_virtual_key, winapi::um::winuser::MAPVK_VK_TO_CHAR)
    } & 0x7FFF;
    match char::from_u32(char_key) {
        Some(';') => Some(VirtualKeyCode::Semicolon),
        Some('/') => Some(VirtualKeyCode::Slash),
        Some('`') => Some(VirtualKeyCode::Grave),
        Some('[') => Some(VirtualKeyCode::LBracket),
        Some(']') => Some(VirtualKeyCode::RBracket),
        Some('\'') => Some(VirtualKeyCode::Apostrophe),
        Some('\\') => Some(VirtualKeyCode::Backslash),
        _ => None,
    }
}
