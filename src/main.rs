mod audio;

use anyhow::Result;

use audio::{analyzer::Analyzer, capture::AudioCapture};

fn main() -> Result<()> {
    println!("notch-visualizer");
    println!("Starting audio capture...");

    let capture = AudioCapture::new()?;

    let mut analyzer = Analyzer::new(1024, 24, capture.sample_rate());

    println!("Audio capture started @ {} Hz", capture.sample_rate());

    loop {
        let samples = capture.recv()?;

        if let Some(frame) = analyzer.process(&samples) {
            println!(
                "bands: {:02} | rms: {:.3} | peak: {:.3} | active: {}",
                frame.bands.len(),
                frame.rms,
                frame.peak,
                frame.active,
            );
        }
    }
}
