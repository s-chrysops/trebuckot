use crate::game::*;
use macroquad::math::*;
use std::f32::consts;

use super::perlin::PerlinNoise;
use super::WorldClass;

const AMPL_PLAIN: f64 = 1000.0;
const AMPL_HILLS: f64 = 2000.0;
const AMPL_ROCKY: f64 = 4000.0;

const FREQ_PLAIN: f64 = 0.001;
const FREQ_HILLS: f64 = 0.01;
const FREQ_ROCKY: f64 = 0.5;

#[derive(Clone, Copy)]
pub enum TerrainClass {
    Plain,
    Hills,
    Rocky,
    Ocean,
    Desert,
}

// Contains all the points of a World in i64 space
pub struct Terrain {
    pub circ:    usize,
    pub surface: Vec<I64Vec2>,
}

impl Terrain {
    pub fn new(position: I64Vec2, radius: f32, class: &WorldClass) -> Self {
        let circ = (radius / 1000.0 * consts::TAU).floor() as usize;
        let ami_cute = u64::from_le_bytes("ami cute".as_bytes().try_into().unwrap());
        let noise = PerlinNoise::new(ami_cute, circ);

        let terrain: Vec<(usize, f32)> = (0..circ)
            .map(|i| match class {
                WorldClass::Minshara(terra) => {
                    let mut ampl: f64 = 1.0;
                    let mut freq: f64 = 1.0;
                    if let Some(t) = terra {
                        match t.iter().find(|(p, _)| i < *p).unwrap().1 {
                            TerrainClass::Plain => {
                                ampl = AMPL_PLAIN;
                                freq = FREQ_PLAIN;
                            }
                            TerrainClass::Hills => {
                                ampl = AMPL_HILLS;
                                freq = FREQ_HILLS;
                            }
                            TerrainClass::Rocky => {
                                ampl = AMPL_ROCKY;
                                freq = FREQ_ROCKY;
                            }
                            TerrainClass::Ocean => {
                                ampl = AMPL_HILLS;
                                freq = FREQ_ROCKY;
                            }
                            TerrainClass::Desert => {
                                ampl = AMPL_HILLS;
                                freq = FREQ_PLAIN;
                            }
                        };
                    };
                    noise.get(i as f64, ampl, freq, 4)
                },
                WorldClass::Desert => noise.get(i as f64, AMPL_PLAIN, FREQ_HILLS, 4),
                WorldClass::Demon => noise.get(i as f64, AMPL_HILLS, FREQ_ROCKY, 4),
                WorldClass::Moon => noise.get(i as f64, AMPL_PLAIN, FREQ_ROCKY, 4),
                _ => 0.0,
            })
            .enumerate()
            .collect();

        let surface: Vec<I64Vec2> = terrain
            .iter()
            .map(|(i, height)| {
                to_i64coords(polar_to_cartesian(
                    radius + height,
                    *i as f32 * 1000.0 / radius,
                )) + position
            })
            .collect();

        Self { circ, surface }
    }
}
