use core::f32::consts;
use macroquad::prelude::*;

pub mod units {
    pub type Kilometers = usize;
    pub type Meters = f32;

    pub type Kilograms = f32;
}

pub use units::*;

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

pub trait Vec2Ext {
    fn to_i64coords(self) -> I64Vec2;
    fn to_angle_tau(self) -> f32;
}

impl Vec2Ext for Vec2 {
    fn to_i64coords(self) -> I64Vec2 {
        (self * 256.0).as_i64vec2()
    }
    fn to_angle_tau(self) -> f32 {
        let theta = self.y.atan2(self.x);
        theta.rem_euclid(consts::TAU)
    }
}

pub trait I64Vec2Ext {
    fn to_meters(self) -> Vec2;
}

impl I64Vec2Ext for I64Vec2 {
    fn to_meters(self) -> Vec2 {
        self.as_vec2() / 256.0
    }
}

use std::ops::{Add, Div, Mul, Sub};
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
    let k2 = dt * f(dt, x + k1 / 4.0);
    let k3 = dt * f(dt, x + (k1 + k2) / 8.0);
    let k4 = dt * f(dt, x - (k2 / 2.0) + k3);
    let k5 = dt * f(dt, x + (3.0 * k1 + 9.0 * k4) / 16.0);
    let k6 = dt
        * f(
            dt,
            x - (3.0 * k1 + 2.0 * k2 + 12.0 * k3 - 12.0 * k4 + 8.0 * k5) / 7.0,
        );
    x + (7.0 * k1 + 32.0 * k3 + 12.0 * k4 + 32.0 * k5 + 7.0 * k6) / 90.0
}

// | |a-b| a-b |
// | |c-d| c-d |
// -------------
//  | a-b c-d |
pub fn get_intersection(a: I64Vec2, b: I64Vec2, c: I64Vec2, d: I64Vec2) -> Option<I64Vec2> {
    let xdiff = I64Vec2::new(a.x - b.x, c.x - d.x).as_dvec2();
    let ydiff = I64Vec2::new(a.y - b.y, c.y - d.y).as_dvec2();
    let div = xdiff.perp_dot(ydiff);
    if div == 0.0 {
        return None;
    }
    let dets = I64Vec2::new(a.perp_dot(b), c.perp_dot(d)).as_dvec2();
    let x = dets.perp_dot(xdiff) / div;
    let y = dets.perp_dot(ydiff) / div;
    Some(I64Vec2::new(x.round() as i64, y.round() as i64))
}
