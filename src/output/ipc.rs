use crate::audio::analyzer::VisualizerFrame;
use serde::Serialize;
use serde_json::to_string;
use std::fs;
use std::io::Write;
use std::os::unix::net::{UnixListener, UnixStream};

use super::VisualizerOutput;

#[derive(Serialize)]
struct VisualizerPacket<'a> {
    version: u32,
    sample_rate: u32,
    bands: &'a [f32],
    rms: f32,
    peak: f32,
}

pub struct IpcOutput {
    listener: UnixListener,
    clients: Vec<UnixStream>,
    path: String,
    sample_rate: u32,
}

impl IpcOutput {
    pub fn new(path: impl Into<String>, sample_rate: u32) -> std::io::Result<Self> {
        let path = path.into();

        let _ = fs::remove_file(&path);

        let listener = UnixListener::bind(&path)?;
        listener.set_nonblocking(true)?;

        Ok(Self {
            listener,
            clients: Vec::new(),
            path,
            sample_rate,
        })
    }

    fn accept_clients(&mut self) {
        loop {
            match self.listener.accept() {
                Ok((stream, _)) => {
                    let _ = stream.set_nonblocking(true);
                    self.clients.push(stream);
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(_) => break,
            }
        }
    }
}

impl VisualizerOutput for IpcOutput {
    fn render(&mut self, frame: &VisualizerFrame) {
        self.accept_clients();

        let packet = VisualizerPacket {
            version: 1,
            sample_rate: self.sample_rate,
            bands: &frame.bands,
            rms: frame.rms,
            peak: frame.peak,
        };

        let Ok(mut message) = to_string(&packet) else {
            return;
        };

        message.push('\n');

        self.clients
            .retain_mut(|client| client.write_all(message.as_bytes()).is_ok());
    }
}

impl Drop for IpcOutput {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
