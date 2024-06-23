use super::{render_assets::RenderAssets, render_space::RenderSpace};
use crate::trebuchet::Trebuchet;
use macroquad::prelude::*;

pub fn draw_trebuchet(render_space: &RenderSpace, trebuchet: &Trebuchet, assets: &RenderAssets) {
    if !render_space.within(trebuchet.position) {
        return;
    }

    let base = render_space.to_screen(trebuchet.position);
    let a_pivot = vec2(base.x, base.y + trebuchet.height);
    let w_position = trebuchet.weight_point() + a_pivot;
    let w_pivot = trebuchet.armweight_point() + a_pivot;
    let s_position = trebuchet.sling_point() + a_pivot;
    let s_pivot = trebuchet.armsling_point() + a_pivot;

    draw_line(
        s_position.x,
        s_position.y,
        s_pivot.x,
        s_pivot.y,
        0.005,
        GRAY,
    );

    {
        let arm_texture = assets.get(&trebuchet.arm.texture());
        // Texture is extended by 1/16 for portion past weight pivot
        let arm_size = Vec2::splat(trebuchet.arm.total_length()) * vec2(17.0 / 16.0, 1.0 / 8.0);
        let arm_params = DrawTextureParams {
            dest_size: Some(arm_size),
            rotation: (w_pivot - s_pivot).to_angle(),
            pivot: Some(a_pivot),
            ..Default::default()
        };
        draw_texture_ex(
            arm_texture,
            a_pivot.x - a_pivot.distance(s_pivot),
            a_pivot.y - arm_size.y / 2.0,
            WHITE,
            arm_params,
        );
        // draw_line(s_pivot.x, s_pivot.y, w_pivot.x, w_pivot.y, 0.005, YELLOW);
    }

    {
        let weight_texture = assets.get(&trebuchet.weight.texture());
        // Texture assumed square and extends at all sides by half
        let weight_size = Vec2::splat(trebuchet.weight.length) * vec2(2.0, 2.0);
        let weight_params = DrawTextureParams {
            dest_size: Some(weight_size),
            rotation: (w_pivot - w_position).perp().to_angle(),
            pivot: Some(w_pivot),
            flip_x: true,
            ..Default::default()
        };
        draw_texture_ex(
            weight_texture,
            w_pivot.x - weight_size.x / 2.0,
            w_pivot.y - weight_size.y / 4.0,
            WHITE,
            weight_params,
        );
        // draw_line(
        //     w_position.x,
        //     w_position.y,
        //     w_pivot.x,
        //     w_pivot.y,
        //     0.005,
        //     BLACK,
        // );
    }

    {
        let base_texture = assets.get(&trebuchet.texture());
        // Texture extends both sides vertically by 1/8
        let base_size = Vec2::splat(trebuchet.height) * vec2(2.0, 5.0 / 4.0);
        let base_params = DrawTextureParams {
            dest_size: Some(base_size),
            rotation: (a_pivot - base).perp().to_angle(),
            flip_x: true,
            ..Default::default()
        };
        draw_texture_ex(
            base_texture,
            base.x - base_size.x / 2.0,
            base.y - base_size.y / 10.0,
            WHITE,
            base_params,
        );
        // draw_line(base.x, base.y, a_pivot.x, a_pivot.y, 0.005, BROWN);
    }
    
    // let p = self.v_projectile() + s;
    // draw_line(s.x, s.y, p.x, p.y, 0.05, PINK);
}
