use super::*;
use ::glam::i64::I64Vec2;

pub struct Entity {
    pub position: I64Vec2,
    rotation: f32,
    velocity: Vec2,
}

impl Entity {
    pub fn new(x: i64, y: i64, rotation: f32, velocity: Vec2) -> Self {
        let mut instance = Self {
            position: I64Vec2 { x, y },
            rotation,
            velocity,
        };
        instance
    }
}
