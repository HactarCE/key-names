use thiserror::Error;
use wayland_client::protocol::{wl_keyboard, wl_registry, wl_seat};
use winit::keyboard::{NamedKey, PhysicalKey};
use winit::platform::scancode::PhysicalKeyExtScancode;
use xkb::x11::{MIN_MAJOR_XKB_VERSION, MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb;

pub const ALT_STR: &str = "Alt";
pub const LOGO_STR: &str = "Super";
pub const MODIFIERS_ORDER: &str = "csam"; // Ctrl + Shift + Alt + Meta

pub fn os_specific_key_name(key: NamedKey) -> Option<&'static str> {
    match key {
        NamedKey::AltGraph => Some("AltGr"),
        NamedKey::ArrowDown => Some("Down"),
        NamedKey::ArrowLeft => Some("Left"),
        NamedKey::ArrowRight => Some("Right"),
        NamedKey::ArrowUp => Some("Up"),
        _ => None,
    }
}

pub fn try_physical_key_name(physical_key: PhysicalKey) -> Option<String> {
    physical_key
        .to_scancode()
        .map(|sc| scancode_name(sc as u16))
}

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
        // https://docs.rs/xkbcommon/0.8.0/xkbcommon/xkb/struct.Keycode.html
        xkb::State::new(xkb_keymap).key_get_one_sym(xkb::Keycode::new(sc as u32 + 8))
    });
    match keysym.raw() {
        // Better names for numpad keys
        xkb::keysyms::KEY_KP_Insert => "Numpad0".to_string(),
        xkb::keysyms::KEY_KP_End => "Numpad1".to_string(),
        xkb::keysyms::KEY_KP_Down => "Numpad2".to_string(),
        xkb::keysyms::KEY_KP_Next => "Numpad3".to_string(),
        xkb::keysyms::KEY_KP_Left => "Numpad4".to_string(),
        xkb::keysyms::KEY_KP_Begin => "Numpad5".to_string(),
        xkb::keysyms::KEY_KP_Right => "Numpad6".to_string(),
        xkb::keysyms::KEY_KP_Home => "Numpad7".to_string(),
        xkb::keysyms::KEY_KP_Up => "Numpad8".to_string(),
        xkb::keysyms::KEY_KP_Prior => "Numpad9".to_string(),
        xkb::keysyms::KEY_KP_Add => "NumpadAdd".to_string(),
        xkb::keysyms::KEY_KP_Decimal => "NumpadComma".to_string(),
        xkb::keysyms::KEY_KP_Delete => "NumpadDecimal".to_string(),
        xkb::keysyms::KEY_KP_Divide => "NumpadDivide".to_string(),
        xkb::keysyms::KEY_KP_Enter => "NumpadEnter".to_string(),
        xkb::keysyms::KEY_KP_Equal => "NumpadEqual".to_string(),
        xkb::keysyms::KEY_KP_Multiply => "NumpadMultiply".to_string(),
        xkb::keysyms::KEY_KP_Subtract => "NumpadSubtract".to_string(),

        _ => {
            let mut key_name = xkb::keysym_get_name(keysym);
            if key_name.len() == 1 {
                key_name.make_ascii_uppercase();
            }
            key_name
        }
    }
}

/// Constructs a keymap using either X11 or Wayland automatically.
fn new_keymap() -> Result<xkb::Keymap, KeymapError> {
    // Just try both and return whichever succeeds.
    let wayland_error = match new_wayland_keymap() {
        Ok(keymap) => return Ok(keymap),
        Err(e) => e,
    };
    let x11_error = match new_x11_keymap() {
        Ok(keymap) => return Ok(keymap),
        Err(e) => e,
    };
    // Decide which error to report
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        match session_type.as_str() {
            "wayland" if std::env::var("WAYLAND_DISPLAY").is_ok() => Err(wayland_error),
            "x11" => Err(x11_error),
            _ => Err(x11_error),
        }
    } else {
        Err(x11_error)
    }
}

#[derive(Error, Debug)]
pub enum KeymapError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unable to connect to X server")]
    X11Connect,

    #[error("wayland dispatch error")]
    Wayland(#[from] wayland_client::DispatchError),
    #[error("unable to connect to wayland")]
    WaylandConnect,
    #[error("wl_seat not found in available interfaces")]
    MissingWlSeat,
    #[error("wl_seat does not have keyboard capability")]
    MissingKeyboardCapability,
    #[error("failed to create keymap")]
    FailedToCreateKeymap,
    #[error("unsupported keymap format: {0:?}")]
    UnsupportedKeymapFormat(wayland_client::WEnum<wl_keyboard::KeymapFormat>),
}

/// Constructs a keymap in an X11 environment.
fn new_x11_keymap() -> Result<xkb::Keymap, KeymapError> {
    // This code is modified from Frinksy's `keyboard-keynames` crate:
    // https://gitlab.com/Frinksy/keyboard-keynames/-/blob/master/src/platform/unix/key_layout.rs

    let (connection, _) = xcb::Connection::connect(None).map_err(|_| KeymapError::X11Connect)?;
    let mut major_xkb_version_out = 0;
    let mut minor_xkb_version_out = 0;
    let mut base_event_out = 0;
    let mut base_error_out = 0;

    let _ = xkb::x11::setup_xkb_extension(
        &connection,
        MIN_MAJOR_XKB_VERSION,
        MIN_MINOR_XKB_VERSION,
        xkb::x11::SetupXkbExtensionFlags::NoFlags,
        &mut major_xkb_version_out,
        &mut minor_xkb_version_out,
        &mut base_event_out,
        &mut base_error_out,
    );

    let device_id = xkb::x11::get_core_keyboard_device_id(&connection);

    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    Ok(xkb::x11::keymap_new_from_device(
        &ctx,
        &connection,
        device_id,
        0,
    ))
}

/// Constructs a keymap in a Wayland environment.
fn new_wayland_keymap() -> Result<xkb::Keymap, KeymapError> {
    let connection =
        wayland_client::Connection::connect_to_env().map_err(|_| KeymapError::WaylandConnect)?;
    let display = connection.display();

    // Get the registry.
    let mut state = State::default();
    let mut event_queue = connection.new_event_queue::<State>();
    let qhandle = event_queue.handle();
    let _registry = display.get_registry(&qhandle, ());

    event_queue.roundtrip(&mut state)?; // Get WlSeat
    if !state.wl_seat {
        return Err(KeymapError::MissingWlSeat);
    }

    event_queue.roundtrip(&mut state)?; // Get WlKeyboard
    if !state.wl_keyboard {
        return Err(KeymapError::MissingKeyboardCapability);
    }

    event_queue.roundtrip(&mut state)?; // Get keymap
    state
        .keymap
        .clone()
        .ok_or(state.error.unwrap_or(KeymapError::FailedToCreateKeymap))
}

#[derive(Default)]
struct State {
    wl_seat: bool,
    wl_keyboard: bool,
    keymap: Option<xkb::Keymap>,
    error: Option<KeymapError>,
}

impl wayland_client::Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface.as_str() == "wl_seat" {
                state.wl_seat = true;
                registry.bind::<wl_seat::WlSeat, _, _>(name, version, qh, ());
            }
        }
    }
}

impl wayland_client::Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: wayland_client::WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                state.wl_keyboard = true;
                seat.get_keyboard(qh, ());
            }
        }
    }
}

impl wayland_client::Dispatch<wl_keyboard::WlKeyboard, ()> for State {
    fn event(
        state: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_keyboard::Event::Keymap { format, fd, size } = event {
            match format {
                wayland_client::WEnum::Value(wl_keyboard::KeymapFormat::XkbV1) => {
                    // Construct keymap from file descriptor
                    let ctx = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
                    let result = unsafe {
                        xkb::Keymap::new_from_fd(
                            &ctx,
                            fd,
                            size as usize,
                            xkb::KEYMAP_FORMAT_TEXT_V1,
                            xkb::KEYMAP_COMPILE_NO_FLAGS,
                        )
                    };
                    match result {
                        Ok(Some(keymap)) => state.keymap = Some(keymap),
                        Ok(None) => state.error = Some(KeymapError::FailedToCreateKeymap),
                        Err(e) => state.error = Some(KeymapError::Io(e)),
                    }
                }

                other => state.error = Some(KeymapError::UnsupportedKeymapFormat(other)),
            }
        }
    }
}

// Ignore events from other object types
wayland_client::delegate_noop!(State: ignore wayland_client::protocol::wl_compositor::WlCompositor);
wayland_client::delegate_noop!(State: ignore wayland_client::protocol::wl_surface::WlSurface);
wayland_client::delegate_noop!(State: ignore wayland_client::protocol::wl_shm::WlShm);
wayland_client::delegate_noop!(State: ignore wayland_client::protocol::wl_shm_pool::WlShmPool);
wayland_client::delegate_noop!(State: ignore wayland_client::protocol::wl_buffer::WlBuffer);
