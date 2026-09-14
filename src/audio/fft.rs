use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::Arc;

pub struct FftAnalyzer {
    size: usize,
    fft: Arc<dyn rustfft::Fft<f32>>,
    buffer: Vec<Complex<f32>>,
    window: Vec<f32>,
}

impl FftAnalyzer {
    pub fn new(size: usize) -> Self {
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(size);

        let window = (0..size)
            .map(|i| {
                let phase = 2.0 * std::f32::consts::PI * i as f32 / (size - 1) as f32;

                0.5 * (1.0 - phase.cos())
            })
            .collect();

        Self {
            size,
            fft,
            buffer: vec![Complex::new(0.0, 0.0); size],
            window,
        }
    }

    pub fn push(&mut self, samples: &[f32]) {
        for i in 0..self.size {
            let sample = samples.get(i).copied().unwrap_or(0.0);

            self.buffer[i] = Complex::new(sample * self.window[i], 0.0);
        }
    }

    pub fn ready(&self) -> bool {
        self.buffer.iter().any(|sample| sample.re != 0.0)
    }

    pub fn analyze(&mut self) -> Option<&[Complex<f32>]> {
        if !self.ready() {
            return None;
        }

        self.fft.process(&mut self.buffer);

        Some(&self.buffer)
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
