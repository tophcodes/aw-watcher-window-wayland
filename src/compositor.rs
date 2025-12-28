use anyhow::Result;
use crate::current_window::Window;
use std::fmt;
use niri_ipc::socket::SOCKET_PATH_ENV as NIRI_SOCKET_PATH_ENV;
use mio::event::Source;
use std::os::fd;

pub trait CompositorWatcher: fmt::Debug + fd::AsRawFd {
    fn read_event(&mut self) -> Result<()>;
    fn get_active_window(&self) -> Option<Window>;
}

pub fn detect_compositor() -> Result<Box<dyn CompositorWatcher>> {
    #[cfg(feature = "niri")]
    if let Ok(socket_path) = std::env::var(NIRI_SOCKET_PATH_ENV) {
        let watcher = crate::compositors::niri::NiriEventSource::new(socket_path)?;
        return Ok(Box::new(watcher));
    }

    // #[cfg(feature = "wlr-protocols")]
    // if let Ok(watcher) = create::compositors::wlr_client::WlrWatcher::new() {
    //     return Ok(Box::new(watcher));
    // }

    anyhow::bail!(
        "No supported Wayland compositor detected.\n\
         Supported: niri or any wlr-protocols compositor.\n\
         Make sure you're running under Wayland and the compositor is active."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_niri_detection() {
        std::env::set_var("NIRI_SOCKET", "/tmp/test-socket");

        // This will fail to connect, but should detect
        let result = detect_compositor();
        assert!(result.is_err());
        std::env::remove_var("NIRI_SOCKET");
    }
}
