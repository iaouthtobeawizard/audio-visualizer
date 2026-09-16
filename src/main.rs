mod audio;
mod output;

use anyhow::Result;

use audio::{analyzer::Analyzer, pipewire::PipeWireCapture};

use output::{ipc::IpcOutput, terminal::TerminalRenderer, VisualizerOutput};

fn main() -> Result<()> {
    let capture = PipeWireCapture::new()?;

    let mut analyzer = Analyzer::new(1024, 24, 48_000.0);

    let mut terminal = TerminalRenderer::new(12);

    let mut ipc = IpcOutput::new("/tmp/audio-visualizer.sock", 48_000)?;

    loop {
        let samples = capture.recv()?;

        if let Some(frame) = analyzer.process(&samples) {
            terminal.render(&frame);
            ipc.render(&frame);
        }
    }
}
