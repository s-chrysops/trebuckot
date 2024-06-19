use crate::{get_screen, utils::*};
use macroquad::prelude::*;

const VIEW_RADIUS: f32 = 100_000.0; // meters

#[derive(Default)]
pub struct RenderSpace {
    pub position: I64Vec2,
    pub radius:   f32,
}

impl RenderSpace {
    pub fn init() -> Self {
        Self {
            position: I64Vec2::ZERO,
            radius:   VIEW_RADIUS,
        }
    }

    pub fn within(&self, point: I64Vec2) -> bool {
        to_meters(point - self.position).length() < self.radius
    }

    pub fn to_screen(&self, point: I64Vec2) -> Vec2 {
        to_meters(point - self.position) + get_screen() / 2.0
    }

    // pub fn draw(&self) {
    //     draw_circle_lines(
    //         screen_width() / 2.0,
    //         screen_height() / 2.0,
    //         self.radius,
    //         50.0,
    //         RED,
    //     );
    // }
}
