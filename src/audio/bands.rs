use rustfft::num_complex::Complex;

pub struct FrequencyBands {
    count: usize,
    min_frequency: f32,
    max_frequency: f32,
}

impl FrequencyBands {
    pub fn new(count: usize, sample_rate: f32, fft_size: usize) -> Self {
        let max_frequency = (sample_rate / 2.0).min(20_000.0);

        let max_frequency = max_frequency.max(20.0);

        let _ = fft_size;

        Self {
            count,
            min_frequency: 20.0,
            max_frequency,
        }
    }

    pub fn analyze(&self, spectrum: &[Complex<f32>], sample_rate: f32) -> Vec<f32> {
        let fft_size = spectrum.len();

        let mut bands = vec![0.0; self.count];

        for (bin, value) in spectrum.iter().enumerate().skip(1) {
            let frequency = bin as f32 * sample_rate / fft_size as f32;

            if frequency < self.min_frequency || frequency > self.max_frequency {
                continue;
            }

            let normalized = (frequency / self.min_frequency).ln()
                / (self.max_frequency / self.min_frequency).ln();

            let index = (normalized * self.count as f32) as usize;

            if index < self.count {
                bands[index] += value.norm();
            }
        }

        for band in &mut bands {
            *band = (*band).sqrt();
        }

        bands
    }
}
