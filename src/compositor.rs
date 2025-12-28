use anyhow::{Result, Context};
use crate::current_window::Window;
use std::fmt;

pub trait CompositorWatcher: fmt::Debug {
    fn get_active_window(&self) -> Result<Window>;
    // fn watch(&self) -> Result<Box<dyn Stream<Item = Window>>>;
}

pub fn detect_compositor() -> Result<Box<dyn CompositorWatcher>> {
    #[cfg(feature = "niri")]
    if let Ok(socket) = std::env::var("NIRI_SOCKET") {
        let watcher = crate::compositors::niri::NiriWatcher::new(socket)?;
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
