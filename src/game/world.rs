use std::f32::consts;

use crate::utils::*;
use macroquad::math::*;
use terrain::*;

pub mod perlin;
pub mod terrain;

type TerrainIndex = (TerrainClass, usize);

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

    pub height_map: Vec<f32>,
    pub class_map:  Vec<TerrainIndex>,
    pub class:      WorldClass,
}

impl World {
    pub fn new(
        system: IVec2,
        position: I64Vec2,
        radius: f32,
        mass: f32,
        class: WorldClass,
        preset: Option<&[TerrainIndex]>,
    ) -> World {
        let circ = (radius / 1000.0 * consts::TAU).floor() as usize;

        let sections = preset
            .map(|s| s.to_vec())
            .unwrap_or_else(|| _gen_sections(circ, class));
        let mut height_map = gen_height_map(circ, &sections);
        let mut class_map = Vec::<TerrainIndex>::with_capacity(sections.len());

        // Sink oceans and smoothen terrain transitions
        let mut current_index = 0;
        sections.into_iter().for_each(|(class, length)| {
            if let TerrainClass::Ocean = class {
                let end = current_index + length;
                let factor = consts::PI / length as f32;
                let mut i = current_index;
                while i < end {
                    height_map[i] -= 4000.0 * ((i - current_index) as f32 * factor).sin();
                    i += 1;
                }
            }
            smooth_at(&mut height_map, current_index);
            current_index += length;
            class_map.push((class, current_index));
        });

        World {
            system,
            position,
            radius,
            mass,
            height_map,
            class_map,
            class,
        }
    }

    pub fn grativy_at(&self, point: I64Vec2) -> Vec2 {
        let r = to_meters(point - self.position);
        -r.normalize_or_zero() * GRAVITATION * self.mass / r.length_squared()
    }

    pub fn altitude_at(&self, point: I64Vec2) -> f32 {
        to_meters(point - self.position).length() - self.radius
    }

    pub fn terrain_index_beneath(&self, point: I64Vec2) -> usize {
        (self.radius / 1000.0 * to_angle(to_meters(point - self.position))) as usize
            % self.height_map.len()
    }

    pub fn terrain_class(&self, index: usize) -> TerrainClass {
        self.class_map
            .iter()
            .find(|(_, length)| index <= *length)
            .unwrap()
            .0
    }

    pub fn surface(&self, index: usize) -> I64Vec2 {
        to_i64coords(polar_to_cartesian(
            self.radius + self.height_map[index],
            index as f32 * 1000.0 / self.radius,
        )) + self.position
    }

    pub fn point_from_sealevel(&self, index: usize, raise: f32) -> I64Vec2 {
        to_i64coords(polar_to_cartesian(
            self.radius + raise,
            index as f32 * 1000.0 / self.radius,
        )) + self.position
    }
}
