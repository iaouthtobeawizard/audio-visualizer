use rustfft::num_complex::Complex;

pub struct FrequencyBands {
    count: usize,
    min_frequency: f32,
    max_frequency: f32,
    gain: f32,
}

impl FrequencyBands {
    pub fn new(count: usize, sample_rate: f32, _fft_size: usize, gain: f32) -> Self {
        Self {
            count,
            min_frequency: 20.0,
            max_frequency: (sample_rate / 2.0).min(20_000.0),
            gain,
        }
    }

    pub fn analyze(&self, spectrum: &[Complex<f32>], sample_rate: f32) -> Vec<f32> {
        let mut bands = vec![0.0; self.count];

        if spectrum.is_empty() {
            return bands;
        }

        let bin_count = spectrum.len() / 2;

        let min_log = self.min_frequency.ln();
        let max_log = self.max_frequency.ln();

        for bin in 1..bin_count {
            let frequency = bin as f32 * sample_rate / spectrum.len() as f32;

            if frequency < self.min_frequency || frequency > self.max_frequency {
                continue;
            }

            let position = (frequency.ln() - min_log) / (max_log - min_log);

            let index = (position * self.count as f32) as usize;

            if index >= self.count {
                continue;
            }

            bands[index] += spectrum[bin].norm();
        }

        for band in &mut bands {
            *band = (*band * self.gain).sqrt();
        }

        bands
    }
}
