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
        render_target.texture.set_filter(FilterMode::Nearest);

        let camera_rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
        let mut camera = Camera2D::from_display_rect(camera_rect);
        camera.render_target = Some(render_target.clone());
        camera.zoom *= 100.0;
        camera.offset = vec2(0.0, 0.5);

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
        if game.state != GameState::Paused {
            match mouse_wheel() {
                (_x, y) if y != 0.0 => {
                    let min_zoom = get_screen() / 2.0e-2;
                    let max_zoom = get_screen() / 2.0e+4;
                    self.smooth_zoom *= 10.0_f32.powf(y.signum() / 4.0);
                    self.smooth_zoom = self.smooth_zoom.clamp(
                        vec2(min_zoom.x.recip(), -max_zoom.y.recip()),
                        vec2(max_zoom.x.recip(), -min_zoom.y.recip()),
                    );
                }
                _ => (),
            }
        }

        if is_key_pressed(KeyCode::Tab) {
            self.freecam_on ^= true;
        };

        self.smooth_offset = match (self.freecam_on, &game.state) {
            (true, _) => Vec2::ZERO,
            (false, GameState::Paused) => self.camera.offset,
            (false, GameState::PreLaunch) => vec2(0.0, 0.5),
            (false, GameState::Launched) => game.player.velocity.normalize() * vec2(-0.5, 0.5),
            (false, GameState::Landed) => vec2(0.0, 0.5),
        };

        self.camera.zoom += (self.smooth_zoom - self.camera.zoom) / 8.0;
        self.camera.offset += (self.smooth_offset - self.camera.offset) / 128.0;
        self.camera.rotation = -to_meters(game.world.position - self.render_space.position)
            .perp()
            .to_angle()
            .to_degrees();

        if self.freecam_on {
            let mut freecam_pos =
                cartesian_to_polar(to_meters(self.render_space.position - game.world.position));

            let fast = match is_key_down(KeyCode::LeftShift) {
                true => 1000.0,
                false => 1.0,
            };

            if is_key_down(KeyCode::W) {
                freecam_pos.x += 1.0 * fast;
            }
            if is_key_down(KeyCode::S) {
                freecam_pos.x -= 1.0 * fast;
            }
            if is_key_down(KeyCode::A) {
                freecam_pos.y += 1.0e-6 * fast;
            }
            if is_key_down(KeyCode::D) {
                freecam_pos.y -= 1.0e-6 * fast;
            }

            self.render_space.position =
                to_i64coords(polar_to_cartesian(freecam_pos.x, freecam_pos.y))
                    + game.world.position;
        } else {
            self.render_space.position = game.player.position;
        };

        if self.prev_screen != get_screen() {
            self.prev_screen = get_screen();
            // Reset camera
            self.render_target = render_target(screen_width() as u32, screen_height() as u32);
            self.render_target.texture.set_filter(FilterMode::Nearest);

            let camera_rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
            self.camera = Camera2D::from_display_rect(camera_rect);
            self.camera.render_target = Some(self.render_target.clone());
            self.camera.zoom *= 100.0;
            self.camera.offset = vec2(0.0, 0.5);
            self.smooth_zoom = self.camera.zoom;
        };
    }

    pub fn draw(&self, game: &Game) {
        set_camera(&self.camera);

        //Draw & Clear Background
        clear_background(SKYBLUE);

        draw_player(&self.render_space, &game.player, &self.assets);
        draw_world(
            &self.render_space,
            &game.world,
            &self.assets.terrain_material,
        );
        draw_trebuchet(&self.render_space, &game.trebuchet, &self.assets);

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
