use anyhow::Result;
use crate::current_window::Window;
use crate::compositor::CompositorWatcher;
use std::fmt;

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
