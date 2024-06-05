use macroquad::math::{I64Vec2, Vec2};

mod upgrades;
pub use upgrades::*;

//#[derive(Default)]
pub struct Player {
    pub position: I64Vec2,
    pub rotation: f32,

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

            rotation: 0.0,
            velocity: Vec2::default(),
            acceleration: Vec2::default(),

            gun: None,
            melee: None,

            coating: None,
            brakes: None,
            thrusters: None,
            stablizer: None,

            move_speed: 1000.0,
        }
    }
}
