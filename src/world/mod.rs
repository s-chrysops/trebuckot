use crate::game::*;
use glam::I64Vec2;
use macroquad::math::*;
use terrain::*;

pub mod perlin;
mod terrain;

const GRAVITATION: f32 = 6.6743e-11;

#[allow(dead_code)]
pub enum WorldClass {
    Minshara,
    Planet,
    Demon,
    Planetoid,
    Gas,
    Star,
}

pub struct World {
    pub system:   IVec2,   // lightyears
    pub position: I64Vec2, // 1/256 meters
    pub radius:   f32,     // meters
    pub mass:     f32,     // kilograms

    pub terrain: Terrain,
    pub class:   WorldClass,
}

impl World {
    pub fn new(
        system: IVec2,
        position: I64Vec2,
        radius: f32,
        mass: f32,
        class: WorldClass,
    ) -> Self {
        Self {
            system,
            position,
            radius,
            mass,
            terrain: Terrain::new(position, radius),
            class,
        }
    }

    pub fn get_grativy(&self, point: I64Vec2) -> Vec2 {
        let r = to_f32coords(point - self.position);
        -r.normalize_or_zero() * GRAVITATION * self.mass / r.length_squared()
    }

    pub fn get_altitude(&self, point: I64Vec2) -> f32 {
        to_f32coords(point - self.position).length() - self.radius
    }

    pub fn get_terrain_idx_beneath(&self, point: I64Vec2) -> usize {
        (self.radius / 1000.0 * to_angle(to_f32coords(point - self.position))
            % self.terrain.circ as f32) as usize
    }
}
