use crate::{trebuchet::Trebuchet, utils::*, world::World, Game, GameState};
use macroquad::prelude::*;

const TERRAIN_DEPTH: f32 = 100_000.0;
const VIEW_RADIUS: f32 = 100000.0; // meters

pub fn get_screen() -> Vec2 {
    vec2(screen_width(), screen_height())
}

#[derive(Default)]
pub struct RenderSpace {
    pub position: I64Vec2,
    radius:       f32,
}

impl RenderSpace {
    pub fn init() -> Self {
        Self {
            position: I64Vec2::ZERO,
            radius:   VIEW_RADIUS * 256.0,
        }
    }

    pub fn within(&self, point: I64Vec2) -> bool {
        (point - self.position).as_vec2().length() < self.radius
    }

    pub fn to_screen(&self, point: I64Vec2) -> Vec2 {
        to_meters(point - self.position) + get_screen() / 2.0
    }

    pub fn draw(&self) {
        draw_circle_lines(
            screen_width() / 2.0,
            screen_height() / 2.0,
            self.radius / 256.0,
            50.0,
            RED,
        );
    }
}

pub struct Render {
    pub camera:        Camera2D,
    pub render_target: RenderTarget,

    pub render_space: RenderSpace,
    freecam_on:       bool,
    prev_screen:      Vec2,
    smooth_zoom:      Vec2,

    font: Font,
}

impl Render {
    pub async fn init() -> Result<Render, macroquad::Error> {
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Linear);

        let camera_rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
        let mut camera = Camera2D::from_display_rect(camera_rect);
        camera.render_target = Some(render_target.clone());
        camera.zoom *= 100.0;

        let smooth_zoom = camera.zoom;
        set_camera(&camera);

        let font = load_ttf_font("VT323.ttf").await?;

        let render_space = RenderSpace::init();

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
                )
            }
            _ => (),
        }
        if is_key_pressed(KeyCode::Tab) {
            self.freecam_on = !self.freecam_on;
        }
        let freecam_speed = if is_key_down(KeyCode::LeftShift) {
            256
        } else {
            25600
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
        }

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
        }
    }

    pub fn draw(&self, game: &Game) {
        set_camera(&self.camera);

        //Draw & Clear Background
        clear_background(SKYBLUE);

        self.draw_world(&game.world);
        self.draw_trebuchet(&game.trebuchet);

        // Placeholder player
        let player_pos = self.render_space.to_screen(game.player.position);
        draw_circle(player_pos.x, player_pos.y, 0.08, PINK);

        self.render_space.draw();

        // Draw to screen
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

        // HUD
        match game.state {
            GameState::PreLaunch => {}
            GameState::Launched => {}
            GameState::Landed => {
                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    color_u8!(0, 0, 0, 64),
                );
                let margin_x = screen_width() * 0.25;
                let margin_y = screen_height() * 0.25;
                let spacing = 60.0;
                let params = TextParams {
                    font: Some(&self.font),
                    font_size: 48,
                    color: WHITE,
                    ..Default::default()
                };

                for (i, stat) in game.stats.as_vec().iter().enumerate() {
                    let stat_fmt = format!("{:.2}{}", stat.value, stat.unit);
                    let stat_width = measure_text(&stat_fmt, Some(&self.font), 48, 1.0).width;
                    draw_text_ex(
                        &stat.field,
                        margin_x,
                        (spacing * i as f32) + margin_y,
                        params.clone(),
                    );
                    draw_text_ex(
                        &stat_fmt,
                        screen_width() - margin_x - stat_width,
                        (spacing * i as f32) + margin_y,
                        params.clone(),
                    );
                }

                let re = "Research Earned";
                let points = game.stats.crunch().to_string();
                let re_width = measure_text(re, Some(&self.font), 48, 1.0).width;
                let points_width = measure_text(&points, Some(&self.font), 48, 1.0).width;
                draw_text_ex(
                    "Research Earned",
                    (screen_width() - re_width) / 2.0,
                    screen_height() - margin_y - spacing,
                    params.clone(),
                );
                draw_text_ex(
                    &points,
                    (screen_width() - points_width) / 2.0,
                    screen_height() - margin_y,
                    params.clone(),
                );
            }
            GameState::Paused => {
                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    color_u8!(0, 0, 0, 64),
                );
            }
        }
    }

    fn draw_world(&self, world: &World) {
        let surface = &world.terrain.surface;
        let circ = world.terrain.circ;
        let radius_bot = world.radius - TERRAIN_DEPTH;
        let terrain_idx = world.get_terrain_idx_beneath(self.render_space.position);

        let l_scan = surface
            .iter()
            .cycle()
            .skip(terrain_idx)
            .position(|p| !self.render_space.within(*p))
            .unwrap();
        let r_scan = surface
            .iter()
            .rev()
            .cycle()
            .skip(circ - terrain_idx)
            .position(|p| !self.render_space.within(*p))
            .unwrap();
        let l_bound = (l_scan + terrain_idx) % circ;
        let r_bound = (circ + terrain_idx - r_scan) % circ;

        let active: Vec<usize> = if r_bound > l_bound {
            (r_bound..circ).chain(0..l_bound).collect()
        } else {
            (r_bound..l_bound).collect()
        };

        active.into_iter().for_each(|current_idx| {
            let next_idx = (current_idx + 1) % circ;
            let u1 = world.terrain.surface[current_idx];
            let u2 = world.terrain.surface[next_idx];

            let l1 = to_i64coords(polar_to_cartesian(
                radius_bot,
                current_idx as f32 * 1000.0 / world.radius,
            )) + world.position;
            let l2 = to_i64coords(polar_to_cartesian(
                radius_bot,
                next_idx as f32 * 1000.0 / world.radius,
            )) + world.position;

            let s1 = to_i64coords(polar_to_cartesian(
                world.radius,
                current_idx as f32 * 1000.0 / world.radius,
            )) + world.position;
            let s2 = to_i64coords(polar_to_cartesian(
                world.radius,
                next_idx as f32 * 1000.0 / world.radius,
            )) + world.position;

            // Draw water
            draw_triangle(
                self.render_space.to_screen(s1),
                self.render_space.to_screen(s2),
                self.render_space.to_screen(l1),
                BLUE,
            );
            draw_triangle(
                self.render_space.to_screen(l1),
                self.render_space.to_screen(l2),
                self.render_space.to_screen(s2),
                DARKBLUE,
            );

            // Draw terrain
            draw_triangle(
                self.render_space.to_screen(u1),
                self.render_space.to_screen(u2),
                self.render_space.to_screen(l1),
                GREEN,
            );
            draw_triangle(
                self.render_space.to_screen(l1),
                self.render_space.to_screen(l2),
                self.render_space.to_screen(u2),
                DARKGREEN,
            );
        });
    }

    fn draw_trebuchet(&self, trebuchet: &Trebuchet) {
        let base = self.render_space.to_screen(trebuchet.position);
        let pivot = vec2(base.x, base.y + trebuchet.height);
        let arm_s = trebuchet.armsling_point() + pivot;
        let arm_w = trebuchet.armweight_point() + pivot;
        let s = trebuchet.sling_point() + pivot;
        let w = trebuchet.weight_point() + pivot;

        draw_line(base.x, base.y, pivot.x, pivot.y, 0.1, BROWN);
        draw_line(arm_s.x, arm_s.y, arm_w.x, arm_w.y, 0.1, YELLOW);
        draw_line(s.x, s.y, arm_s.x, arm_s.y, 0.01, GRAY);
        draw_line(w.x, w.y, arm_w.x, arm_w.y, 0.1, BLACK);

        // let p = self.v_projectile() + s;
        // draw_line(s.x, s.y, p.x, p.y, 0.05, PINK);
    }
}
