use crate::game::*;
use macroquad::math::*;
use std::f32::consts;

use super::perlin::PerlinNoise;
use super::WorldClass;

const AMPL_PLAIN: f32 = 500.0;
const AMPL_HILLS: f32 = 1000.0;
const AMPL_ROCKY: f32 = 2000.0;

const FREQ_PLAIN: f32 = 0.001;
const FREQ_HILLS: f32 = 0.01;
const FREQ_ROCKY: f32 = 0.1;

#[derive(Debug, Clone, Copy)]
pub enum TerrainClass {
    Plain(usize),
    Hills(usize),
    Desert(usize),
    Rocky(usize),
    Ocean(usize),
}

// Contains all the points of a World in i64 space
pub struct Terrain {
    pub circ:    usize,
    pub surface: Vec<I64Vec2>,
}

impl Terrain {
    pub fn new(
        position: I64Vec2,
        radius: f32,
        class: WorldClass,
        preset: Option<&[TerrainClass]>,
    ) -> Self {
        let circ = (radius / 1000.0 * consts::TAU).floor() as usize;

        let height_map = match preset {
            Some(sections) => generate_height_map(circ, class, sections),
            None => vec![0.0; circ], // TODO replace with terrain section generator
        };

        let surface: Vec<I64Vec2> = height_map
            .iter()
            .enumerate()
            .map(|(i, height)| {
                to_i64coords(polar_to_cartesian(
                    radius + height,
                    i as f32 * 1000.0 / radius,
                )) + position
            })
            .collect();

        Self { circ, surface }
    }
}

fn generate_height_map(circ: usize, _class: WorldClass, sections: &[TerrainClass]) -> Vec<f32> {
    let ami_cute = u64::from_le_bytes("ami cute".as_bytes().try_into().unwrap());
    let noise = PerlinNoise::new(ami_cute, circ);

    let mut current_idx: usize = 0;
    let mut transition_points: Vec<usize> = Vec::with_capacity(sections.len());
    let mut noise_parameters: Vec<Vec<(f32, f32)>> = Vec::with_capacity(sections.len());

    sections.iter().for_each(|section| {
        match section {
            TerrainClass::Plain(length) => {
                current_idx += length;
                noise_parameters.push(vec![(AMPL_PLAIN, FREQ_PLAIN); *length])
            }
            TerrainClass::Hills(length) => {
                current_idx += length;
                noise_parameters.push(vec![(AMPL_HILLS, FREQ_HILLS); *length])
            }
            TerrainClass::Desert(length) => {
                current_idx += length;
                noise_parameters.push(vec![(AMPL_HILLS, FREQ_PLAIN); *length])
            }
            TerrainClass::Rocky(length) => {
                current_idx += length;
                noise_parameters.push(vec![(AMPL_ROCKY, FREQ_ROCKY); *length])
            }
            TerrainClass::Ocean(length) => {
                current_idx += length;
                noise_parameters.push(vec![(-AMPL_HILLS, FREQ_ROCKY); *length])
            }
        }
        transition_points.push(current_idx % circ);
    });

    let mut height_map: Vec<f32> = noise_parameters
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, (ampl, freq))| noise.get(i as f32, *ampl, *freq) + ampl * 2.0)
        .collect();

    transition_points.iter().for_each(|p| {
        smooth_at(&mut height_map, *p);
    });

    height_map
}

const SMOOTH_LENGTH: usize = 100;
const FACTOR: f32 = consts::PI / SMOOTH_LENGTH as f32;
fn smooth_at(array: &mut [f32], idx: usize) {
    let len = array.len();
    let prev_idx = (idx + len - 1) % len;
    let mid = (array[idx] - array[prev_idx]) / 2.0;

    let mut i = (idx + len - SMOOTH_LENGTH / 2) % len;
    (0..SMOOTH_LENGTH).for_each(|j| {
        let weight = (j as f32 * FACTOR).cos();
        array[i % len] -= mid * (weight - weight.signum());
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
