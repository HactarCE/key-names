use winit::keyboard::{KeyCode, NamedKey, PhysicalKey};

pub const ALT_STR: &str = "Option";
pub const LOGO_STR: &str = "Cmd";
pub const MODIFIERS_ORDER: &str = "casm"; // Ctrl + Alt + Shift + Meta

pub fn os_specific_key_name(key: NamedKey) -> Option<&'static str> {
    match key {
        NamedKey::Alt => Some("Option"),
        NamedKey::AltGraph => Some("Right Option"),
        NamedKey::ArrowDown => Some("Down"),
        NamedKey::ArrowLeft => Some("Left"),
        NamedKey::ArrowRight => Some("Right"),
        NamedKey::ArrowUp => Some("Up"),
        NamedKey::Backspace => Some("Delete"),
        NamedKey::Delete => Some("Forward Delete"),
        NamedKey::Enter => Some("Return"),
        NamedKey::Escape => Some("Esc"),
        NamedKey::Super => Some("Command"),
        _ => None,
    }
}

pub fn try_physical_key_name(physical_key: PhysicalKey) -> Option<String> {
    let s = match physical_key {
        PhysicalKey::Code(key_code) => match key_code {
            KeyCode::Backquote => "`",
            KeyCode::Backslash => "\\",
            KeyCode::BracketLeft => "[",
            KeyCode::BracketRight => "]",
            KeyCode::Comma => ",",
            KeyCode::Digit0 => "0",
            KeyCode::Digit1 => "1",
            KeyCode::Digit2 => "2",
            KeyCode::Digit3 => "3",
            KeyCode::Digit4 => "4",
            KeyCode::Digit5 => "5",
            KeyCode::Digit6 => "6",
            KeyCode::Digit7 => "7",
            KeyCode::Digit8 => "8",
            KeyCode::Digit9 => "9",
            KeyCode::Equal => "=",

            KeyCode::KeyA => "A",
            KeyCode::KeyB => "B",
            KeyCode::KeyC => "C",
            KeyCode::KeyD => "D",
            KeyCode::KeyE => "E",
            KeyCode::KeyF => "F",
            KeyCode::KeyG => "G",
            KeyCode::KeyH => "H",
            KeyCode::KeyI => "I",
            KeyCode::KeyJ => "J",
            KeyCode::KeyK => "K",
            KeyCode::KeyL => "L",
            KeyCode::KeyM => "M",
            KeyCode::KeyN => "N",
            KeyCode::KeyO => "O",
            KeyCode::KeyP => "P",
            KeyCode::KeyQ => "Q",
            KeyCode::KeyR => "R",
            KeyCode::KeyS => "S",
            KeyCode::KeyT => "T",
            KeyCode::KeyU => "U",
            KeyCode::KeyV => "V",
            KeyCode::KeyW => "W",
            KeyCode::KeyX => "X",
            KeyCode::KeyY => "Y",
            KeyCode::KeyZ => "Z",
            KeyCode::Minus => "-",
            KeyCode::Period => ".",
            KeyCode::Quote => "'",
            KeyCode::Semicolon => ";",
            KeyCode::Slash => "/",

            KeyCode::AltLeft => "Option",
            KeyCode::AltRight => "Right Option",
            KeyCode::ControlLeft => "Control",
            KeyCode::ControlRight => "Right Control",
            KeyCode::SuperLeft => "Command",
            KeyCode::SuperRight => "Right Command", // doesn't exist anyway
            KeyCode::ShiftLeft => "Shift",
            KeyCode::ShiftRight => "Right Shift",

            KeyCode::Backspace => "Delete",
            KeyCode::Enter => "Return",

            KeyCode::ArrowDown => "Down",
            KeyCode::ArrowLeft => "Left",
            KeyCode::ArrowRight => "Right",
            KeyCode::ArrowUp => "Up",
            KeyCode::Delete => "Forward Delete",

            KeyCode::Escape => "Esc",

            _ => return None,
        },

        PhysicalKey::Unidentified(_) => return None,
    };

    Some(s.to_string())
}
