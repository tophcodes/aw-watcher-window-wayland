use anyhow::{Result, Context};
use niri_ipc::socket::Socket;
use crate::current_window::Window;
use crate::compositor::CompositorWatcher;
use std::fmt;

pub struct NiriWatcher {
    socket: Socket,
}

impl NiriWatcher {
    pub fn new(socket_path: String) -> Result<Self> {
        let socket = Socket::connect_to(socket_path)
            .context("Failed to connect to niri socket")?;

        Ok(Self { socket })
    }
}

impl CompositorWatcher for NiriWatcher {
    fn get_active_window(&self) -> Result<Window> {
        Ok(Window {
            title: "hi".into(),
            appid: "hi".into(),
            pid: None,
        })
    }
}

impl fmt::Debug for NiriWatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NiriWatcher")
    }
}

