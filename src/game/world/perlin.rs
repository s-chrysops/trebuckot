use macroquad::rand::*;

const OCTAVES: i32 = 4;

pub struct PerlinNoise {
    noise: Vec<f32>,
}

impl PerlinNoise {
    pub fn new(seed: u64, length: usize) -> Self {
        srand(seed);
        let mut noise: Vec<f32> = Vec::with_capacity(length);
        (0..length).for_each(|_| noise.push(gen_range(-1.0, 1.0)));
        Self { noise }
    }

    pub fn get(&self, x: f32, ampl: f32, freq: f32) -> f32 {
        (0..OCTAVES).fold(0.0, |result, octave| {
            result + self.interpolate(x * freq * 2.0_f32.powi(octave)) * ampl / 2.0_f32.powi(octave)
        })
    }

    fn interpolate(&self, x: f32) -> f32 {
        let prev_x = x.floor() as usize;
        let next_x = prev_x + 1;
        let a = self.noise[prev_x % self.noise.len()];
        let b = self.noise[next_x % self.noise.len()];
        let mut frac_x = x.fract();
        // fade
        frac_x = (6.0 * frac_x.powi(5)) - (15.0 * frac_x.powi(4)) + (10.0 * frac_x.powi(3));
        a + frac_x * (b - a)
    }
}
