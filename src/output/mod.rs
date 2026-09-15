pub mod terminal;

use crate::audio::analyzer::VisualizerFrame;

pub trait VisualizerOutput {
    fn render(&self, frame: &VisualizerFrame);
}
