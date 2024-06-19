use super::upgrades::*;
use macroquad::math::{I64Vec2, Vec2};

#[derive(Default)]
pub struct Player {
    pub position:     I64Vec2,
    pub rotation:     f32,
    pub ang_velocity: f32,

    pub mass:         f32,
    pub velocity:     Vec2,
    pub acceleration: Vec2,

    pub gun:   Option<Gun>,
    pub melee: Option<Melee>,

    pub coating:   Option<Coating>,
    pub brakes:    Option<Brakes>,
    pub thrusters: Option<Thruster>,
    pub stablizer: Option<Stabilizer>,

    pub move_speed: f32,
}

impl Player {
    pub fn new(position: I64Vec2) -> Self {
        Self {
            position,
            mass: 0.3,
            move_speed: 1000.0,
            ..Default::default()
        }
    }
}
