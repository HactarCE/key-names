use std::cell::RefCell;
use std::rc::Rc;
use thiserror::Error;
use wayland_client::protocol::wl_keyboard::{KeymapFormat, WlKeyboard};
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::{DispatchData, Main};
use xkb::x11::{MIN_MAJOR_XKB_VERSION, MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb::{self, KEYMAP_COMPILE_NO_FLAGS, KEYMAP_FORMAT_TEXT_V1};

use super::*;

pub const ALT_STR: &str = "Alt";
pub const LOGO_STR: &str = "Super";
pub const MODIFIERS_ORDER: &str = "csam"; // Ctrl + Shift + Alt + Meta

pub const SC_INVALID: u16 = 0x0000;
pub const SC_TO_KEY_MAPPING: fn(u16) -> KeyMapping = KeyMapping::Evdev;
pub const KEY_MAP_TO_SC: fn(KeyMap) -> u16 = |k| k.evdev;

thread_local! {
    static XKB_KEYMAP: xkb::Keymap =
        new_keymap().expect("failed to connect to X11 or Wayland to get keymap");
}

pub fn scancode_name(sc: u16) -> String {
    let keysym = XKB_KEYMAP.with(|xkb_keymap| {
        // Get keysym from key.
        //
        // According to the xkbcommon documentation, there is a fixed offset
        // of 8 between X11-compatible keymaps and Linux evdev scancodes:
        // https://docs.rs/xkbcommon/latest/xkbcommon/xkb/type.Keycode.html
        xkb::State::new(xkb_keymap).key_get_one_sym(sc as u32 + 8)
    });
    let mut key_name = xkb::keysym_get_name(keysym);
    if key_name.len() == 1 {
        key_name.make_ascii_uppercase();
    }
    key_name
}

/// Stolen shamelessly from winit:
/// https://github.com/rust-windowing/winit/blob/bcd76d47186b074e536ca5ab9714953931796243/src/platform_impl/linux/x11/events.rs#L5-L1008
///
/// I removed a big chunk of commented-out lines.
#[cfg(feature = "winit")]
pub fn key_map_to_winit_vkey(key: KeyMap) -> Option<winit::event::VirtualKeyCode> {
    use winit::event::VirtualKeyCode;
    use xkb::keysyms;

    let keysym = XKB_KEYMAP.with(|xkb_keymap| {
        // Get keysym from key.
        xkb::State::new(xkb_keymap).key_get_one_sym(key.evdev as u32)
    });

    Some(match keysym {
        keysyms::KEY_BackSpace => VirtualKeyCode::Back,
        keysyms::KEY_Tab => VirtualKeyCode::Tab,
        //keysyms::KEY_Linefeed => VirtualKeyCode::Linefeed,
        //keysyms::KEY_Clear => VirtualKeyCode::Clear,
        keysyms::KEY_Return => VirtualKeyCode::Return,
        keysyms::KEY_Pause => VirtualKeyCode::Pause,
        //keysyms::KEY_Scroll_Lock => VirtualKeyCode::Scroll_lock,
        //keysyms::KEY_Sys_Req => VirtualKeyCode::Sys_req,
        keysyms::KEY_Escape => VirtualKeyCode::Escape,
        keysyms::KEY_Delete => VirtualKeyCode::Delete,
        keysyms::KEY_Multi_key => VirtualKeyCode::Compose,
        //keysyms::KEY_Kanji => VirtualKeyCode::Kanji,
        //keysyms::KEY_Muhenkan => VirtualKeyCode::Muhenkan,
        //keysyms::KEY_Henkan_Mode => VirtualKeyCode::Henkan_mode,
        //keysyms::KEY_Henkan => VirtualKeyCode::Henkan,
        //keysyms::KEY_Romaji => VirtualKeyCode::Romaji,
        //keysyms::KEY_Hiragana => VirtualKeyCode::Hiragana,
        //keysyms::KEY_Katakana => VirtualKeyCode::Katakana,
        //keysyms::KEY_Hiragana_Katakana => VirtualKeyCode::Hiragana_katakana,
        //keysyms::KEY_Zenkaku => VirtualKeyCode::Zenkaku,
        //keysyms::KEY_Hankaku => VirtualKeyCode::Hankaku,
        //keysyms::KEY_Zenkaku_Hankaku => VirtualKeyCode::Zenkaku_hankaku,
        //keysyms::KEY_Touroku => VirtualKeyCode::Touroku,
        //keysyms::KEY_Massyo => VirtualKeyCode::Massyo,
        //keysyms::KEY_Kana_Lock => VirtualKeyCode::Kana_lock,
        //keysyms::KEY_Kana_Shift => VirtualKeyCode::Kana_shift,
        //keysyms::KEY_Eisu_Shift => VirtualKeyCode::Eisu_shift,
        //keysyms::KEY_Eisu_toggle => VirtualKeyCode::Eisu_toggle,
        keysyms::KEY_Home => VirtualKeyCode::Home,
        keysyms::KEY_Left => VirtualKeyCode::Left,
        keysyms::KEY_Up => VirtualKeyCode::Up,
        keysyms::KEY_Right => VirtualKeyCode::Right,
        keysyms::KEY_Down => VirtualKeyCode::Down,
        //keysyms::KEY_Prior => VirtualKeyCode::Prior,
        keysyms::KEY_Page_Up => VirtualKeyCode::PageUp,
        //keysyms::KEY_Next => VirtualKeyCode::Next,
        keysyms::KEY_Page_Down => VirtualKeyCode::PageDown,
        keysyms::KEY_End => VirtualKeyCode::End,
        //keysyms::KEY_Begin => VirtualKeyCode::Begin,
        //keysyms::KEY_Win_L => VirtualKeyCode::Win_l,
        //keysyms::KEY_Win_R => VirtualKeyCode::Win_r,
        //keysyms::KEY_App => VirtualKeyCode::App,
        //keysyms::KEY_Select => VirtualKeyCode::Select,
        //keysyms::KEY_Print => VirtualKeyCode::Print,
        //keysyms::KEY_Execute => VirtualKeyCode::Execute,
        keysyms::KEY_Insert => VirtualKeyCode::Insert,
        //keysyms::KEY_Undo => VirtualKeyCode::Undo,
        //keysyms::KEY_Redo => VirtualKeyCode::Redo,
        //keysyms::KEY_Menu => VirtualKeyCode::Menu,
        //keysyms::KEY_Find => VirtualKeyCode::Find,
        //keysyms::KEY_Cancel => VirtualKeyCode::Cancel,
        //keysyms::KEY_Help => VirtualKeyCode::Help,
        //keysyms::KEY_Break => VirtualKeyCode::Break,
        //keysyms::KEY_Mode_switch => VirtualKeyCode::Mode_switch,
        //keysyms::KEY_script_switch => VirtualKeyCode::Script_switch,
        keysyms::KEY_Num_Lock => VirtualKeyCode::Numlock,
        //keysyms::KEY_KP_Space => VirtualKeyCode::Kp_space,
        //keysyms::KEY_KP_Tab => VirtualKeyCode::Kp_tab,
        keysyms::KEY_KP_Enter => VirtualKeyCode::NumpadEnter,
        //keysyms::KEY_KP_F1 => VirtualKeyCode::Kp_f1,
        //keysyms::KEY_KP_F2 => VirtualKeyCode::Kp_f2,
        //keysyms::KEY_KP_F3 => VirtualKeyCode::Kp_f3,
        //keysyms::KEY_KP_F4 => VirtualKeyCode::Kp_f4,
        keysyms::KEY_KP_Home => VirtualKeyCode::Home,
        keysyms::KEY_KP_Left => VirtualKeyCode::Left,
        keysyms::KEY_KP_Up => VirtualKeyCode::Up,
        keysyms::KEY_KP_Right => VirtualKeyCode::Right,
        keysyms::KEY_KP_Down => VirtualKeyCode::Down,
        //keysyms::KEY_KP_Prior => VirtualKeyCode::Kp_prior,
        keysyms::KEY_KP_Page_Up => VirtualKeyCode::PageUp,
        //keysyms::KEY_KP_Next => VirtualKeyCode::Kp_next,
        keysyms::KEY_KP_Page_Down => VirtualKeyCode::PageDown,
        keysyms::KEY_KP_End => VirtualKeyCode::End,
        //keysyms::KEY_KP_Begin => VirtualKeyCode::Kp_begin,
        keysyms::KEY_KP_Insert => VirtualKeyCode::Insert,
        keysyms::KEY_KP_Delete => VirtualKeyCode::Delete,
        keysyms::KEY_KP_Equal => VirtualKeyCode::NumpadEquals,
        keysyms::KEY_KP_Multiply => VirtualKeyCode::NumpadMultiply,
        keysyms::KEY_KP_Add => VirtualKeyCode::NumpadAdd,
        keysyms::KEY_KP_Separator => VirtualKeyCode::NumpadComma,
        keysyms::KEY_KP_Subtract => VirtualKeyCode::NumpadSubtract,
        keysyms::KEY_KP_Decimal => VirtualKeyCode::NumpadDecimal,
        keysyms::KEY_KP_Divide => VirtualKeyCode::NumpadDivide,
        keysyms::KEY_KP_0 => VirtualKeyCode::Numpad0,
        keysyms::KEY_KP_1 => VirtualKeyCode::Numpad1,
        keysyms::KEY_KP_2 => VirtualKeyCode::Numpad2,
        keysyms::KEY_KP_3 => VirtualKeyCode::Numpad3,
        keysyms::KEY_KP_4 => VirtualKeyCode::Numpad4,
        keysyms::KEY_KP_5 => VirtualKeyCode::Numpad5,
        keysyms::KEY_KP_6 => VirtualKeyCode::Numpad6,
        keysyms::KEY_KP_7 => VirtualKeyCode::Numpad7,
        keysyms::KEY_KP_8 => VirtualKeyCode::Numpad8,
        keysyms::KEY_KP_9 => VirtualKeyCode::Numpad9,
        keysyms::KEY_F1 => VirtualKeyCode::F1,
        keysyms::KEY_F2 => VirtualKeyCode::F2,
        keysyms::KEY_F3 => VirtualKeyCode::F3,
        keysyms::KEY_F4 => VirtualKeyCode::F4,
        keysyms::KEY_F5 => VirtualKeyCode::F5,
        keysyms::KEY_F6 => VirtualKeyCode::F6,
        keysyms::KEY_F7 => VirtualKeyCode::F7,
        keysyms::KEY_F8 => VirtualKeyCode::F8,
        keysyms::KEY_F9 => VirtualKeyCode::F9,
        keysyms::KEY_F10 => VirtualKeyCode::F10,
        keysyms::KEY_F11 => VirtualKeyCode::F11,
        //keysyms::KEY_L1 => VirtualKeyCode::L1,
        keysyms::KEY_F12 => VirtualKeyCode::F12,
        //keysyms::KEY_L2 => VirtualKeyCode::L2,
        keysyms::KEY_F13 => VirtualKeyCode::F13,
        //keysyms::KEY_L3 => VirtualKeyCode::L3,
        keysyms::KEY_F14 => VirtualKeyCode::F14,
        //keysyms::KEY_L4 => VirtualKeyCode::L4,
        keysyms::KEY_F15 => VirtualKeyCode::F15,
        //keysyms::KEY_L5 => VirtualKeyCode::L5,
        keysyms::KEY_F16 => VirtualKeyCode::F16,
        //keysyms::KEY_L6 => VirtualKeyCode::L6,
        keysyms::KEY_F17 => VirtualKeyCode::F17,
        //keysyms::KEY_L7 => VirtualKeyCode::L7,
        keysyms::KEY_F18 => VirtualKeyCode::F18,
        //keysyms::KEY_L8 => VirtualKeyCode::L8,
        keysyms::KEY_F19 => VirtualKeyCode::F19,
        //keysyms::KEY_L9 => VirtualKeyCode::L9,
        keysyms::KEY_F20 => VirtualKeyCode::F20,
        //keysyms::KEY_L10 => VirtualKeyCode::L10,
        keysyms::KEY_F21 => VirtualKeyCode::F21,
        //keysyms::KEY_R1 => VirtualKeyCode::R1,
        keysyms::KEY_F22 => VirtualKeyCode::F22,
        //keysyms::KEY_R2 => VirtualKeyCode::R2,
        keysyms::KEY_F23 => VirtualKeyCode::F23,
        //keysyms::KEY_R3 => VirtualKeyCode::R3,
        keysyms::KEY_F24 => VirtualKeyCode::F24,
        //keysyms::KEY_R4 => VirtualKeyCode::R4,
        //keysyms::KEY_F25 => VirtualKeyCode::F25,
        //keysyms::KEY_R5 => VirtualKeyCode::R5,
        //keysyms::KEY_F26 => VirtualKeyCode::F26,
        //keysyms::KEY_R6 => VirtualKeyCode::R6,
        //keysyms::KEY_F27 => VirtualKeyCode::F27,
        //keysyms::KEY_R7 => VirtualKeyCode::R7,
        //keysyms::KEY_F28 => VirtualKeyCode::F28,
        //keysyms::KEY_R8 => VirtualKeyCode::R8,
        //keysyms::KEY_F29 => VirtualKeyCode::F29,
        //keysyms::KEY_R9 => VirtualKeyCode::R9,
        //keysyms::KEY_F30 => VirtualKeyCode::F30,
        //keysyms::KEY_R10 => VirtualKeyCode::R10,
        //keysyms::KEY_F31 => VirtualKeyCode::F31,
        //keysyms::KEY_R11 => VirtualKeyCode::R11,
        //keysyms::KEY_F32 => VirtualKeyCode::F32,
        //keysyms::KEY_R12 => VirtualKeyCode::R12,
        //keysyms::KEY_F33 => VirtualKeyCode::F33,
        //keysyms::KEY_R13 => VirtualKeyCode::R13,
        //keysyms::KEY_F34 => VirtualKeyCode::F34,
        //keysyms::KEY_R14 => VirtualKeyCode::R14,
        //keysyms::KEY_F35 => VirtualKeyCode::F35,
        //keysyms::KEY_R15 => VirtualKeyCode::R15,
        keysyms::KEY_Shift_L => VirtualKeyCode::LShift,
        keysyms::KEY_Shift_R => VirtualKeyCode::RShift,
        keysyms::KEY_Control_L => VirtualKeyCode::LControl,
        keysyms::KEY_Control_R => VirtualKeyCode::RControl,
        //keysyms::KEY_Caps_Lock => VirtualKeyCode::Caps_lock,
        //keysyms::KEY_Shift_Lock => VirtualKeyCode::Shift_lock,
        //keysyms::KEY_Meta_L => VirtualKeyCode::Meta_l,
        //keysyms::KEY_Meta_R => VirtualKeyCode::Meta_r,
        keysyms::KEY_Alt_L => VirtualKeyCode::LAlt,
        keysyms::KEY_Alt_R => VirtualKeyCode::RAlt,
        //keysyms::KEY_Super_L => VirtualKeyCode::Super_l,
        //keysyms::KEY_Super_R => VirtualKeyCode::Super_r,
        //keysyms::KEY_Hyper_L => VirtualKeyCode::Hyper_l,
        //keysyms::KEY_Hyper_R => VirtualKeyCode::Hyper_r,
        keysyms::KEY_ISO_Left_Tab => VirtualKeyCode::Tab,
        keysyms::KEY_space => VirtualKeyCode::Space,
        //keysyms::KEY_exclam => VirtualKeyCode::Exclam,
        //keysyms::KEY_quotedbl => VirtualKeyCode::Quotedbl,
        //keysyms::KEY_numbersign => VirtualKeyCode::Numbersign,
        //keysyms::KEY_dollar => VirtualKeyCode::Dollar,
        //keysyms::KEY_percent => VirtualKeyCode::Percent,
        //keysyms::KEY_ampersand => VirtualKeyCode::Ampersand,
        keysyms::KEY_apostrophe => VirtualKeyCode::Apostrophe,
        //keysyms::KEY_quoteright => VirtualKeyCode::Quoteright,
        //keysyms::KEY_parenleft => VirtualKeyCode::Parenleft,
        //keysyms::KEY_parenright => VirtualKeyCode::Parenright,
        keysyms::KEY_asterisk => VirtualKeyCode::Asterisk,
        keysyms::KEY_plus => VirtualKeyCode::Plus,
        keysyms::KEY_comma => VirtualKeyCode::Comma,
        keysyms::KEY_minus => VirtualKeyCode::Minus,
        keysyms::KEY_period => VirtualKeyCode::Period,
        keysyms::KEY_slash => VirtualKeyCode::Slash,
        keysyms::KEY_0 => VirtualKeyCode::Key0,
        keysyms::KEY_1 => VirtualKeyCode::Key1,
        keysyms::KEY_2 => VirtualKeyCode::Key2,
        keysyms::KEY_3 => VirtualKeyCode::Key3,
        keysyms::KEY_4 => VirtualKeyCode::Key4,
        keysyms::KEY_5 => VirtualKeyCode::Key5,
        keysyms::KEY_6 => VirtualKeyCode::Key6,
        keysyms::KEY_7 => VirtualKeyCode::Key7,
        keysyms::KEY_8 => VirtualKeyCode::Key8,
        keysyms::KEY_9 => VirtualKeyCode::Key9,
        keysyms::KEY_colon => VirtualKeyCode::Colon,
        keysyms::KEY_semicolon => VirtualKeyCode::Semicolon,
        //keysyms::KEY_less => VirtualKeyCode::Less,
        keysyms::KEY_equal => VirtualKeyCode::Equals,
        //keysyms::KEY_greater => VirtualKeyCode::Greater,
        //keysyms::KEY_question => VirtualKeyCode::Question,
        keysyms::KEY_at => VirtualKeyCode::At,
        keysyms::KEY_A => VirtualKeyCode::A,
        keysyms::KEY_B => VirtualKeyCode::B,
        keysyms::KEY_C => VirtualKeyCode::C,
        keysyms::KEY_D => VirtualKeyCode::D,
        keysyms::KEY_E => VirtualKeyCode::E,
        keysyms::KEY_F => VirtualKeyCode::F,
        keysyms::KEY_G => VirtualKeyCode::G,
        keysyms::KEY_H => VirtualKeyCode::H,
        keysyms::KEY_I => VirtualKeyCode::I,
        keysyms::KEY_J => VirtualKeyCode::J,
        keysyms::KEY_K => VirtualKeyCode::K,
        keysyms::KEY_L => VirtualKeyCode::L,
        keysyms::KEY_M => VirtualKeyCode::M,
        keysyms::KEY_N => VirtualKeyCode::N,
        keysyms::KEY_O => VirtualKeyCode::O,
        keysyms::KEY_P => VirtualKeyCode::P,
        keysyms::KEY_Q => VirtualKeyCode::Q,
        keysyms::KEY_R => VirtualKeyCode::R,
        keysyms::KEY_S => VirtualKeyCode::S,
        keysyms::KEY_T => VirtualKeyCode::T,
        keysyms::KEY_U => VirtualKeyCode::U,
        keysyms::KEY_V => VirtualKeyCode::V,
        keysyms::KEY_W => VirtualKeyCode::W,
        keysyms::KEY_X => VirtualKeyCode::X,
        keysyms::KEY_Y => VirtualKeyCode::Y,
        keysyms::KEY_Z => VirtualKeyCode::Z,
        keysyms::KEY_bracketleft => VirtualKeyCode::LBracket,
        keysyms::KEY_backslash => VirtualKeyCode::Backslash,
        keysyms::KEY_bracketright => VirtualKeyCode::RBracket,
        //keysyms::KEY_asciicircum => VirtualKeyCode::Asciicircum,
        //keysyms::KEY_underscore => VirtualKeyCode::Underscore,
        keysyms::KEY_grave => VirtualKeyCode::Grave,
        //keysyms::KEY_quoteleft => VirtualKeyCode::Quoteleft,
        keysyms::KEY_a => VirtualKeyCode::A,
        keysyms::KEY_b => VirtualKeyCode::B,
        keysyms::KEY_c => VirtualKeyCode::C,
        keysyms::KEY_d => VirtualKeyCode::D,
        keysyms::KEY_e => VirtualKeyCode::E,
        keysyms::KEY_f => VirtualKeyCode::F,
        keysyms::KEY_g => VirtualKeyCode::G,
        keysyms::KEY_h => VirtualKeyCode::H,
        keysyms::KEY_i => VirtualKeyCode::I,
        keysyms::KEY_j => VirtualKeyCode::J,
        keysyms::KEY_k => VirtualKeyCode::K,
        keysyms::KEY_l => VirtualKeyCode::L,
        keysyms::KEY_m => VirtualKeyCode::M,
        keysyms::KEY_n => VirtualKeyCode::N,
        keysyms::KEY_o => VirtualKeyCode::O,
        keysyms::KEY_p => VirtualKeyCode::P,
        keysyms::KEY_q => VirtualKeyCode::Q,
        keysyms::KEY_r => VirtualKeyCode::R,
        keysyms::KEY_s => VirtualKeyCode::S,
        keysyms::KEY_t => VirtualKeyCode::T,
        keysyms::KEY_u => VirtualKeyCode::U,
        keysyms::KEY_v => VirtualKeyCode::V,
        keysyms::KEY_w => VirtualKeyCode::W,
        keysyms::KEY_x => VirtualKeyCode::X,
        keysyms::KEY_y => VirtualKeyCode::Y,
        keysyms::KEY_z => VirtualKeyCode::Z,
        //keysyms::KEY_braceleft => VirtualKeyCode::Braceleft,
        //keysyms::KEY_bar => VirtualKeyCode::Bar,
        //keysyms::KEY_braceright => VirtualKeyCode::Braceright,
        //keysyms::KEY_asciitilde => VirtualKeyCode::Asciitilde,
        //keysyms::KEY_nobreakspace => VirtualKeyCode::Nobreakspace,
        //keysyms::KEY_exclamdown => VirtualKeyCode::Exclamdown,
        // ... removed some lines here
        keysyms::KEY_XF86Back => VirtualKeyCode::NavigateBackward,
        keysyms::KEY_XF86Forward => VirtualKeyCode::NavigateForward,
        keysyms::KEY_XF86Copy => VirtualKeyCode::Copy,
        keysyms::KEY_XF86Paste => VirtualKeyCode::Paste,
        keysyms::KEY_XF86Cut => VirtualKeyCode::Cut,
        _ => return None,
    })
}

/// Constructs a keymap using either X11 or Wayland automatically.
fn new_keymap() -> Result<xkb::Keymap, KeymapError> {
    match std::env::var("XDG_SESSION_TYPE") {
        Ok(session_type) => match session_type.as_str() {
            "wayland" if std::env::var("WAYLAND_DISPLAY").is_ok() => return new_wayland_keymap(),
            "x11" => return new_x11_keymap(),
            _ => (),
        },
        Err(_) => (),
    }
    // Just try both and return whichever succeeds.
    new_wayland_keymap().or_else(|_| new_x11_keymap())
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

/// Constructs a keymap in an X11 environment.
fn new_x11_keymap() -> Result<xkb::Keymap, KeymapError> {
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

    Ok(xkb::x11::keymap_new_from_device(&ctx, &conn, device_id, 0))
}

/// Constructs a keymap in a Wayland environment.
fn new_wayland_keymap() -> Result<xkb::Keymap, KeymapError> {
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
            if let wayland_client::protocol::wl_keyboard::Event::Keymap { format, fd, size } = event
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

    Ok(keymap)
}
