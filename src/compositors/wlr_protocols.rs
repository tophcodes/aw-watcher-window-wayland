// The generated code tends to trigger a lot of warnings
// so we isolate it into a very permissive module
#![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
#![allow(non_upper_case_globals,non_snake_case,unused_imports)]

use anyhow::Result;
use crate::current_window::Window;
use crate::compositor::CompositorWatcher;
use std::fmt;

// Re-export only the actual code, and then only use this re-export
// The `generated` module below is just some boilerplate to properly isolate stuff
// and avoid exposing internal details.
//
// You can use all the types from my_protocol as if they went from `wayland_client::protocol`.

pub mod toplevel_management {
    pub(crate) use smallvec;
    pub(crate) use wayland_sys as sys;
    pub(crate) use wayland_client::{AnonymousObject, Interface, Main, Proxy, ProxyMap};
    pub(crate) use wayland_client::protocol::{wl_surface, wl_region, wl_seat, wl_output};
    pub(crate) use wayland_commons::{MessageGroup};
    pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
    pub(crate) use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};

    include!("protocols/wlr-foreign-toplevel-management.rs");
}

pub mod kde_idle {
    pub(crate) use smallvec;
    pub(crate) use wayland_sys as sys;
    pub(crate) use wayland_client::{AnonymousObject, Interface, Main, Proxy, ProxyMap};
    pub(crate) use wayland_client::protocol::{wl_surface, wl_region, wl_seat, wl_output};
    pub(crate) use wayland_commons::{MessageGroup};
    pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
    pub(crate) use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};

    include!("protocols/idle.rs");
}

pub mod ext_idle {
    pub(crate) use smallvec;
    pub(crate) use wayland_sys as sys;
    pub(crate) use wayland_client::{AnonymousObject, Interface, Main, Proxy, ProxyMap};
    pub(crate) use wayland_client::protocol::{wl_surface, wl_region, wl_seat, wl_output};
    pub(crate) use wayland_commons::{MessageGroup};
    pub(crate) use wayland_commons::map::{Object, ObjectMetadata};
    pub(crate) use wayland_commons::wire::{Argument, ArgumentType, Message, MessageDesc};

    include!("protocols/ext-idle-notify-v1.rs");
}

pub struct WlrProtocolsWatcher {

}

impl WlrProtocolsWatcher {
    
}

impl CompositorWatcher for WlrProtocolsWatcher {
    fn get_active_window(&self) -> Result<Window> {
        Ok(Window {
            title: "unknown".into(),
            appid: "unknown".into(),
            pid: None,
        })
    }
}

impl fmt::Debug for WlrProtocolsWatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WlrProtocolsWatcher")
    }
}
