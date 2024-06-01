use macroquad::rand::*;

pub struct PerlinNoise {
    noise: Vec<f64>,
}

impl PerlinNoise {
    pub fn new(seed: u64, length: usize) -> Self {
        srand(seed);
        let norm = (u32::MAX / 2) as f64;
        let noise: Vec<f64> = (0..length).map(|_| rand() as f64 / norm - 1.0).collect();
        Self { noise }
    }

    pub fn get(&self, x: f64, mut ampl: f64, mut freq: f64, octa: u8) -> f32 {
        let mut result: f64 = 0.0;
        for _ in 0..octa {
            result += self.interpolate(x * freq) * ampl;
            freq *= 2.0;
            ampl /= 2.0;
        }
        result as f32
    }

    fn interpolate(&self, x: f64) -> f64 {
        let prev_x = x.floor() as usize;
        let next_x = prev_x + 1;
        let a = self.noise[prev_x % self.noise.len()];
        let b = self.noise[next_x % self.noise.len()];
        let mut frac_x = x - x.floor();
        // fade
        frac_x = (6.0 * frac_x.powi(5)) - (15.0 * frac_x.powi(4)) + (10.0 * frac_x.powi(3));
        a + frac_x * (b - a)
    }
}
