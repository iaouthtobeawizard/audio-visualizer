use crate::audio::analyzer::VisualizerFrame;

pub struct TerminalRenderer {
    height: usize,
}

impl TerminalRenderer {
    pub fn new(height: usize) -> Self {
        Self { height }
    }

    pub fn render(&self, frame: &VisualizerFrame) {
        print!("\x1b[2J\x1b[H");

        println!("notch-visualizer");
        println!();

        for row in (0..self.height).rev() {
            for &band in &frame.bands {
                let level = (band * 8.0).clamp(0.0, self.height as f32) as usize;

                if level > row {
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
