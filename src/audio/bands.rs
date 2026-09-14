use rustfft::num_complex::Complex;

pub struct FrequencyBands {
    count: usize,
    min_frequency: f32,
    max_frequency: f32,
}

impl FrequencyBands {
    pub fn new(count: usize, sample_rate: f32, _fft_size: usize) -> Self {
        let max_frequency = (sample_rate / 2.0).min(20_000.0).max(20.0);

        Self {
            count,
            min_frequency: 20.0,
            max_frequency,
        }
    }

    pub fn analyze(&self, spectrum: &[Complex<f32>], sample_rate: f32) -> Vec<f32> {
        let fft_size = spectrum.len();
        let mut bands = vec![0.0; self.count];
        let mut counts = vec![0usize; self.count];

        let log_range = (self.max_frequency / self.min_frequency).ln();

        for (bin, value) in spectrum.iter().enumerate().skip(1) {
            let frequency = bin as f32 * sample_rate / fft_size as f32;

            if frequency < self.min_frequency || frequency > self.max_frequency {
                continue;
            }

            let normalized = (frequency / self.min_frequency).ln() / log_range;

            let index = (normalized * self.count as f32) as usize;

            if index < self.count {
                bands[index] += value.norm_sqr();
                counts[index] += 1;
            }
        }

        for (band, count) in bands.iter_mut().zip(counts) {
            if count > 0 {
                *band = (*band / count as f32).sqrt();
            }
        }

        bands
    }
}
