use super::*;

const INVALID: u16 = 0xFFFF;

#[derive(Debug, Default, Copy, Clone)]
pub struct MacosKeymap;
impl OsKeymap for MacosKeymap {
    fn alt_str(&self) -> &'static str {
        "Option"
    }
    fn logo_str(&self) -> &'static str {
        "Cmd"
    }

    fn display_mods(&self, shift: bool, ctrl: bool, alt: bool, logo: bool) -> String {
        let mut ret = String::new();
        if ctrl {
            ret += self.ctrl_str();
            ret += " + ";
        }
        if alt {
            ret += self.alt_str();
            ret += " + ";
        }
        if shift {
            ret += self.shift_str();
            ret += " + ";
        }
        if logo {
            ret += self.logo_str();
            ret += " + ";
        }
        ret
    }

    fn decode_scancode(&self, sc: u16) -> Option<KeyMappingCode> {
        KeyMap::try_from(KeyMapping::Mac(sc as u16)).ok()?.code
    }
    fn encode_scancode(&self, key: KeyMappingCode) -> Option<u16> {
        Some(KeyMap::try_from(KeyMapping::Code(Some(key))).ok()?.mac as u16)
            .filter(|&sc| sc != INVALID)
    }

    fn scancode_name(&self, sc: u16) -> String {
        match self.decode_scancode(sc) {
            Some(key) => self.key_name(key),
            None => format!("SC{}", sc),
        }
    }

    fn key_name(&self, key: KeyMappingCode) -> String {
        use KeyMappingCode::*;
        match key {
            Backquote => "`".to_owned(),
            Backslash => "\\".to_owned(),
            BracketLeft => "[".to_owned(),
            BracketRight => "]".to_owned(),
            Comma => ",".to_owned(),
            Digit0 => "0".to_owned(),
            Digit1 => "1".to_owned(),
            Digit2 => "2".to_owned(),
            Digit3 => "3".to_owned(),
            Digit4 => "4".to_owned(),
            Digit5 => "5".to_owned(),
            Digit6 => "6".to_owned(),
            Digit7 => "7".to_owned(),
            Digit8 => "8".to_owned(),
            Digit9 => "9".to_owned(),
            Equal => "=".to_owned(),

            KeyA => "A".to_owned(),
            KeyB => "B".to_owned(),
            KeyC => "C".to_owned(),
            KeyD => "D".to_owned(),
            KeyE => "E".to_owned(),
            KeyF => "F".to_owned(),
            KeyG => "G".to_owned(),
            KeyH => "H".to_owned(),
            KeyI => "I".to_owned(),
            KeyJ => "J".to_owned(),
            KeyK => "K".to_owned(),
            KeyL => "L".to_owned(),
            KeyM => "M".to_owned(),
            KeyN => "N".to_owned(),
            KeyO => "O".to_owned(),
            KeyP => "P".to_owned(),
            KeyQ => "Q".to_owned(),
            KeyR => "R".to_owned(),
            KeyS => "S".to_owned(),
            KeyT => "T".to_owned(),
            KeyU => "U".to_owned(),
            KeyV => "V".to_owned(),
            KeyW => "W".to_owned(),
            KeyX => "X".to_owned(),
            KeyY => "Y".to_owned(),
            KeyZ => "Z".to_owned(),
            Minus => "-".to_owned(),
            Period => ".".to_owned(),
            Quote => "'".to_owned(),
            Semicolon => ";".to_owned(),
            Slash => "/".to_owned(),

            AltLeft => "Option".to_owned(),
            AltRight => "Right Option".to_owned(),
            ControlLeft => "Control".to_owned(),
            ControlRight => "Right Control".to_owned(),
            MetaLeft => "Command".to_owned(),
            MetaRight => "Right Command".to_owned(), // doesn't exist anyway
            ShiftLeft => "Shift".to_owned(),
            ShiftRight => "Right Shift".to_owned(),

            Backspace => "Delete".to_owned(),
            Enter => "Return".to_owned(),

            ArrowDown => "Down".to_owned(),
            ArrowLeft => "Left".to_owned(),
            ArrowRight => "Right".to_owned(),
            ArrowUp => "Up".to_owned(),
            Delete => "Forward Delete".to_owned(),

            Escape => "Esc".to_owned(),

            other => format!("{:?}", other),
        }
    }
}
