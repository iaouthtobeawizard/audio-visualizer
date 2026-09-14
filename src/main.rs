mod audio;
mod output;

use anyhow::Result;

use audio::{analyzer::Analyzer, pipewire::PipeWireCapture};

use output::terminal::TerminalRenderer;

fn main() -> Result<()> {
    let capture = PipeWireCapture::new()?;

    let mut analyzer = Analyzer::new(1024, 24, 48_000.0);

    let renderer = TerminalRenderer::new(12);

    loop {
        let samples = capture.recv()?;

        if let Some(frame) = analyzer.process(&samples) {
            renderer.render(&frame);
        }
    }
}
