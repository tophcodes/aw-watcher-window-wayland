use anyhow::{Result, Context};
use niri_ipc::socket::Socket;
use niri_ipc::{Request, Reply, Response};
use crate::current_window::Window;
use crate::compositor::{Event, CompositorWatcher};
use std::os::unix::net::UnixStream;
use mio::Registry;
use mio::{Interest, Token};
use std::{fmt, io};
use std::path::Path;
use std::os::fd;
use std::io::{Write, BufReader, BufRead};

pub struct NiriEventSource {
    stream: io::BufReader<UnixStream>,
}

impl NiriEventSource {
    pub fn new(socket_path: impl AsRef<Path>) -> Result<Self> {
        let mut raw_stream: UnixStream = UnixStream::connect(socket_path.as_ref())?;

        let request = serde_json::to_string(&Request::EventStream)?;
        raw_stream.write_all(request.as_bytes())?;
        raw_stream.write_all(b"\n");
        raw_stream.flush()?;

        let mut buf = String::new();
        let mut reader = BufReader::new(raw_stream);
        reader.read_line(&mut buf)?;
        let reply: Reply = serde_json::from_str(&buf)?;

        let response = reply.map_err(|e| anyhow::anyhow!("niri error: {}", e))?;
        if !matches!(response, Response::Handled) {
            anyhow::bail!("Failed to start event stream");
        }

        reader.get_mut().set_nonblocking(true)?;
        reader.get_mut().shutdown(std::net::Shutdown::Write)?;

        Ok(Self { stream: reader })
    }
}

impl fd::AsRawFd for NiriEventSource {
    fn as_raw_fd(&self) -> fd::RawFd {
        let stream = self.stream.get_ref();
        fd::AsRawFd::as_raw_fd(stream)
    }
}

impl CompositorWatcher for NiriEventSource {
    fn read_event(&mut self) -> Result<Option<Event>> {
        let mut buf = String::new();

        match self.stream.read_line(&mut buf) {
            Ok(0) => anyhow::bail!("Connection closed"),
            Ok(_) => {
                use niri_ipc::Event as NiriEvent;

                let event: NiriEvent = serde_json::from_str(buf.trim())?;
                match event {
                    NiriEvent::WindowFocusChanged { id } => {
                        println!("window focus changed: {:?}", id);
                    }
                    _ => {
                        println!("{:?}", event);
                    }
                }
                Ok(None) //Some(event))
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl fmt::Debug for NiriEventSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NiriEventSource({:?})", fd::AsRawFd::as_raw_fd(self))
    }
}
