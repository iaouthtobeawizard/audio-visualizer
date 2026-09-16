pub mod ipc;
pub mod terminal;
use crate::audio::analyzer::VisualizerFrame;

pub trait VisualizerOutput {
    fn render(&mut self, frame: &VisualizerFrame);
}
