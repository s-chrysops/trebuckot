use crate::utils::*;
use macroquad::math::*;
use terrain::*;

pub mod perlin;
pub mod terrain;

const GRAVITATION: f32 = 6.6743e-11;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum WorldClass {
    Minshara,
    Desert,
    Demon,
    Moon,
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
        preset: Option<&[(TerrainClass, usize)]>,
    ) -> Self {
        Self {
            system,
            position,
            radius,
            mass,
            terrain: Terrain::new(radius, class, preset),
            class,
        }
    }

    pub fn get_grativy(&self, point: I64Vec2) -> Vec2 {
        let r = to_meters(point - self.position);
        -r.normalize_or_zero() * GRAVITATION * self.mass / r.length_squared()
    }

    pub fn get_altitude(&self, point: I64Vec2) -> f32 {
        to_meters(point - self.position).length() - self.radius
    }

    pub fn get_terrain_idx_beneath(&self, point: I64Vec2) -> usize {
        (self.radius / 1000.0 * to_angle(to_meters(point - self.position))
            % self.terrain.circ as f32) as usize
    }

    pub fn get_terrain_class(&self, index: usize) -> TerrainClass {
        self.terrain
            .class_map
            .iter()
            .find(|(_, length)| index <= *length)
            .unwrap()
            .0
    }

    pub fn get_terrain(&self, index: usize) -> I64Vec2 {
        to_i64coords(polar_to_cartesian(
            self.radius + self.terrain.height_map[index],
            index as f32 * 1000.0 / self.radius,
        )) + self.position
    }

    pub fn get_from_sealevel(&self, index: usize, raise: f32) -> I64Vec2 {
        to_i64coords(polar_to_cartesian(
            self.radius + raise,
            index as f32 * 1000.0 / self.radius,
        )) + self.position
    }
}
