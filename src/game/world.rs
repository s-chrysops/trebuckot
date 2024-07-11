use std::f32::consts;

use crate::utils::*;
use macroquad::math::*;
use terrain::*;

pub mod perlin;
pub mod terrain;

const GRAVITATION: f32 = 6.6743e-11;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorldClass {
    Minshara,
    Desert,
    Demon,
    Gas,
}

pub struct World {
    // pub system:   IVec2,   // lightyears
    pub position: I64Vec2, // 1/256 meters
    pub radius:   Meters,
    pub mass:     Kilograms,

    pub height_map: Vec<Meters>,
    pub class_map:  Vec<(TerrainClass, usize)>,
    pub class:      WorldClass,
}

impl World {
    pub fn new(
        scale: f32,
        // system: IVec2,
        position: I64Vec2,
        radius: Meters,
        mass: Kilograms,
        class: WorldClass,
        preset: Option<&[TerrainSection]>,
    ) -> World {
        let circ = (radius * scale / 1000.0 * consts::TAU).floor() as Kilometers;
        let inv_scale = (1.0 / scale) as usize; // to avoid the cast hell multiplying usize with scale

        let sections: Vec<TerrainSection> = match preset {
            Some(p) => p.iter().map(|(c, l)| (*c, l / inv_scale)).collect(),
            None => gen_sections(circ, class),
        };

        let mut height_map = gen_height_map(circ, &sections, scale);
        let class_map: Vec<(TerrainClass, usize)> = sections
            .iter()
            .scan(0, |current_index, (class, length)| {
                *current_index += length;
                Some((*class, *current_index))
            })
            .collect();

        // Sink oceans and smoothen terrain transitions
        let mut current_index = 0;
        for (class, length) in sections.into_iter() {
            if let TerrainClass::Ocean = class {
                let factor = consts::PI / length as f32;
                let max_depth = 4000.0 * scale;
                for (i, height) in height_map
                    .iter_mut()
                    .skip(current_index)
                    .take(length)
                    .enumerate()
                {
                    *height -= max_depth * (factor * i as f32).sin();
                }
            }

            smooth_at(&mut height_map, current_index);

            current_index += length;
        }

        World {
            // system,
            position,
            radius: radius * scale,
            mass: mass * scale.powi(2),
            height_map,
            class_map,
            class,
        }
    }

    pub fn grativy_at(&self, point: I64Vec2) -> Vec2 {
        let r = (point - self.position).to_meters();
        -r.normalize_or_zero() * GRAVITATION * self.mass / r.length_squared()
    }

    pub fn altitude_at(&self, point: I64Vec2) -> Meters {
        (point - self.position).to_meters().length() - self.radius
    }

    pub fn terrain_index_beneath(&self, point: I64Vec2) -> Kilometers {
        (self.radius / 1000.0 * (point - self.position).to_meters().to_angle_tau()) as Kilometers
            % self.height_map.len()
    }

    pub fn terrain_class(&self, index: Kilometers) -> TerrainClass {
        self.class_map
            .iter()
            .find(|(_, length)| index <= *length)
            .unwrap()
            .0
    }

    pub fn surface(&self, index: Kilometers) -> I64Vec2 {
        polar_to_cartesian(
            self.radius + self.height_map[index],
            index as f32 * 1000.0 / self.radius,
        )
        .to_i64coords()
            + self.position
    }

    pub fn point_from_sealevel(&self, index: Kilometers, raise: Meters) -> I64Vec2 {
        (polar_to_cartesian(self.radius + raise, index as f32 * 1000.0 / self.radius))
            .to_i64coords()
            + self.position
    }
}
