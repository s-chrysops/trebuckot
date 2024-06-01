use std::f32::consts;

use crate::game::*;
use glam::I64Vec2;
use macroquad::math::*;

use super::perlin::PerlinNoise;

const TERRAIN_DEPTH: f32 = 100_000.0;

// Contains all the points of a World in i64 space
pub struct Terrain {
    pub circ:  usize,
    pub upper: Vec<I64Vec2>,
    pub lower: Vec<I64Vec2>,
    pub sea:   Vec<I64Vec2>,
}

impl Terrain {
    pub fn new(position: I64Vec2, radius: f32) -> Self {
        let circ = (radius / 1000.0 * consts::TAU).floor() as usize;
        let rad_bot = radius - TERRAIN_DEPTH;
        let noise = PerlinNoise::new(0, circ);

        let terrain: Vec<(usize, f32)> = (0..circ)
            .map(|i| noise.get(i as f64, 4000.0, 0.01, 4))
            .enumerate()
            .collect();
        
        let upper: Vec<I64Vec2> = terrain
            .iter()
            .map(|(i, height)| {
                to_i64coords(polar_to_cartesian(
                    radius + height,
                    *i as f32 * 1000.0 / radius,
                )) + position
            })
            .collect();

        let lower: Vec<I64Vec2> = (0..circ)
            .map(|i| {
                to_i64coords(polar_to_cartesian(rad_bot, i as f32 * 1000.0 / radius)) + position
            })
            .collect();

        let sea: Vec<I64Vec2> = (0..circ)
            .map(|i| {
                to_i64coords(polar_to_cartesian(radius, i as f32 * 1000.0 / radius)) + position
            })
            .collect();

        Self {
            circ,
            upper,
            lower,
            sea,
        }
    }
}
