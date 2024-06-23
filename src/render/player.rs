use super::{render_assets::RenderAssets, render_space::RenderSpace};
use crate::player::Player;
use macroquad::prelude::*;
use std::f32::consts;

pub fn draw_player(render_space: &RenderSpace, player: &Player, assets: &RenderAssets) {
    let player_pos = render_space.to_screen(player.position);
    let texture = assets.get("plushie_test");
    let params = DrawTextureParams {
        dest_size: Some(vec2(0.4, 0.4)),
        rotation: player.rotation + consts::FRAC_PI_2,
        pivot: Some(player_pos),
        ..Default::default()
    };
    draw_texture_ex(
        texture,
        player_pos.x - 0.2,
        player_pos.y - 0.12,
        WHITE,
        params,
    );
    // draw_circle_lines(player_pos.x, player_pos.y, 0.08, 0.01, PINK);
}
