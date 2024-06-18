use core::f32::consts;
use macroquad::prelude::*;

// meters to i64 coordinates
pub fn to_i64coords(f32coords: Vec2) -> I64Vec2 {
    (f32coords * 256.0).as_i64vec2()
}

// i64 coordinates to meters
pub fn to_meters(i64coords: I64Vec2) -> Vec2 {
    i64coords.as_vec2() / 256.0
}

pub fn to_angle(coords: Vec2) -> f32 {
    let theta = coords.y.atan2(coords.x);
    theta.rem_euclid(consts::TAU)
}

use std::ops::{Add, Sub, Mul, Div};
#[allow(dead_code)]
pub fn rk4<T, U>(x: T, dt: f32, f: U) -> T
where
    T: Copy + Add<Output = T> + Div<f32, Output = T>,
    f32: Mul<T, Output = T>,
    U: Fn(f32, T) -> T,
{
    let k1 = dt * f(dt, x);
    let k2 = dt * f(dt, x + 0.5 * k1);
    let k3 = dt * f(dt, x + 0.5 * k1);
    let k4 = dt * f(dt, x + k3);
    x + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0 
}

pub fn rk5<T, U>(x: T, dt: f32, f: U) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Div<f32, Output = T>,
    f32: Mul<T, Output = T>,
    U: Fn(f32, T) -> T,
{
    let k1 = dt * f(dt, x);
    let k2 = dt * f(dt, x + (1.0 / 4.0) * k1);
    let k3 = dt * f(dt, x + (1.0 / 8.0) * k1 + (1.0 / 8.0) * k2);
    let k4 = dt * f(dt, x - (1.0 / 2.0) * k2 + k3);
    let k5 = dt * f(dt, x + (3.0 / 16.0) * k1 + (9.0 / 16.0) * k4);
    let k6 = dt * f(dt, x - (3.0 / 7.0) * k1 + (2.0 / 7.0) * k2 + (12.0 / 7.0) 
        * k3 - (12.0/ 7.0) * k4 + (8.0 / 7.0) * k5);
    x + (7.0 * k1 + 32.0 * k3 + 12.0 * k4 + 32.0 * k5 + 7.0 * k6) / 90.0 
}

