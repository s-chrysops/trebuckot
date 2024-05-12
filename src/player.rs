use super::*;

pub struct Player {
    pub entity: Entity,
    pub move_speed: i64,
}

impl Player {
    pub fn new() -> Self {
        let mut entity = Entity::new(0, 6_366_200, 0.0, vec2(0.0, 0.0));
        Self { entity, move_speed: 256}
    }
}
