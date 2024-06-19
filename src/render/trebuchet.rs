use super::{render_assets::RenderAssets, render_space::RenderSpace};
use crate::trebuchet::Trebuchet;
use macroquad::prelude::*;

pub fn draw_trebuchet(render_space: &RenderSpace, trebuchet: &Trebuchet, assets: &RenderAssets) {
    if !render_space.within(trebuchet.position) {
        return;
    }

    let base = render_space.to_screen(trebuchet.position);
    let pivot = vec2(base.x, base.y + trebuchet.height);
    let arm_s = trebuchet.armsling_point() + pivot;
    let arm_w = trebuchet.armweight_point() + pivot;
    let s = trebuchet.sling_point() + pivot;
    let w = trebuchet.weight_point() + pivot;

    draw_line(base.x, base.y, pivot.x, pivot.y, 0.1, BROWN);
    draw_line(arm_s.x, arm_s.y, arm_w.x, arm_w.y, 0.1, YELLOW);
    draw_line(s.x, s.y, arm_s.x, arm_s.y, 0.01, GRAY);
    // draw_line(w.x, w.y, arm_w.x, arm_w.y, 0.1, BLACK);

    let weight_texture = assets.get(trebuchet.weight.texture());
    let weight_params = DrawTextureParams {
        dest_size: Some(Vec2::splat(arm_w.distance(w))),
        rotation: (arm_w - w).perp().to_angle(),
        pivot: Some(arm_w),
        ..Default::default()
    };
    draw_texture_ex(
        weight_texture,
        arm_w.x - arm_w.distance(w) / 2.0,
        arm_w.y,
        WHITE,
        weight_params,
    )

    // let p = self.v_projectile() + s;
    // draw_line(s.x, s.y, p.x, p.y, 0.05, PINK);
}
