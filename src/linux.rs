use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use wayland_client::protocol::wl_keyboard::{KeymapFormat, WlKeyboard};
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::{DispatchData, Main};
use xkb::x11::{MIN_MAJOR_XKB_VERSION, MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb::{self, KEYMAP_COMPILE_NO_FLAGS, KEYMAP_FORMAT_TEXT_V1};

use super::*;

#[derive(Clone)]
pub struct LinuxKeymap {
    keymap: xkb::Keymap,
}
impl fmt::Debug for LinuxKeymap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LinuxKeymap")
    }
}
impl LinuxKeymap {
    /// Constructs a keymap using either X11 or Wayland automatically.
    pub fn new() -> Result<Self, KeymapError> {
        match std::env::var("XDG_SESSION_TYPE") {
            Ok(session_type) => match session_type.as_str() {
                "wayland" => return Self::new_wayland(),
                "x11" => return Self::new_x11(),
                _ => (),
            },
            Err(_) => (),
        }
        // Just try both and return whichever succeeds.
        Self::new_wayland().or_else(|_| Self::new_x11())
    }

    /// Constructs a keymap in an X11 environment.
    pub fn new_x11() -> Result<Self, KeymapError> {
        // This code is modified from Frinksy's `keyboard-keynames` crate:
        // https://gitlab.com/Frinksy/keyboard-keynames/-/blob/master/src/platform/unix/key_layout.rs

        let (conn, _) = xcb::Connection::connect(None).map_err(|_| KeymapError::X11Connect)?;
        let mut major_xkb_version_out = 0;
        let mut minor_xkb_version_out = 0;
        let mut base_event_out = 0;
        let mut base_error_out = 0;

        let _ = xkb::x11::setup_xkb_extension(
            &conn,
            MIN_MAJOR_XKB_VERSION,
            MIN_MINOR_XKB_VERSION,
            xkb::x11::SetupXkbExtensionFlags::NoFlags,
            &mut major_xkb_version_out,
            &mut minor_xkb_version_out,
            &mut base_event_out,
            &mut base_error_out,
        );

        let device_id = xkb::x11::get_core_keyboard_device_id(&conn);

        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

        let keymap = xkb::x11::keymap_new_from_device(&ctx, &conn, device_id, 0);

        Ok(Self { keymap })
    }
    /// Constructs a keymap in a Wayland environment.
    pub fn new_wayland() -> Result<Self, KeymapError> {
        // This code is modified from Frinksy's `keyboard-keynames` crate:
        // https://gitlab.com/Frinksy/keyboard-keynames/-/blob/master/src/platform/unix/key_layout.rs

        let display =
            wayland_client::Display::connect_to_env().map_err(|_| KeymapError::WaylandConnect)?;

        // Set up the event queue.
        let mut event_queue = display.create_event_queue();
        let token = event_queue.token();

        let proxy = &*display;
        let attached = proxy.attach(token);
        let registry = attached.get_registry();

        // Listen for available interfaces.
        let available_interfaces = Rc::new(RefCell::new(Vec::<(u32, String, u32)>::new()));
        let available_interfaces_copy = Rc::clone(&available_interfaces);

        registry.quick_assign(move |_reg, event, _data| {
            if let wayland_client::protocol::wl_registry::Event::Global {
                name,
                interface,
                version,
            } = event
            {
                (*available_interfaces_copy)
                    .borrow_mut()
                    .push((name, interface, version));
            }
        });

        event_queue.sync_roundtrip(&mut (), |_, _, _| {})?;

        // Bind to wl_seat if available. First, find wl_seat tuple.
        let (seat_name, _seat_interface, seat_version) = (*available_interfaces)
            .borrow()
            .iter()
            .find(|(_name, interface, _version)| interface == "wl_seat")
            .ok_or(KeymapError::MissingWlSeat)?
            .clone();

        attached.sync();

        let wl_seat = registry.bind::<WlSeat>(seat_version, seat_name);

        let capabilities = Rc::new(RefCell::new(
            wayland_client::protocol::wl_seat::Capability::empty(),
        ));
        let capabilities_copy = Rc::clone(&capabilities);
        wl_seat.quick_assign(move |_seat, event, _data| {
            if let wayland_client::protocol::wl_seat::Event::Capabilities { capabilities } = event {
                (*capabilities_copy).borrow_mut().set(capabilities, true);
            }
        });
        event_queue.sync_roundtrip(&mut (), |_, _, _| {})?;

        // Check capabilities of wl_seat.
        if !(*capabilities)
            .borrow()
            .contains(wayland_client::protocol::wl_seat::Capability::Keyboard)
        {
            return Err(KeymapError::MissingKeyboardCapability);
        }

        let wl_keyboard = wl_seat.get_keyboard();

        // Get keymap from compositor.
        let file_descriptor = Rc::new(RefCell::new(-1));
        let size = Rc::new(RefCell::new(0));
        let file_descriptor_copy = Rc::clone(&file_descriptor);
        let size_copy = Rc::clone(&size);
        wl_keyboard.quick_assign(
            move |_object: Main<WlKeyboard>,
                  event: wayland_client::protocol::wl_keyboard::Event,
                  _data: DispatchData<'_>| {
                if let wayland_client::protocol::wl_keyboard::Event::Keymap { format, fd, size } =
                    event
                {
                    match format {
                        KeymapFormat::XkbV1 => {
                            *file_descriptor_copy.borrow_mut() = fd;
                            *size_copy.borrow_mut() = size;
                        }
                        KeymapFormat::NoKeymap => {
                            panic!("NoKeymap format");
                        }
                        _ => {
                            panic!("Keymap Format not supported");
                        }
                    };
                }
            },
        );
        event_queue.sync_roundtrip(&mut (), |_, _, _| {})?;

        // Construct keymap from file descriptor.
        let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb::Keymap::new_from_fd(
            &ctx,
            *(*file_descriptor).borrow(),
            (*(*size).borrow()).try_into().unwrap(),
            KEYMAP_FORMAT_TEXT_V1,
            KEYMAP_COMPILE_NO_FLAGS,
        )
        .ok_or(KeymapError::FailedToCreateKeymap)?;

        Ok(Self { keymap })
    }
}
impl OsKeymap for LinuxKeymap {
    fn logo_str(&self) -> &'static str {
        "Super"
    }

    #[rustfmt::skip]
    fn decode_scancode(&self, sc: u32) -> Option<Key> {
        // Sources:
        //
        // - https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html (vague about F11-F24)
        // - https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
        // - https://askubuntu.com/questions/444810/editing-keyboard-layout (`IntlBackslash` = `102nd`)
        Some(match sc {
            0x01 => Key::Escape,         // 1
            0x02 => Key::Digit1,         // 2
            0x03 => Key::Digit2,         // 3
            0x04 => Key::Digit3,         // 4
            0x05 => Key::Digit4,         // 5
            0x06 => Key::Digit5,         // 6
            0x07 => Key::Digit6,         // 7
            0x08 => Key::Digit7,         // 8
            0x09 => Key::Digit8,         // 9
            0x0A => Key::Digit9,         // 10
            0x0B => Key::Digit0,         // 11
            0x0C => Key::Minus,          // 12
            0x0D => Key::Equal,          // 13
            0x0E => Key::Backspace,      // 14
            0x0F => Key::Tab,            // 15
            0x10 => Key::KeyQ,           // 16
            0x11 => Key::KeyW,           // 17
            0x12 => Key::KeyE,           // 18
            0x13 => Key::KeyR,           // 19
            0x14 => Key::KeyT,           // 20
            0x15 => Key::KeyY,           // 21
            0x16 => Key::KeyU,           // 22
            0x17 => Key::KeyI,           // 23
            0x18 => Key::KeyO,           // 24
            0x19 => Key::KeyP,           // 25
            0x1A => Key::BracketLeft,    // 26
            0x1B => Key::BracketRight,   // 27
            0x1C => Key::Enter,          // 28
            0x1D => Key::ControlLeft,    // 29
            0x1E => Key::KeyA,           // 30
            0x1F => Key::KeyS,           // 31
            0x20 => Key::KeyD,           // 32
            0x21 => Key::KeyF,           // 33
            0x22 => Key::KeyG,           // 34
            0x23 => Key::KeyH,           // 35
            0x24 => Key::KeyJ,           // 36
            0x25 => Key::KeyK,           // 37
            0x26 => Key::KeyL,           // 38
            0x27 => Key::Semicolon,      // 39
            0x28 => Key::Quote,          // 40
            0x29 => Key::Backquote,      // 41
            0x2A => Key::ShiftLeft,      // 42
            0x2B => Key::Backslash,      // 43
            0x2C => Key::KeyZ,           // 44
            0x2D => Key::KeyX,           // 45
            0x2E => Key::KeyC,           // 46
            0x2F => Key::KeyV,           // 47
            0x30 => Key::KeyB,           // 48
            0x31 => Key::KeyN,           // 49
            0x32 => Key::KeyM,           // 50
            0x33 => Key::Comma,          // 51
            0x34 => Key::Period,         // 52
            0x35 => Key::Slash,          // 53
            0x36 => Key::ShiftRight,     // 54
            0x37 => Key::NumpadMultiply, // 55
            0x38 => Key::AltLeft,        // 56
            0x39 => Key::Space,          // 57
            0x3A => Key::CapsLock,       // 58
            0x3B => Key::F1,             // 59
            0x3C => Key::F2,             // 60
            0x3D => Key::F3,             // 61
            0x3E => Key::F4,             // 62
            0x3F => Key::F5,             // 63
            0x40 => Key::F6,             // 64
            0x41 => Key::F7,             // 65
            0x42 => Key::F8,             // 66
            0x43 => Key::F9,             // 67
            0x44 => Key::F10,            // 68
            0x45 => Key::NumLock,        // 69
            0x46 => Key::ScrollLock,     // 70
            0x47 => Key::Numpad7,        // 71
            0x48 => Key::Numpad8,        // 72
            0x49 => Key::Numpad9,        // 73
            0x4A => Key::NumpadSubtract, // 74
            0x4B => Key::Numpad4,        // 75
            0x4C => Key::Numpad5,        // 76
            0x4D => Key::Numpad6,        // 77
            0x4E => Key::NumpadAdd,      // 78
            0x4F => Key::Numpad1,        // 79
            0x50 => Key::Numpad2,        // 80
            0x51 => Key::Numpad3,        // 81
            0x52 => Key::Numpad0,        // 82
            0x53 => Key::NumpadDecimal,  // 83
            0x54 => None?, // (unused)   // 84
            0x55 => None?, // Zenkakuhank// 85
            0x56 => Key::IntlBackslash,  // 86
            0x57 => Key::F11,            // 87
            0x58 => Key::F12,            // 88
            0x59 => Key::IntlRo,         // 89
            0x5A => None?, // Katakana   // 90
            0x5B => None?, // Hiragana   // 91
            0x5C => Key::Convert,        // 92
            0x5D => Key::KanaMode,       // 93
            0x5E => Key::NonConvert,     // 94
            0x5F => None?, // Kpjpcomma  // 95
            0x60 => Key::NumpadEnter,    // 96
            0x61 => Key::ControlRight,   // 97
            0x62 => Key::NumpadDivide,   // 98
            0x63 => Key::PrintScreen,    // 99
            0x64 => Key::AltRight,       // 100
            0x65 => None?, // Linefeed   // 101
            0x66 => Key::Home,           // 102
            0x67 => Key::ArrowUp,        // 103
            0x68 => Key::PageUp,         // 104
            0x69 => Key::ArrowLeft,      // 105
            0x6A => Key::ArrowRight,     // 106
            0x6B => Key::End,            // 107
            0x6C => Key::ArrowDown,      // 108
            0x6D => Key::PageDown,       // 109
            0x6E => Key::Insert,         // 110
            0x6F => Key::Delete,         // 111
            0x70 => None?, // Macro      // 112
            0x71 => None?, // Mute       // 113
            0x72 => None?, // Volumedown // 114
            0x73 => None?, // Volumeup   // 115
            0x74 => None?, // Power      // 116
            0x75 => None?, // Kpequal    // 117
            0x76 => None?, // Kpplusminus// 118
            0x77 => Key::Pause,          // 119
            0x78 => None?, // Scale      // 120

            0x79 => None?, // Kpcomma    // 121
            0x7A => None?, // Hangeul    // 122
            0x7B => None?, // Hanja      // 123
            0x7C => Key::IntlYen,        // 124
            0x7D => Key::LogoLeft,       // 125
            0x7E => Key::LogoRight,      // 126
            0x7F => None?, // Compose    // 127

            0x80 => None?, // Stop       // 128
            0x81 => None?, // Again      // 129
            0x82 => None?, // Props      // 130
            0x83 => None?, // Undo       // 131
            0x84 => None?, // Front      // 132
            0x85 => None?, // Copy       // 133
            0x86 => None?, // Open       // 134
            0x87 => None?, // Paste      // 135
            0x88 => None?, // Find       // 136
            0x89 => None?, // Cut        // 137
            0x8A => Key::Help,           // 138
            0x8B => Key::ContextMenu,    // 139

            0xB7 => Key::F13,            // 183
            0xB8 => Key::F14,            // 184
            0xB9 => Key::F15,            // 185
            0xBA => Key::F16,            // 186
            0xBB => Key::F17,            // 187
            0xBC => Key::F18,            // 188
            0xBD => Key::F19,            // 189
            0xBE => Key::F20,            // 190
            0xBF => Key::F21,            // 191
            0xC0 => Key::F22,            // 192
            0xC1 => Key::F23,            // 193
            0xC2 => Key::F24,            // 194

            _ => None?,
        })
    }

    #[rustfmt::skip]
    fn encode_scancode(&self, key: Key) -> Option<u32> {
        // Sources:
        //
        // - https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html (vague about F11-F24)
        // - https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
        // - https://askubuntu.com/questions/444810/editing-keyboard-layout (`IntlBackslash` = `102nd`)
        Some(match key {
            Key::Escape         => 0x01, // 1
            Key::Digit1         => 0x02, // 2
            Key::Digit2         => 0x03, // 3
            Key::Digit3         => 0x04, // 4
            Key::Digit4         => 0x05, // 5
            Key::Digit5         => 0x06, // 6
            Key::Digit6         => 0x07, // 7
            Key::Digit7         => 0x08, // 8
            Key::Digit8         => 0x09, // 9
            Key::Digit9         => 0x0A, // 10
            Key::Digit0         => 0x0B, // 11
            Key::Minus          => 0x0C, // 12
            Key::Equal          => 0x0D, // 13
            Key::Backspace      => 0x0E, // 14
            Key::Tab            => 0x0F, // 15
            Key::KeyQ           => 0x10, // 16
            Key::KeyW           => 0x11, // 17
            Key::KeyE           => 0x12, // 18
            Key::KeyR           => 0x13, // 19
            Key::KeyT           => 0x14, // 20
            Key::KeyY           => 0x15, // 21
            Key::KeyU           => 0x16, // 22
            Key::KeyI           => 0x17, // 23
            Key::KeyO           => 0x18, // 24
            Key::KeyP           => 0x19, // 25
            Key::BracketLeft    => 0x1A, // 26
            Key::BracketRight   => 0x1B, // 27
            Key::Enter          => 0x1C, // 28
            Key::ControlLeft    => 0x1D, // 29
            Key::KeyA           => 0x1E, // 30
            Key::KeyS           => 0x1F, // 31
            Key::KeyD           => 0x20, // 32
            Key::KeyF           => 0x21, // 33
            Key::KeyG           => 0x22, // 34
            Key::KeyH           => 0x23, // 35
            Key::KeyJ           => 0x24, // 36
            Key::KeyK           => 0x25, // 37
            Key::KeyL           => 0x26, // 38
            Key::Semicolon      => 0x27, // 39
            Key::Quote          => 0x28, // 40
            Key::Backquote      => 0x29, // 41
            Key::ShiftLeft      => 0x2A, // 42
            Key::Backslash      => 0x2B, // 43
            Key::KeyZ           => 0x2C, // 44
            Key::KeyX           => 0x2D, // 45
            Key::KeyC           => 0x2E, // 46
            Key::KeyV           => 0x2F, // 47
            Key::KeyB           => 0x30, // 48
            Key::KeyN           => 0x31, // 49
            Key::KeyM           => 0x32, // 50
            Key::Comma          => 0x33, // 51
            Key::Period         => 0x34, // 52
            Key::Slash          => 0x35, // 53
            Key::ShiftRight     => 0x36, // 54
            Key::NumpadMultiply => 0x37, // 55
            Key::AltLeft        => 0x38, // 56
            Key::Space          => 0x39, // 57
            Key::CapsLock       => 0x3A, // 58
            Key::F1             => 0x3B, // 59
            Key::F2             => 0x3C, // 60
            Key::F3             => 0x3D, // 61
            Key::F4             => 0x3E, // 62
            Key::F5             => 0x3F, // 63
            Key::F6             => 0x40, // 64
            Key::F7             => 0x41, // 65
            Key::F8             => 0x42, // 66
            Key::F9             => 0x43, // 67
            Key::F10            => 0x44, // 68
            Key::NumLock        => 0x45, // 69
            Key::ScrollLock     => 0x46, // 70
            Key::Numpad7        => 0x47, // 71
            Key::Numpad8        => 0x48, // 72
            Key::Numpad9        => 0x49, // 73
            Key::NumpadSubtract => 0x4A, // 74
            Key::Numpad4        => 0x4B, // 75
            Key::Numpad5        => 0x4C, // 76
            Key::Numpad6        => 0x4D, // 77
            Key::NumpadAdd      => 0x4E, // 78
            Key::Numpad1        => 0x4F, // 79
            Key::Numpad2        => 0x50, // 80
            Key::Numpad3        => 0x51, // 81
            Key::Numpad0        => 0x52, // 82
            Key::NumpadDecimal  => 0x53, // 83
            // (unused)         => 0x54, // 84
            // Zenkakuhankaku   => 0x55, // 85
            Key::IntlBackslash  => 0x56, // 86
            Key::F11            => 0x57, // 87
            Key::F12            => 0x58, // 88
            Key::IntlRo         => 0x59, // 89
            // Katakana         => 0x5A, // 90
            // Hiragana         => 0x5B, // 91
            Key::Convert        => 0x5C, // 92
            Key::KanaMode       => 0x5D, // 93
            Key::NonConvert     => 0x5E, // 94
            // Kpjpcomma        => 0x5F, // 95
            Key::NumpadEnter    => 0x60, // 96
            Key::ControlRight   => 0x61, // 97
            Key::NumpadDivide   => 0x62, // 98
            Key::PrintScreen    => 0x63, // 99
            Key::AltRight       => 0x64, // 100
            // Linefeed         => 0x65, // 101
            Key::Home           => 0x66, // 102
            Key::ArrowUp        => 0x67, // 103
            Key::PageUp         => 0x68, // 104
            Key::ArrowLeft      => 0x69, // 105
            Key::ArrowRight     => 0x6A, // 106
            Key::End            => 0x6B, // 107
            Key::ArrowDown      => 0x6C, // 108
            Key::PageDown       => 0x6D, // 109
            Key::Insert         => 0x6E, // 110
            Key::Delete         => 0x6F, // 111
            // Macro            => 0x70, // 112
            // Mute             => 0x71, // 113
            // Volumedown       => 0x72, // 114
            // Volumeup         => 0x73, // 115
            // Power            => 0x74, // 116
            // Kpequal          => 0x75, // 117
            // Kpplusminus      => 0x76, // 118
            Key::Pause          => 0x77, // 119
            // Scale            => 0x78, // 120

            // Kpcomma          => 0x79, // 121
            // Hangeul          => 0x7A, // 122
            // Hanja            => 0x7B, // 123
            Key::IntlYen        => 0x7C, // 124
            Key::LogoLeft       => 0x7D, // 125
            Key::LogoRight      => 0x7E, // 126
            // Compose          => 0x7F, // 127

            // Stop             => 0x80, // 128
            // Again            => 0x81, // 129
            // Props            => 0x82, // 130
            // Undo             => 0x83, // 131
            // Front            => 0x84, // 132
            // Copy             => 0x85, // 133
            // Open             => 0x86, // 134
            // Paste            => 0x87, // 135
            // Find             => 0x88, // 136
            // Cut              => 0x89, // 137
            Key::Help           => 0x8A, // 138
            Key::ContextMenu    => 0x8B, // 139

            Key::F13            => 0xB7, // 183
            Key::F14            => 0xB8, // 184
            Key::F15            => 0xB9, // 185
            Key::F16            => 0xBA, // 186
            Key::F17            => 0xBB, // 187
            Key::F18            => 0xBC, // 188
            Key::F19            => 0xBD, // 189
            Key::F20            => 0xBE, // 190
            Key::F21            => 0xBF, // 191
            Key::F22            => 0xC0, // 192
            Key::F23            => 0xC1, // 193
            Key::F24            => 0xC2, // 194
        })
    }

    fn scancode_name(&self, sc: u32) -> String {
        // Get keysym from key
        let state = xkb::State::new(&self.keymap);
        let keysym = state.key_get_one_sym(sc);
        let mut key_name = xkb::keysym_get_name(keysym);
        if key_name.len() == 1 {
            key_name.make_ascii_uppercase();
        }
        key_name
    }
}
