mod audio;
mod config;
mod output;

use anyhow::Result;

use audio::{analyzer::Analyzer, pipewire::PipeWireCapture};

use config::Config;

use output::{ipc::IpcOutput, terminal::TerminalRenderer, VisualizerOutput};

fn main() -> Result<()> {
    let config = Config::default();

    let capture = PipeWireCapture::new()?;

    let mut analyzer = Analyzer::new(
        config.fft_size,
        config.band_count,
        config.sample_rate,
        config.attack,
        config.decay,
        config.sensitivity,
    );

    let mut terminal = TerminalRenderer::new(config.visualizer_height);

    let mut ipc = IpcOutput::new("/tmp/audio-visualizer.sock", config.sample_rate as u32)?;

    loop {
        let samples = capture.recv()?;

        if let Some(frame) = analyzer.process(&samples) {
            terminal.render(&frame);
            ipc.render(&frame);
        }
    }
}
