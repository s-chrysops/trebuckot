use crate::{game::*, utils::*, GameError};
use hud::draw_hud;
use macroquad::prelude::*;
use player::draw_player;
use render_assets::RenderAssets;
use render_space::RenderSpace;
use trebuchet::draw_trebuchet;
use world::draw_world;

mod hud;
pub mod icon;
mod player;
mod render_assets;
mod render_space;
mod trebuchet;
mod world;

pub fn get_screen() -> Vec2 {
    vec2(screen_width(), screen_height())
}

pub struct Render {
    pub camera:    Camera2D,
    render_target: RenderTarget,

    pub render_space: RenderSpace,
    freecam_on:       bool,
    prev_screen:      Vec2,
    smooth_zoom:      Vec2,
    smooth_offset:    Vec2,

    assets: RenderAssets,
}

impl Render {
    pub async fn init() -> Result<Render, GameError> {
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Linear);

        let camera_rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
        let mut camera = Camera2D::from_display_rect(camera_rect);
        camera.render_target = Some(render_target.clone());
        camera.zoom *= 100.0;

        let smooth_zoom = camera.zoom;
        set_camera(&camera);

        Ok(Render {
            camera,
            render_target,

            render_space: RenderSpace::init(),
            freecam_on: false,
            prev_screen: get_screen(),
            smooth_zoom,
            smooth_offset: Vec2::ZERO,

            assets: RenderAssets::init().await?,
        })
    }

    pub fn update(&mut self, game: &Game) {
        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                self.smooth_zoom *= 10.0_f32.powf(y.signum() / 4.0);
                self.smooth_zoom = self.smooth_zoom.clamp(
                    vec2(
                        (screen_width() * 50.0).recip(),
                        -(screen_height() / 20000.0).recip(),
                    ),
                    vec2(
                        (screen_width() / 20000.0).recip(),
                        -(screen_height() * 50.0).recip(),
                    ),
                );
            }
            _ => (),
        }

        if is_key_pressed(KeyCode::Tab) {
            self.freecam_on ^= true;
        };

        let freecam_speed = 256
            * if is_key_down(KeyCode::LeftShift) {
                1
            } else {
                100
            };

        self.smooth_offset = match game.state {
            GameState::Paused => self.camera.offset,
            GameState::PreLaunch => vec2(0.0, 0.5),
            GameState::Launched => game.player.velocity.normalize() * vec2(-0.5, 0.5),
            GameState::Landed => vec2(0.0, 0.5),
        };

        self.camera.zoom += (self.smooth_zoom - self.camera.zoom) / 8.0;
        self.camera.offset += (self.smooth_offset - self.camera.offset) / 128.0;

        let rel_pos = self.render_space.position - game.world.position;
        self.camera.rotation = 90.0 - to_meters(rel_pos).to_angle().to_degrees();

        if self.freecam_on {
            if is_key_down(KeyCode::W) {
                self.render_space.position += rel_pos / 25600;
            }
            if is_key_down(KeyCode::S) {
                self.render_space.position -= rel_pos / 25600;
            }
            if is_key_down(KeyCode::A) {
                self.render_space.position += rel_pos.perp() / freecam_speed;
            }
            if is_key_down(KeyCode::D) {
                self.render_space.position -= rel_pos.perp() / freecam_speed;
            }
        } else {
            self.render_space.position = game.player.position;
        };

        // if is_key_down(KeyCode::Up) {
        //     self.camera.offset += Vec2::ZERO.with_y(0.01);
        // }
        // if is_key_down(KeyCode::Down) {
        //     self.camera.offset -= Vec2::ZERO.with_y(0.01);
        // }
        // if is_key_down(KeyCode::Left) {
        //     self.camera.offset += Vec2::ZERO.with_x(0.01);
        // }
        // if is_key_down(KeyCode::Right) {
        //     self.camera.offset -= Vec2::ZERO.with_x(0.01);
        // }

        if self.prev_screen != get_screen() {
            self.prev_screen = get_screen();
            // Reset camera
            self.render_target = render_target(screen_width() as u32, screen_height() as u32);
            self.render_target.texture.set_filter(FilterMode::Linear);

            let camera_rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
            self.camera = Camera2D::from_display_rect(camera_rect);
            self.camera.render_target = Some(self.render_target.clone());
            self.camera.zoom *= 100.0;
            self.smooth_zoom = self.camera.zoom;
        };
    }

    pub fn draw(&self, game: &Game) {
        set_camera(&self.camera);

        //Draw & Clear Background
        clear_background(SKYBLUE);

        draw_world(
            &self.render_space,
            &game.world,
            &self.assets.terrain_material,
        );
        draw_trebuchet(&self.render_space, &game.trebuchet, &self.assets);
        draw_player(&self.render_space, &game.player, &self.assets);

        // self.render_space.draw();

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &self.render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                flip_y: false,
                ..Default::default()
            },
        );

        draw_hud(game, &self.assets);
    }
}
