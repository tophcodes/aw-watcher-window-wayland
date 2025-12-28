use anyhow::{Result, Context};
use niri_ipc::socket::Socket;
use crate::current_window::Window;
use crate::compositor::CompositorWatcher;
use mio::Registry;
// use mio::event::Source;
use std::fmt;

pub struct NiriEventSource {
    socket: Socket,
}

impl NiriEventSource {
    pub fn new(socket_path: String) -> Result<Self> {
        let socket = Socket::connect_to(socket_path)
            .context("Failed to connect to niri socket")?;

        Ok(Self { socket })
    }
}

impl CompositorWatcher for NiriEventSource {
    fn get_active_window(&self) -> Result<Window> {
        Ok(Window {
            title: "hi".into(),
            appid: "hi".into(),
            pid: None,
        })
    }
}

impl fmt::Debug for NiriEventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NiriEventSource")
    }
}

// impl Source for NiriEventSource {
//     fn register(&mut self, registry: &Registry, token: Token, interests: Interest)
//         -> io::Result<()>
//     {
//         // Delegate the `register` call to `socket`
//         self.socket.register(registry, token, interests)
//     }

//     fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest)
//         -> io::Result<()>
//     {
//         // Delegate the `reregister` call to `socket`
//         self.socket.reregister(registry, token, interests)
//     }

//     fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
//         // Delegate the `deregister` call to `socket`
//         self.socket.deregister(registry)
//     }
// }
