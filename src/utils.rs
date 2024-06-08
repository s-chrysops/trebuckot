use core::f32::consts;
use macroquad::prelude::*;

// meters to i64 coordinates
pub fn to_i64coords(f32coords: Vec2) -> I64Vec2 {
    i64vec2(
        (f32coords.x * 256.0).round() as i64,
        (f32coords.y * 256.0).round() as i64,
    )
}

// i64 coordinates to meters
pub fn to_meters(i64coords: I64Vec2) -> Vec2 {
    vec2((i64coords.x as f32) / 256.0, (i64coords.y as f32) / 256.0)
}

pub fn to_angle(coords: Vec2) -> f32 {
    let theta = coords.y.atan2(coords.x);
    theta.rem_euclid(consts::TAU)
}