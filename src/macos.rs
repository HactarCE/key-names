use super::*;
#[cfg(feature = "winit")]
pub use crate::common::key_map_to_winit_vkey;

pub const ALT_STR: &str = "Option";
pub const LOGO_STR: &str = "Cmd";
pub const MODIFIERS_ORDER: &str = "casm"; // Ctrl + Alt + Shift + Meta

pub const SC_INVALID: u16 = 0xFFFF;
pub const SC_TO_KEY_MAPPING: fn(u16) -> KeyMapping = KeyMapping::Mac;
pub const KEY_MAP_TO_SC: fn(KeyMap) -> u16 = |k| k.mac;

pub fn scancode_name(sc: u16) -> String {
    match super::sc_to_key(sc) {
        Some(key) => key_name(key),
        None => format!("SC{}", sc),
    }
}

fn key_name(key: KeyMappingCode) -> String {
    use KeyMappingCode::*;
    match key {
        Backquote => "`",
        Backslash => "\\",
        BracketLeft => "[",
        BracketRight => "]",
        Comma => ",",
        Digit0 => "0",
        Digit1 => "1",
        Digit2 => "2",
        Digit3 => "3",
        Digit4 => "4",
        Digit5 => "5",
        Digit6 => "6",
        Digit7 => "7",
        Digit8 => "8",
        Digit9 => "9",
        Equal => "=",

        KeyA => "A",
        KeyB => "B",
        KeyC => "C",
        KeyD => "D",
        KeyE => "E",
        KeyF => "F",
        KeyG => "G",
        KeyH => "H",
        KeyI => "I",
        KeyJ => "J",
        KeyK => "K",
        KeyL => "L",
        KeyM => "M",
        KeyN => "N",
        KeyO => "O",
        KeyP => "P",
        KeyQ => "Q",
        KeyR => "R",
        KeyS => "S",
        KeyT => "T",
        KeyU => "U",
        KeyV => "V",
        KeyW => "W",
        KeyX => "X",
        KeyY => "Y",
        KeyZ => "Z",
        Minus => "-",
        Period => ".",
        Quote => "'",
        Semicolon => ";",
        Slash => "/",

        AltLeft => "Option",
        AltRight => "Right Option",
        ControlLeft => "Control",
        ControlRight => "Right Control",
        MetaLeft => "Command",
        MetaRight => "Right Command", // doesn't exist anyway
        ShiftLeft => "Shift",
        ShiftRight => "Right Shift",

        Backspace => "Delete",
        Enter => "Return",

        ArrowDown => "Down",
        ArrowLeft => "Left",
        ArrowRight => "Right",
        ArrowUp => "Up",
        Delete => "Forward Delete",

        Escape => "Esc",

        other => return format!("{:?}", other),
    }
    .to_owned()
}
