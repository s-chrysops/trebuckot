use macroquad::math::*;
use std::f32::consts;

use super::perlin::PerlinNoise;
use super::WorldClass;

use crate::utils::units::*;

pub type TerrainSection = (TerrainClass, usize);

const AMPL_PLAIN: Meters = 500.0;
const AMPL_HILLS: Meters = 1000.0;
const AMPL_ROCKY: Meters = 2000.0;

const FREQ_PLAIN: f32 = 0.001;
const FREQ_HILLS: f32 = 0.01;
const FREQ_ROCKY: f32 = 0.1;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TerrainClass {
    Plain,
    Hills,
    Sands,
    Rocky,
    Ocean,
}

pub fn gen_height_map(circ: Kilometers, sections: &[TerrainSection], scale: f32) -> Vec<Meters> {
    let ami_cute = u64::from_le_bytes("ami cute".as_bytes().try_into().unwrap());
    let noise = PerlinNoise::new(ami_cute, circ);

    sections
        .iter()
        .flat_map(|(class, length)| {
            let noise_params = match class {
                TerrainClass::Plain => (AMPL_PLAIN * scale, FREQ_PLAIN),
                TerrainClass::Hills => (AMPL_HILLS * scale, FREQ_HILLS),
                TerrainClass::Sands => (AMPL_HILLS * scale, FREQ_PLAIN),
                TerrainClass::Rocky => (AMPL_ROCKY * scale, FREQ_ROCKY),
                TerrainClass::Ocean => (-AMPL_HILLS * scale, FREQ_ROCKY),
            };
            vec![noise_params; *length]
        })
        .enumerate()
        .map(|(i, (ampl, freq))| noise.get(i as f32, ampl, freq) + ampl)
        .collect()
}

pub fn gen_sections(circ: Kilometers, _class: WorldClass) -> Vec<TerrainSection> {
    vec![(TerrainClass::Plain, circ)]
}

const SMOOTH_LENGTH: Kilometers = 20;
const FACTOR: f32 = consts::PI / SMOOTH_LENGTH as f32;
pub fn smooth_at(height_map: &mut [Meters], index: Kilometers) {
    let circ = height_map.len();
    let avg_height = {
        let prev_index = (index + circ - 1) % circ;
        (height_map[index] - height_map[prev_index]) / 2.0
    };

    let mut i = (index + circ - SMOOTH_LENGTH / 2) % circ;
    (0..SMOOTH_LENGTH).for_each(|j| {
        let weight = (j as f32 * FACTOR).cos();
        height_map[i % circ] -= avg_height * (weight - weight.signum());
        i += 1;
    });
}

#[cfg(test)]
mod terrain_tests {
    use super::smooth_at;

    #[test]
    fn smooth() {
        let a: f32 = 50.0;
        let b: f32 = -8.0;
        let mut v: Vec<f32> = [[a; 60], [b; 60]].into_iter().flatten().collect();
        smooth_at(&mut v, 0);
        v.iter()
            .enumerate()
            .for_each(|(i, x)| println!("{i:2}:{x}"));
    }
}
