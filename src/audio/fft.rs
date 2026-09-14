use rustfft::{num_complex::Complex, FftPlanner};
use std::collections::VecDeque;
use std::sync::Arc;

pub struct FftAnalyzer {
    size: usize,
    fft: Arc<dyn rustfft::Fft<f32>>,
    buffer: Vec<Complex<f32>>,
    samples: VecDeque<f32>,
    window: Vec<f32>,
}

impl FftAnalyzer {
    pub fn new(size: usize) -> Self {
        let mut planner = FftPlanner::<f32>::new();

        let fft = planner.plan_fft_forward(size);

        let window = (0..size)
            .map(|i| {
                let n = i as f32;
                let size = size as f32;

                0.5 * (1.0 - (2.0 * std::f32::consts::PI * n / (size - 1.0)).cos())
            })
            .collect();

        Self {
            size,
            fft,
            buffer: vec![Complex::new(0.0, 0.0); size],
            samples: VecDeque::with_capacity(size * 2),
            window,
        }
    }

    pub fn push(&mut self, samples: &[f32]) {
        self.samples.extend(samples);

        // Prevent unbounded growth if processing falls behind.
        while self.samples.len() > self.size * 4 {
            self.samples.pop_front();
        }
    }

    pub fn ready(&self) -> bool {
        self.samples.len() >= self.size
    }

    pub fn analyze(&mut self) -> Option<&[Complex<f32>]> {
        if !self.ready() {
            return None;
        }

        for i in 0..self.size {
            let sample = self.samples[i];

            self.buffer[i] = Complex::new(sample * self.window[i], 0.0);
        }

        // 50% overlap between FFT frames.
        let hop = self.size / 2;

        for _ in 0..hop {
            self.samples.pop_front();
        }

        self.fft.process(&mut self.buffer);

        Some(&self.buffer)
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
