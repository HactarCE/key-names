use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use wayland_client::protocol::wl_keyboard::{KeymapFormat, WlKeyboard};
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::{DispatchData, Main};
use xkb::x11::{MIN_MAJOR_XKB_VERSION, MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb::{self, KEYMAP_COMPILE_NO_FLAGS, KEYMAP_FORMAT_TEXT_V1};

use super::*;

const INVALID: u16 = 0x0000;

#[derive(Clone)]
pub struct LinuxKeyNamer {
    keymap: xkb::Keymap,
}
impl fmt::Debug for LinuxKeyNamer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LinuxKeyNamer")
    }
}
impl OsKeynamer for LinuxKeyNamer {
    fn logo_str(&self) -> &'static str {
        "Super"
    }

    fn decode_scancode(&self, sc: u16) -> Option<KeyMappingCode> {
        KeyMap::try_from(KeyMapping::Xkb(sc as u16)).ok()?.code
    }
    fn encode_scancode(&self, key: KeyMappingCode) -> Option<u16> {
        Some(KeyMap::try_from(KeyMapping::Code(Some(key))).ok()?.xkb as u16)
            .filter(|&sc| sc != INVALID)
    }

    fn scancode_name(&self, sc: u16) -> String {
        // Get keysym from key.
        let state = xkb::State::new(&self.keymap);
        let keysym = state.key_get_one_sym(sc as u32);
        let mut key_name = xkb::keysym_get_name(keysym);
        if key_name.len() == 1 {
            key_name.make_ascii_uppercase();
        }
        key_name
    }
}
impl LinuxKeyNamer {
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
