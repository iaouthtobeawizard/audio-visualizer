use crate::audio::analyzer::VisualizerFrame;

use super::VisualizerOutput;

pub struct TerminalRenderer {
    height: usize,
    scale: f32,
}

impl TerminalRenderer {
    pub fn new(height: usize) -> Self {
        Self { height, scale: 0.5 }
    }
}

impl VisualizerOutput for TerminalRenderer {
    fn render(&mut self, frame: &VisualizerFrame) {
        print!("\x1b[2J\x1b[H");

        for row in (0..self.height).rev() {
            for &band in &frame.bands {
                let level = (band * self.scale).clamp(0.0, self.height as f32);

                let visible = if row == 0 { true } else { level > row as f32 };

                if visible {
                    print!("█ ");
                } else {
                    print!("  ");
                }
            }

            println!();
        }
    }
}
