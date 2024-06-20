use macroquad::math::*;
use std::f32::consts;

use super::perlin::PerlinNoise;
use super::WorldClass;

type Kilometers = usize;
type Meters = f32;
type TerrainIndex = (TerrainClass, usize);

const AMPL_PLAIN: f32 = 500.0;
const AMPL_HILLS: f32 = 1000.0;
const AMPL_ROCKY: f32 = 2000.0;

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

pub fn gen_height_map(circ: Kilometers, sections: &[TerrainIndex]) -> Vec<Meters> {
    let ami_cute = u64::from_le_bytes("ami cute".as_bytes().try_into().unwrap());
    let noise = PerlinNoise::new(ami_cute, circ);

    sections
        .iter()
        .flat_map(|(class, length)| match class {
            TerrainClass::Plain => vec![(AMPL_PLAIN, FREQ_PLAIN); *length],
            TerrainClass::Hills => vec![(AMPL_HILLS, FREQ_HILLS); *length],
            TerrainClass::Sands => vec![(AMPL_HILLS, FREQ_PLAIN); *length],
            TerrainClass::Rocky => vec![(AMPL_ROCKY, FREQ_ROCKY); *length],
            TerrainClass::Ocean => vec![(-AMPL_HILLS, FREQ_ROCKY); *length],
        })
        .enumerate()
        .map(|(i, (ampl, freq))| noise.get(i as f32, ampl, freq) + ampl)
        .collect()
}

pub fn _gen_sections(_circ: Kilometers, _class: WorldClass) -> Vec<TerrainIndex> {
    todo!()
}

const SMOOTH_LENGTH: Kilometers = 30;
const FACTOR: f32 = consts::PI / SMOOTH_LENGTH as f32;
pub fn smooth_at(array: &mut [Meters], index: Kilometers) {
    let len = array.len();
    let prev_index = (index + len - 1) % len;
    let avg = (array[index] - array[prev_index]) / 2.0;

    let mut i = (index + len - SMOOTH_LENGTH / 2) % len;
    (0..SMOOTH_LENGTH).for_each(|j| {
        let weight = (j as f32 * FACTOR).cos();
        array[i % len] -= avg * (weight - weight.signum());
        i += 1;
    });
}

/*
Trudging through the stormy Alaskan rainforest, fighting demigods popping out at every corner,
I find myself depleted from life's basic need... reliable internet. But, in the scortching
ice lands of this temporal void, with scars that run deeper than the Mariana, one thought
powers me to prevail through this herculean challenge with the strength of a dozen Roman legions...
ami cute
*/
#[cfg(test)]
mod tests {
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
