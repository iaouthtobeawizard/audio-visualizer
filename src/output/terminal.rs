use crate::audio::analyzer::VisualizerFrame;

use super::VisualizerOutput;

pub struct TerminalRenderer {
    height: usize,
}

impl TerminalRenderer {
    pub fn new(height: usize) -> Self {
        Self { height }
    }
}

impl VisualizerOutput for TerminalRenderer {
    fn render(&self, frame: &VisualizerFrame) {
        print!("\x1b[2J\x1b[H");

        println!("audio-visualizer");
        println!();

        let max_band = frame.bands.iter().copied().fold(0.0_f32, f32::max);

        let scale = if max_band > 0.00001 {
            1.0 / max_band
        } else {
            0.0
        };

        for row in (0..self.height).rev() {
            for &band in &frame.bands {
                let level = band * scale * self.height as f32;

                if level > row as f32 {
                    print!("█ ");
                } else {
                    print!("  ");
                }
            }

            println!();
        }

        println!();
        println!(
            "RMS: {:.3} | Peak: {:.3} | Active: {}",
            frame.rms, frame.peak, frame.active
        );
    }
}
