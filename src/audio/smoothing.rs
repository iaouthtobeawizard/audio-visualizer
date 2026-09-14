pub struct Smoother {
    values: Vec<f32>,
    attack: f32,
    decay: f32,
}

impl Smoother {
    pub fn new(count: usize, attack: f32, decay: f32) -> Self {
        Self {
            values: vec![0.0; count],
            attack,
            decay,
        }
    }

    pub fn process(&mut self, input: &[f32]) -> &[f32] {
        for (current, &target) in self.values.iter_mut().zip(input) {
            let coefficient = if target > *current {
                self.attack
            } else {
                self.decay
            };

            *current += (target - *current) * coefficient;
        }

        &self.values
    }

    pub fn values(&self) -> &[f32] {
        &self.values
    }
}
