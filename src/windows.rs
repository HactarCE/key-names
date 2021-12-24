use super::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct WindowsKeymap;
impl OsKeymap for WindowsKeymap {
    fn logo_str(&self) -> &'static str {
        "Win"
    }

    fn decode_scancode(&self, sc: u32) -> Option<Key> {
        // Sources:
        //
        // - https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/scancode.doc
        // - http://www.quadibloc.com/comp/scan.htm
        // - https://www.win.tue.nl/~aeb/linux/kbd/scancodes-8.html
        // - https://sharktastica.co.uk/guides/jis_remapping
        Some(match sc {
            0x__01 => Key::Escape,
            0x__02 => Key::Digit1,
            0x__03 => Key::Digit2,
            0x__04 => Key::Digit3,
            0x__05 => Key::Digit4,
            0x__06 => Key::Digit5,
            0x__07 => Key::Digit6,
            0x__08 => Key::Digit7,
            0x__09 => Key::Digit8,
            0x__0A => Key::Digit9,
            0x__0B => Key::Digit0,
            0x__0C => Key::Minus,
            0x__0D => Key::Equal,
            0x__0E => Key::Backspace,
            0x__0F => Key::Tab,
            0x__10 => Key::KeyQ,
            0x__11 => Key::KeyW,
            0x__12 => Key::KeyE,
            0x__13 => Key::KeyR,
            0x__14 => Key::KeyT,
            0x__15 => Key::KeyY,
            0x__16 => Key::KeyU,
            0x__17 => Key::KeyI,
            0x__18 => Key::KeyO,
            0x__19 => Key::KeyP,
            0x__1A => Key::BracketLeft,
            0x__1B => Key::BracketRight,
            0x__1C => Key::Enter,
            0xE01C => Key::NumpadEnter,
            0x__1D => Key::ControlLeft,
            0xE01D => Key::ControlRight,
            0x__1E => Key::KeyA,
            0x__1F => Key::KeyS,
            0x__20 => Key::KeyD,
            0x__21 => Key::KeyF,
            0x__22 => Key::KeyG,
            0x__23 => Key::KeyH,
            0x__24 => Key::KeyJ,
            0x__25 => Key::KeyK,
            0x__26 => Key::KeyL,
            0x__27 => Key::Semicolon,
            0x__28 => Key::Quote,
            0x__29 => Key::Backquote,
            0x__2A => Key::ShiftLeft,
            0x__2B => Key::Backslash,
            0x__2C => Key::KeyZ,
            0x__2D => Key::KeyX,
            0x__2E => Key::KeyC,
            0x__2F => Key::KeyV,
            0x__30 => Key::KeyB,
            0x__31 => Key::KeyN,
            0x__32 => Key::KeyM,
            0x__33 => Key::Comma,
            0x__34 => Key::Period,
            0x__35 => Key::Slash,
            0xE035 => Key::NumpadDivide,
            0x__36 => Key::ShiftRight,
            0xE037 => Key::PrintScreen, // but sometimes 0x__54, if holding alt
            0x__37 => Key::NumpadMultiply,
            0x__38 => Key::AltLeft,
            0xE038 => Key::AltRight,
            0x__39 => Key::Space,
            0x__3A => Key::CapsLock,
            0x__3B => Key::F1,
            0x__3C => Key::F2,
            0x__3D => Key::F3,
            0x__3E => Key::F4,
            0x__3F => Key::F5,
            0x__40 => Key::F6,
            0x__41 => Key::F7,
            0x__42 => Key::F8,
            0x__43 => Key::F9,
            0x__44 => Key::F10,
            0xE045 => Key::NumLock,
            0x__46 => Key::ScrollLock,
            0x__47 => Key::Numpad7,
            0xE047 => Key::Home,
            0x__48 => Key::Numpad8,
            0xE048 => Key::ArrowUp,
            0x__49 => Key::Numpad9,
            0xE049 => Key::PageUp,
            0x__4A => Key::NumpadSubtract,
            0x__4B => Key::Numpad4,
            0xE04B => Key::ArrowLeft,
            0x__4C => Key::Numpad5,
            0x__4D => Key::Numpad6,
            0xE04D => Key::ArrowRight,
            0x__4E => Key::NumpadAdd,
            0x__4F => Key::Numpad1,
            0xE04F => Key::End,
            0x__50 => Key::Numpad2,
            0xE050 => Key::ArrowDown,
            0x__51 => Key::Numpad3,
            0xE051 => Key::PageDown,
            0x__52 => Key::Numpad0,
            0xE052 => Key::Insert,
            0x__53 => Key::NumpadDecimal,
            0xE053 => Key::Delete,
            0x__54 => Key::PrintScreen, // 0xE037 if not holding alt
            0x__56 => Key::IntlBackslash,
            0x__57 => Key::F11,
            0x__58 => Key::F12,
            0x__45 => Key::Pause, // I tested this -- it always works, regardless of modifiers
            0x__5B => Key::F13,
            0xE05B => Key::LogoLeft,
            0x__5C => Key::F14,
            0xE05C => Key::LogoRight,
            0x__5D => Key::F15,
            0xE05D => Key::ContextMenu,
            0x__63 => Key::F16,
            0x__64 => Key::F17,
            0x__65 => Key::F18,
            0x__66 => Key::F19,
            0x__67 => Key::F20,
            0x__68 => Key::F21,
            0x__69 => Key::F22,
            0x__6A => Key::F23,
            0x__6B => Key::F24,

            0x__70 => Key::KanaMode,
            0x__73 => Key::IntlRo,
            0x__79 => Key::Convert,
            0x__7B => Key::NonConvert,
            0x__7D => Key::IntlYen,

            // 0xE05E => Power,
            // 0xE05F => Sleep,
            // 0xE053 => Wake,
            // 0xE020 => Mute,
            // 0xE030 => VolumeUp,
            // 0xE02E => VolumeDown,
            // 0xE017 => Cut,
            // 0xE018 => Copy,
            // 0xE00A => Paste,
            0xE03B => Key::Help,
            // 0xE008 => Undo,
            // 0xE007 => Redo,
            // 0xE002 => Play,
            // 0xE024 => Stop,
            // 0xE010 => SkipBack,
            // 0xE019 => SkipFwd,
            // 0xE02C => Eject,
            // 0xE01E => Mail,
            // 0xE032 => Web,
            // 0xE03C => Music,
            // 0xE064 => Pictures,
            // 0xE06D => Video,
            _ => None?,
        })
    }

    #[rustfmt::skip]
    fn encode_scancode(&self, key: Key) -> Option<u32> {
        // Sources:
        //
        // - http://kbdlayout.info/KBDBR/scancodes
        // - https://download.microsoft.com/download/1/6/1/161ba512-40e2-4cc9-843a-923143f3456c/scancode.doc
        // - http://www.quadibloc.com/comp/scan.htm
        // - https://www.win.tue.nl/~aeb/linux/kbd/scancodes-8.html
        // - https://sharktastica.co.uk/guides/jis_remapping
        Some(match key {
            Key::Escape         => 0x__01,
            Key::Digit1         => 0x__02,
            Key::Digit2         => 0x__03,
            Key::Digit3         => 0x__04,
            Key::Digit4         => 0x__05,
            Key::Digit5         => 0x__06,
            Key::Digit6         => 0x__07,
            Key::Digit7         => 0x__08,
            Key::Digit8         => 0x__09,
            Key::Digit9         => 0x__0A,
            Key::Digit0         => 0x__0B,
            Key::Minus          => 0x__0C,
            Key::Equal          => 0x__0D,
            Key::Backspace      => 0x__0E,
            Key::Tab            => 0x__0F,
            Key::KeyQ           => 0x__10,
            Key::KeyW           => 0x__11,
            Key::KeyE           => 0x__12,
            Key::KeyR           => 0x__13,
            Key::KeyT           => 0x__14,
            Key::KeyY           => 0x__15,
            Key::KeyU           => 0x__16,
            Key::KeyI           => 0x__17,
            Key::KeyO           => 0x__18,
            Key::KeyP           => 0x__19,
            Key::BracketLeft    => 0x__1A,
            Key::BracketRight   => 0x__1B,
            Key::Enter          => 0x__1C,
            Key::NumpadEnter    => 0xE01C,
            Key::ControlLeft    => 0x__1D,
            Key::ControlRight   => 0xE01D,
            Key::KeyA           => 0x__1E,
            Key::KeyS           => 0x__1F,
            Key::KeyD           => 0x__20,
            Key::KeyF           => 0x__21,
            Key::KeyG           => 0x__22,
            Key::KeyH           => 0x__23,
            Key::KeyJ           => 0x__24,
            Key::KeyK           => 0x__25,
            Key::KeyL           => 0x__26,
            Key::Semicolon      => 0x__27,
            Key::Quote          => 0x__28,
            Key::Backquote      => 0x__29,
            Key::ShiftLeft      => 0x__2A,
            Key::Backslash      => 0x__2B,
            Key::KeyZ           => 0x__2C,
            Key::KeyX           => 0x__2D,
            Key::KeyC           => 0x__2E,
            Key::KeyV           => 0x__2F,
            Key::KeyB           => 0x__30,
            Key::KeyN           => 0x__31,
            Key::KeyM           => 0x__32,
            Key::Comma          => 0x__33,
            Key::Period         => 0x__34,
            Key::Slash          => 0x__35,
            Key::NumpadDivide   => 0xE035,
            Key::ShiftRight     => 0x__36,
            Key::PrintScreen    => 0xE037, // but sometimes 0x__54, if holding alt
            Key::NumpadMultiply => 0x__37,
            Key::AltLeft        => 0x__38,
            Key::AltRight       => 0xE038,
            Key::Space          => 0x__39,
            Key::CapsLock       => 0x__3A,
            Key::F1             => 0x__3B,
            Key::F2             => 0x__3C,
            Key::F3             => 0x__3D,
            Key::F4             => 0x__3E,
            Key::F5             => 0x__3F,
            Key::F6             => 0x__40,
            Key::F7             => 0x__41,
            Key::F8             => 0x__42,
            Key::F9             => 0x__43,
            Key::F10            => 0x__44,
            Key::Pause          => 0x__45, // I tested this -- it always works, regardless of modifiers
            Key::NumLock        => 0xE045,
            Key::ScrollLock     => 0x__46,
            Key::Numpad7        => 0x__47,
            Key::Home           => 0xE047,
            Key::Numpad8        => 0x__48,
            Key::ArrowUp        => 0xE048,
            Key::Numpad9        => 0x__49,
            Key::PageUp         => 0xE049,
            Key::NumpadSubtract => 0x__4A,
            Key::Numpad4        => 0x__4B,
            Key::ArrowLeft      => 0xE04B,
            Key::Numpad5        => 0x__4C,
            Key::Numpad6        => 0x__4D,
            Key::ArrowRight     => 0xE04D,
            Key::NumpadAdd      => 0x__4E,
            Key::Numpad1        => 0x__4F,
            Key::End            => 0xE04F,
            Key::Numpad2        => 0x__50,
            Key::ArrowDown      => 0xE050,
            Key::Numpad3        => 0x__51,
            Key::PageDown       => 0xE051,
            Key::Numpad0        => 0x__52,
            Key::Insert         => 0xE052,
            Key::NumpadDecimal  => 0x__53,
            Key::Delete         => 0xE053,
            // PhysicalKey::PrintScreen => 0x__54, // 0xE037 if not holding alt
            Key::IntlBackslash  => 0x__56,
            Key::F11            => 0x__57,
            Key::F12            => 0x__58,
            Key::F13            => 0x__5B,
            Key::LogoLeft       => 0xE05B,
            Key::F14            => 0x__5C,
            Key::LogoRight      => 0xE05C,
            Key::F15            => 0x__5D,
            Key::ContextMenu    => 0xE05D,
            Key::F16            => 0x__63,
            Key::F17            => 0x__64,
            Key::F18            => 0x__65,
            Key::F19            => 0x__66,
            Key::F20            => 0x__67,
            Key::F21            => 0x__68,
            Key::F22            => 0x__69,
            Key::F23            => 0x__6A,
            Key::F24            => 0x__6B,

            Key::KanaMode       => 0x__70,
            Key::IntlRo         => 0x__73,
            Key::Convert        => 0x__79,
            Key::NonConvert     => 0x__7B,
            Key::IntlYen        => 0x__7D,

            // Power            => 0xE05E,
            // Sleep            => 0xE05F,
            // Wake             => 0xE053,
            // Mute             => 0xE020,
            // VolumeUp         => 0xE030,
            // VolumeDown       => 0xE02E,
            // Cut              => 0xE017,
            // Copy             => 0xE018,
            // Paste            => 0xE00A,
            Key::Help           => 0xE03B,
            // Undo             => 0xE008,
            // Redo             => 0xE007,
            // Play             => 0xE002,
            // Stop             => 0xE024,
            // SkipBack         => 0xE010,
            // SkipFwd          => 0xE019,
            // Eject            => 0xE02C,
            // Mail             => 0xE01E,
            // Web              => 0xE032,
            // Music            => 0xE03C,
            // Pictures         => 0xE064,
            // Video            => 0xE06D,
        })
    }

    fn scancode_name(&self, sc: u32) -> String {
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
