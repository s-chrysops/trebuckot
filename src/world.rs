use super::*;

const TERRAIN_DEPTH: f32 = 4000.0;

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
        terrain_data: Vec<f32>,
        class: WorldClass,
    ) -> Self {
        Self {
            system,
            position,
            radius,
            mass,
            terrain: Terrain::new(position, radius, terrain_data),
            class,
        }
    }
}

// Contains all the points of a World in i64 space
pub struct Terrain {
    pub circumference: usize,
    pub upper:         Vec<I64Vec2>,
    pub lower:         Vec<I64Vec2>,
}

impl Terrain {
    fn new(position: I64Vec2, radius: f32, terrain_data: Vec<f32>) -> Self {
        let mut upper = Vec::with_capacity(terrain_data.len());
        let mut lower = Vec::with_capacity(terrain_data.len());
        let mut i = 0.0;
        for terrain_height in terrain_data.iter() {
            upper.push(
                to_i64coords(polar_to_cartesian(radius + terrain_height, i / radius)) + position,
            );
            lower.push(
                to_i64coords(polar_to_cartesian(radius - TERRAIN_DEPTH, i / radius)) + position,
            );
            i += 1000.0;
        }
        Self {
            circumference: terrain_data.len(),
            upper,
            lower,
        }
    }
}

pub fn terrain_idx_beneath(game: &Game) -> usize {
    let slice = game.world.terrain.circumference as f32 / consts::TAU;
    let angle = to_angle(to_f32coords(
        game.render_space.position - game.world.position,
    ));
    (slice * angle % game.world.terrain.circumference as f32) as usize
}
