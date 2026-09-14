use super::{bands::FrequencyBands, fft::FftAnalyzer, smoothing::Smoother};

pub struct VisualizerFrame {
    pub bands: Vec<f32>,
    pub rms: f32,
    pub peak: f32,
    pub active: bool,
}

pub struct Analyzer {
    fft: FftAnalyzer,
    bands: FrequencyBands,
    smoother: Smoother,
    sample_rate: f32,
}

impl Analyzer {
    pub fn new(fft_size: usize, band_count: usize, sample_rate: f32) -> Self {
        Self {
            fft: FftAnalyzer::new(fft_size),
            bands: FrequencyBands::new(band_count, sample_rate, fft_size),
            smoother: Smoother::new(band_count, 0.45, 0.12),
            sample_rate,
        }
    }

    pub fn process(&mut self, samples: &[f32]) -> Option<VisualizerFrame> {
        self.fft.push(samples);

        let spectrum = self.fft.analyze()?;

        let bands = self.bands.analyze(spectrum, self.sample_rate);

        let bands = self.smoother.process(&bands);

        let rms = if samples.is_empty() {
            0.0
        } else {
            (samples.iter().map(|sample| sample * sample).sum::<f32>() / samples.len() as f32)
                .sqrt()
        };

        let peak = samples
            .iter()
            .map(|sample| sample.abs())
            .fold(0.0_f32, f32::max);

        let active = rms > 0.005 || peak > 0.02;

        Some(VisualizerFrame {
            bands: bands.to_vec(),
            rms,
            peak,
            active,
        })
    }
}
