use crate::{game::*, utils::*, GameError};
use hud::draw_hud;
use macroquad::prelude::*;
use render_space::RenderSpace;
use trebuchet::draw_trebuchet;
use world::draw_world;

mod hud;
pub mod icon;
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

    font: Font,
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

        let render_space = RenderSpace::init();
        let font = load_ttf_font("VT323.ttf").await?;

        Ok(Render {
            camera,
            render_target,

            render_space,
            freecam_on: false,
            prev_screen: get_screen(),
            smooth_zoom,

            font,
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
        let freecam_speed = match is_key_down(KeyCode::LeftShift) {
            true => 256,
            false => 25600,
        };

        self.camera.zoom += (self.smooth_zoom - self.camera.zoom) * 0.1;

        let rel_pos = self.render_space.position - game.world.position;
        self.camera.rotation = 90.0 - to_angle(to_meters(rel_pos)).to_degrees();

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

        draw_world(&self.render_space, &game.world);
        draw_trebuchet(&self.render_space, &game.trebuchet);

        // Placeholder player
        let player_pos = self.render_space.to_screen(game.player.position);
        draw_circle(player_pos.x, player_pos.y, 0.08, PINK);

        // self.render_space.draw();

        // Draw render target to screen
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

        draw_hud(game, &self.font);
    }
}
