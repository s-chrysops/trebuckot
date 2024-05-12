use super::*;
use ::glam::i64::I64Vec2;

const GRAVITATION: f32 = 6.6743e-11;
const TERRAIN_DEPTH: f32 = 1000.0;

pub fn to_coords64(f: Vec2) -> I64Vec2 {
    I64Vec2::new((f.x.round() * 256.0) as i64, (f.y.round() * 256.0) as i64)
}

pub struct System {
    position: IVec2,
    worlds: Vec<World>,
}

impl System {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: ivec2(x, y),
            worlds: Vec::new(),
        }
    }
}

enum WorldType {
    Minshara,
    Planet,
    Demon,
    Planetoid,
    Gas,
    Star,
}

pub struct World {
    position: I64Vec2, // 1/256 meters
    radius: f32,       // meters
    mass: f32,

    pub entities: Vec<Entity>,
    pub terrain: Terrain,
}

impl World {
    pub fn new(x: i64, y: i64, radius: f32, mass: f32, terrain_data: Vec<f32>) -> Self {
        // Initialize terrain
        let mut upper = Vec::new();
        let mut lower = Vec::new();
        let mut i = 0.0;
        for altitude in terrain_data.iter() {
            upper.push(to_coords64(polar_to_cartesian(
                radius + altitude,
                i / radius,
            )));
            lower.push(to_coords64(polar_to_cartesian(
                radius - TERRAIN_DEPTH,
                i / radius,
            )));
            i += 1000.0;
        }

        Self {
            position: I64Vec2::new(x, y),
            radius,
            mass,
            entities: Vec::new(),
            terrain: Terrain { upper, lower },
        }
    }
    pub fn pull_force(&self, m: f32, r: f32) -> f32 {
        GRAVITATION * self.mass * m / r.powi(2)
    }
}

pub struct Terrain {
    pub upper: Vec<I64Vec2>,
    pub lower: Vec<I64Vec2>,
}
