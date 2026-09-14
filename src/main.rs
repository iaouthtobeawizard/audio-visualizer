mod audio;
mod output;
use anyhow::Result;

use audio::{analyzer::Analyzer, capture::AudioCapture};
use output::terminal::TerminalRenderer;
fn main() -> Result<()> {
    println!("notch-visualizer");
    println!("Starting audio capture...");

    let capture = AudioCapture::new()?;

    let mut analyzer = Analyzer::new(1024, 24, capture.sample_rate());
    let renderer = TerminalRenderer::new(12);
    println!("Audio capture started @ {} Hz", capture.sample_rate());

    loop {
        let samples = capture.recv()?;

        if let Some(frame) = analyzer.process(&samples) {
            renderer.render(&frame);
        }
    }
}
