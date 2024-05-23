use super::*;

const GRAVITATION: f32 = 6.6743e-11;

#[derive(Default)]
pub struct Player {
    pub position: I64Vec2,
    pub rotation: f32,

    pub mass:         f32,
    pub velocity:     Vec2,
    pub acceleration: Vec2,
    pub move_speed:   f32,
}

impl Player {
    pub fn new(position: I64Vec2) -> Self {
        Self {
            position,
            mass:         0.3,
            rotation:     0.0,
            velocity:     vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            move_speed:   1000.0,
        }
    }

    pub fn get_grativy(&self, world: &World) -> Vec2 {
        let r = to_f32coords(self.position - world.position);
        -r.normalize_or_zero() * GRAVITATION * world.mass / r.length_squared()
    }

    pub fn get_altitude(&self, world: &World) -> f32 {
        to_f32coords(self.position - world.position).length() - world.radius
    }
}
