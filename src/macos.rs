use super::*;

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

    fn decode_scancode(&self, sc: u32) -> Option<Key> {
        // Sources:
        //
        // - http://www.meandmark.com/keycodes.html /
        //   https://gist.github.com/eegrok/949034
        // - https://eastmanreference.com/complete-list-of-applescript-key-codes
        //   [^1]
        // - https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.6.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h
        //   (has some useful international keys)
        //
        // [^1]: The diagram at the top of this page erroneously labels the left
        //       shift key as having code 57 (0x39); a later diagram corrects
        //       this to 56 (0x38).
        Some(match sc {
            0x00 => Key::KeyA,
            0x01 => Key::KeyS,
            0x02 => Key::KeyD,
            0x03 => Key::KeyF,
            0x04 => Key::KeyH,
            0x05 => Key::KeyG,
            0x06 => Key::KeyZ,
            0x07 => Key::KeyX,
            0x08 => Key::KeyC,
            0x09 => Key::KeyV,
            0x0A => Key::IntlBackslash,
            0x0B => Key::KeyB,
            0x0C => Key::KeyQ,
            0x0D => Key::KeyW,
            0x0E => Key::KeyE,
            0x0F => Key::KeyR,
            0x10 => Key::KeyY,
            0x11 => Key::KeyT,
            0x12 => Key::Digit1,
            0x13 => Key::Digit2,
            0x14 => Key::Digit3,
            0x15 => Key::Digit4,
            0x16 => Key::Digit6,
            0x17 => Key::Digit5,
            0x18 => Key::Equal,
            0x19 => Key::Digit9,
            0x1A => Key::Digit7,
            0x1B => Key::Minus,
            0x1C => Key::Digit8,
            0x1D => Key::Digit0,
            0x1E => Key::BracketRight,
            0x1F => Key::KeyO,
            0x20 => Key::KeyU,
            0x21 => Key::BracketLeft,
            0x22 => Key::KeyI,
            0x23 => Key::KeyP,
            0x24 => Key::Enter,
            0x25 => Key::KeyL,
            0x26 => Key::KeyJ,
            0x27 => Key::Quote,
            0x28 => Key::KeyK,
            0x29 => Key::Semicolon,
            0x2A => Key::Backslash,
            0x2B => Key::Comma,
            0x2C => Key::Slash,
            0x2D => Key::KeyN,
            0x2E => Key::KeyM,
            0x2F => Key::Period,
            0x30 => Key::Tab,
            0x31 => Key::Space,
            0x32 => Key::Backquote,
            0x33 => Key::Backspace,
            0x34 => None?, // (unused)
            0x35 => Key::Escape,
            0x36 => None?, // (unused)
            0x37 => Key::LogoLeft,
            0x38 => Key::ShiftLeft,
            0x39 => Key::CapsLock,
            0x3A => Key::AltLeft,
            0x3B => Key::ControlLeft,
            0x3C => Key::ShiftRight,
            0x3D => Key::AltRight,
            0x3E => Key::ControlRight,
            0x40 => Key::F17,
            0x41 => Key::NumpadDecimal,
            0x42 => None?, // (unused)
            0x43 => Key::NumpadMultiply,
            0x44 => None?, // (unused)
            0x45 => Key::NumpadAdd,
            0x46 => None?, // (unused)
            0x47 => None?, // KeypadClear
            0x48 => None?, // VolumeUp
            0x49 => None?, // VolumeDown
            0x4A => None?, // Mute
            0x4B => Key::NumpadDivide,
            0x4C => Key::NumpadEnter,
            0x4D => None?, // (unused)
            0x4E => Key::NumpadSubtract,
            0x4F => Key::F18,
            0x50 => Key::F19,
            0x51 => None?, // KeypadEquals
            0x52 => Key::Numpad0,
            0x53 => Key::Numpad1,
            0x54 => Key::Numpad2,
            0x55 => Key::Numpad3,
            0x56 => Key::Numpad4,
            0x57 => Key::Numpad5,
            0x58 => Key::Numpad6,
            0x59 => Key::Numpad7,
            0x5A => Key::F20,
            0x5B => Key::Numpad8,
            0x5C => Key::Numpad9,
            0x5D => Key::IntlYen,
            0x5E => None?, // JIS_Underscore
            0x5F => None?, // JIS_KeypadComma
            0x60 => Key::F5,
            0x61 => Key::F6,
            0x62 => Key::F7,
            0x63 => Key::F3,
            0x64 => Key::F8,
            0x65 => Key::F9,
            0x66 => None?, // JIS_Eisu
            0x67 => Key::F11,
            0x68 => Key::KanaMode,
            0x69 => Key::F13,
            0x6A => Key::F16,
            0x6B => Key::F14,
            0x6C => None?, // (unused)
            0x6D => Key::F10,
            0x6E => None?, // (unused)
            0x6F => Key::F12,
            0x71 => Key::F15,
            0x72 => Key::Help,
            0x73 => Key::Home,
            0x74 => Key::PageUp,
            0x75 => Key::Delete,
            0x76 => Key::F4,
            0x77 => Key::End,
            0x78 => Key::F2,
            0x79 => Key::PageDown,
            0x7A => Key::F1,
            0x7B => Key::ArrowLeft,
            0x7C => Key::ArrowRight,
            0x7D => Key::ArrowDown,
            0x7E => Key::ArrowUp,

            // As far as I can tell, these keys simply do not exist on macOS.
            // _ => Key::IntlRo,
            // _ => Key::ContextMenu,
            // _ => Key::LogoRight,
            // _ => Key::Convert,
            // _ => Key::NonConvert,
            // _ => Key::Insert,
            // _ => Key::NumLock,
            // _ => Key::F21,
            // _ => Key::F22,
            // _ => Key::F23,
            // _ => Key::F24,
            // _ => Key::PrintScreen,
            // _ => Key::ScrollLock,
            // _ => Key::Pause,
            _ => None?,
        })
    }

    #[rustfmt::skip]
    fn encode_scancode(&self, key: Key) -> Option<u32> {
        // Sources:
        //
        // - http://www.meandmark.com/keycodes.html /
        //   https://gist.github.com/eegrok/949034
        // - https://eastmanreference.com/complete-list-of-applescript-key-codes
        //   [^1]
        // - https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.6.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h
        //   (has some useful international keys)
        //
        // [^1]: The diagram at the top of this page erroneously labels the left
        //       shift key as having code 57 (0x39); a later diagram corrects
        //       this to 56 (0x38).
        Some(match key {
            Key::KeyA           => 0x00,
            Key::KeyS           => 0x01,
            Key::KeyD           => 0x02,
            Key::KeyF           => 0x03,
            Key::KeyH           => 0x04,
            Key::KeyG           => 0x05,
            Key::KeyZ           => 0x06,
            Key::KeyX           => 0x07,
            Key::KeyC           => 0x08,
            Key::KeyV           => 0x09,
            Key::IntlBackslash  => 0x0A,
            Key::KeyB           => 0x0B,
            Key::KeyQ           => 0x0C,
            Key::KeyW           => 0x0D,
            Key::KeyE           => 0x0E,
            Key::KeyR           => 0x0F,
            Key::KeyY           => 0x10,
            Key::KeyT           => 0x11,
            Key::Digit1         => 0x12,
            Key::Digit2         => 0x13,
            Key::Digit3         => 0x14,
            Key::Digit4         => 0x15,
            Key::Digit6         => 0x16,
            Key::Digit5         => 0x17,
            Key::Equal          => 0x18,
            Key::Digit9         => 0x19,
            Key::Digit7         => 0x1A,
            Key::Minus          => 0x1B,
            Key::Digit8         => 0x1C,
            Key::Digit0         => 0x1D,
            Key::BracketRight   => 0x1E,
            Key::KeyO           => 0x1F,
            Key::KeyU           => 0x20,
            Key::BracketLeft    => 0x21,
            Key::KeyI           => 0x22,
            Key::KeyP           => 0x23,
            Key::Enter          => 0x24,
            Key::KeyL           => 0x25,
            Key::KeyJ           => 0x26,
            Key::Quote          => 0x27,
            Key::KeyK           => 0x28,
            Key::Semicolon      => 0x29,
            Key::Backslash      => 0x2A,
            Key::Comma          => 0x2B,
            Key::Slash          => 0x2C,
            Key::KeyN           => 0x2D,
            Key::KeyM           => 0x2E,
            Key::Period         => 0x2F,
            Key::Tab            => 0x30,
            Key::Space          => 0x31,
            Key::Backquote      => 0x32,
            Key::Backspace      => 0x33,
            // (unused)         => 0x34,
            Key::Escape         => 0x35,
            // (unused)         => 0x36,
            Key::LogoLeft       => 0x37,
            Key::ShiftLeft      => 0x38,
            Key::CapsLock       => 0x39,
            Key::AltLeft        => 0x3A,
            Key::ControlLeft    => 0x3B,
            Key::ShiftRight     => 0x3C,
            Key::AltRight       => 0x3D,
            Key::ControlRight   => 0x3E,
            Key::F17            => 0x40,
            Key::NumpadDecimal  => 0x41,
            // (unused)         => 0x42,
            Key::NumpadMultiply => 0x43,
            // (unused)         => 0x44,
            Key::NumpadAdd      => 0x45,
            // (unused)         => 0x46,
            // KeypadClear      => 0x47,
            // VolumeUp         => 0x48,
            // VolumeDown       => 0x49,
            // Mute             => 0x4A,
            Key::NumpadDivide   => 0x4B,
            Key::NumpadEnter    => 0x4C,
            // (unused)         => 0x4D,
            Key::NumpadSubtract => 0x4E,
            Key::F18            => 0x4F,
            Key::F19            => 0x50,
            // KeypadEquals     => 0x51,
            Key::Numpad0        => 0x52,
            Key::Numpad1        => 0x53,
            Key::Numpad2        => 0x54,
            Key::Numpad3        => 0x55,
            Key::Numpad4        => 0x56,
            Key::Numpad5        => 0x57,
            Key::Numpad6        => 0x58,
            Key::Numpad7        => 0x59,
            Key::F20            => 0x5A,
            Key::Numpad8        => 0x5B,
            Key::Numpad9        => 0x5C,
            Key::IntlYen        => 0x5D,
            // JIS_Underscore   => 0x5E,
            // JIS_KeypadComma  => 0x5F,
            Key::F5             => 0x60,
            Key::F6             => 0x61,
            Key::F7             => 0x62,
            Key::F3             => 0x63,
            Key::F8             => 0x64,
            Key::F9             => 0x65,
            // JIS_Eisu         => 0x66,
            Key::F11            => 0x67,
            Key::KanaMode       => 0x68,
            Key::F13            => 0x69,
            Key::F16            => 0x6A,
            Key::F14            => 0x6B,
            // (unused)         => 0x6C,
            Key::F10            => 0x6D,
            // (unused)         => 0x6E,
            Key::F12            => 0x6F,
            Key::F15            => 0x71,
            Key::Help           => 0x72,
            Key::Home           => 0x73,
            Key::PageUp         => 0x74,
            Key::Delete         => 0x75,
            Key::F4             => 0x76,
            Key::End            => 0x77,
            Key::F2             => 0x78,
            Key::PageDown       => 0x79,
            Key::F1             => 0x7A,
            Key::ArrowLeft      => 0x7B,
            Key::ArrowRight     => 0x7C,
            Key::ArrowDown      => 0x7D,
            Key::ArrowUp        => 0x7E,

            // As far as I can tell, these keys simply do not exist on macOS.
            Key::IntlRo         => None?,
            Key::ContextMenu    => None?,
            Key::LogoRight      => None?,
            Key::Convert        => None?,
            Key::NonConvert     => None?,
            Key::Insert         => None?,
            Key::NumLock        => None?,
            Key::F21            => None?,
            Key::F22            => None?,
            Key::F23            => None?,
            Key::F24            => None?,
            Key::PrintScreen    => None?,
            Key::ScrollLock     => None?,
            Key::Pause          => None?,
        })
    }

    fn scancode_name(&self, sc: u32) -> String {
        match self.decode_scancode(sc) {
            Some(key) => self.key_name(key),
            None => format!("SC{}", sc),
        }
    }

    fn key_name(&self, key: Key) -> String {
        match key {
            Key::Backquote => "`".to_owned(),
            Key::Backslash => "\\".to_owned(),
            Key::BracketLeft => "[".to_owned(),
            Key::BracketRight => "]".to_owned(),
            Key::Comma => ",".to_owned(),
            Key::Digit0 => "0".to_owned(),
            Key::Digit1 => "1".to_owned(),
            Key::Digit2 => "2".to_owned(),
            Key::Digit3 => "3".to_owned(),
            Key::Digit4 => "4".to_owned(),
            Key::Digit5 => "5".to_owned(),
            Key::Digit6 => "6".to_owned(),
            Key::Digit7 => "7".to_owned(),
            Key::Digit8 => "8".to_owned(),
            Key::Digit9 => "9".to_owned(),
            Key::Equal => "=".to_owned(),

            Key::KeyA => "A".to_owned(),
            Key::KeyB => "B".to_owned(),
            Key::KeyC => "C".to_owned(),
            Key::KeyD => "D".to_owned(),
            Key::KeyE => "E".to_owned(),
            Key::KeyF => "F".to_owned(),
            Key::KeyG => "G".to_owned(),
            Key::KeyH => "H".to_owned(),
            Key::KeyI => "I".to_owned(),
            Key::KeyJ => "J".to_owned(),
            Key::KeyK => "K".to_owned(),
            Key::KeyL => "L".to_owned(),
            Key::KeyM => "M".to_owned(),
            Key::KeyN => "N".to_owned(),
            Key::KeyO => "O".to_owned(),
            Key::KeyP => "P".to_owned(),
            Key::KeyQ => "Q".to_owned(),
            Key::KeyR => "R".to_owned(),
            Key::KeyS => "S".to_owned(),
            Key::KeyT => "T".to_owned(),
            Key::KeyU => "U".to_owned(),
            Key::KeyV => "V".to_owned(),
            Key::KeyW => "W".to_owned(),
            Key::KeyX => "X".to_owned(),
            Key::KeyY => "Y".to_owned(),
            Key::KeyZ => "Z".to_owned(),
            Key::Minus => "-".to_owned(),
            Key::Period => ".".to_owned(),
            Key::Quote => "'".to_owned(),
            Key::Semicolon => ";".to_owned(),
            Key::Slash => "/".to_owned(),

            Key::AltLeft => "Option".to_owned(),
            Key::AltRight => "Right Option".to_owned(),
            Key::ControlLeft => "Control".to_owned(),
            Key::ControlRight => "Right Control".to_owned(),
            Key::LogoLeft => "Command".to_owned(),
            Key::LogoRight => "Right Command".to_owned(), // doesn't exist anyway
            Key::ShiftLeft => "Shift".to_owned(),
            Key::ShiftRight => "Right Shift".to_owned(),

            Key::Backspace => "Delete".to_owned(),
            Key::Enter => "Return".to_owned(),

            Key::ArrowDown => "Down".to_owned(),
            Key::ArrowLeft => "Left".to_owned(),
            Key::ArrowRight => "Right".to_owned(),
            Key::ArrowUp => "Up".to_owned(),
            Key::Delete => "Forward Delete".to_owned(),

            Key::Escape => "Esc".to_owned(),

            other => format!("{:?}", other),
        }
    }
}
