#[derive(Clone)]
pub struct Config {
    pub fft_size: usize,
    pub band_count: usize,
    pub sample_rate: f32,
    pub visualizer_height: usize,
    pub sensitivity: f32,
    pub attack: f32,
    pub decay: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fft_size: 1024,
            band_count: 24,
            sample_rate: 48_000.0,
            visualizer_height: 12,
            sensitivity: 3.0,
            attack: 0.45,
            decay: 0.12,
        }
    }
}
