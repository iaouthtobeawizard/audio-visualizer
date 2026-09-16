use super::{bands::FrequencyBands, fft::FftAnalyzer, smoothing::Smoother};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct VisualizerFrame {
    pub bands: Vec<f32>,
    pub rms: f32,
    pub peak: f32,
}

pub struct Analyzer {
    fft: FftAnalyzer,
    bands: FrequencyBands,
    smoother: Smoother,
    sample_rate: f32,
    mono: Vec<f32>,
}

impl Analyzer {
    pub fn new(fft_size: usize, band_count: usize, sample_rate: f32) -> Self {
        Self {
            fft: FftAnalyzer::new(fft_size),
            bands: FrequencyBands::new(band_count, sample_rate, fft_size),
            smoother: Smoother::new(band_count, 0.45, 0.12),
            sample_rate,
            mono: Vec::with_capacity(fft_size),
        }
    }

    pub fn process(&mut self, samples: &[f32]) -> Option<VisualizerFrame> {
        if samples.len() < 2 {
            return None;
        }

        self.mono.clear();

        for chunk in samples.chunks_exact(2) {
            self.mono.push((chunk[0] + chunk[1]) * 0.5);
        }

        let rms = (self.mono.iter().map(|sample| sample * sample).sum::<f32>()
            / self.mono.len() as f32)
            .sqrt();

        let peak = self
            .mono
            .iter()
            .map(|sample| sample.abs())
            .fold(0.0_f32, f32::max);

        self.fft.push(&self.mono);

        let spectrum = self.fft.analyze();

        let bands = self.bands.analyze(spectrum, self.sample_rate);

        let bands = self.smoother.process(&bands).to_vec();

        Some(VisualizerFrame { bands, rms, peak })
    }
}
