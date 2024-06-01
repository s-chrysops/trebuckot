use crate::{get_screen, to_angle, to_f32coords, Game};
use ::glam::I64Vec2;
use macroquad::prelude::*;

const VIEW_RADIUS: f32 = 100000.0; // meters

#[derive(Default)]
pub struct RenderSpace {
    pub position: I64Vec2,
    radius:       f32,
}

impl RenderSpace {
    pub fn init() -> Self {
        Self {
            position: I64Vec2::default(),
            radius:   VIEW_RADIUS * 256.0,
        }
    }

    pub fn within(&self, point: I64Vec2) -> bool {
        (self.position.distance_squared(point) as f32).sqrt() < self.radius
    }

    pub fn to_screen(&self, point: I64Vec2) -> Vec2 {
        to_f32coords(point - self.position) + get_screen() / 2.0
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
    pub render_space:  RenderSpace,

    prev_screen: Vec2,
    smooth_zoom: Vec2,
}

impl Render {
    pub async fn init() -> Self {
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Linear);

        let camera_rect = Rect::new(0.0, 0.0, screen_width(), screen_height());
        let mut camera = Camera2D::from_display_rect(camera_rect);
        camera.render_target = Some(render_target.clone());
        camera.zoom *= 100.0;

        let smooth_zoom = camera.zoom;
        set_camera(&camera);

        let render_space = RenderSpace::init();
        Self {
            camera,
            render_target,
            render_space,
            prev_screen: get_screen(),
            smooth_zoom,
        }
    }

    pub fn update(&mut self, game: &Game) {
        match mouse_wheel() {
            (_x, y) if y != 0.0 => {
                self.smooth_zoom *= 10.0_f32.powf(y.signum() / 4.0);
            }
            _ => (),
        }
        self.camera.zoom += (self.smooth_zoom - self.camera.zoom) * 0.1;
        self.camera.rotation =
            90.0 - to_angle(to_f32coords(game.player.position - game.world.position)).to_degrees();
        self.render_space.position = game.player.position;

        // Reset camera at resize
        if self.prev_screen != get_screen() {
            self.prev_screen = get_screen();
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

        // Render Terrain
        {
            let circ = game.world.terrain.circ;
            let terrain_idx = game
                .world
                .get_terrain_idx_beneath(self.render_space.position);

            let surface = &game.world.terrain.upper;
            let l_bound = (surface
                .iter()
                .cycle()
                .skip(terrain_idx)
                .position(|p| !self.render_space.within(*p))
                .unwrap()
                + terrain_idx)
                % circ;
            let r_bound = (circ + terrain_idx
                - surface
                    .iter()
                    .rev()
                    .cycle()
                    .skip(circ - terrain_idx)
                    .position(|p| !self.render_space.within(*p))
                    .unwrap())
                % circ;

            let mut active: Vec<usize> = (r_bound..l_bound).collect();
            if active.is_empty() {
                active = (r_bound..circ).chain(0..l_bound).collect();
            }

            // how to double your lines of code with iters
            // and slow down the game to 6fps apparently

            // let mut active = vec![terrain_idx];
            // let mut i = (terrain_idx + 1) % circ;
            // while self.render_space.within(game.world.terrain.upper[i]) {
            //     active.push(i);
            //     i = (i + 1) % circ;
            // }
            // i = (terrain_idx + circ - 1) % circ;
            // while self.render_space.within(game.world.terrain.upper[i]) {
            //     active.push(i);
            //     i = (i + circ - 1) % circ;
            // }

            active.into_iter().for_each(|point_idx| {
                let u1 = game.world.terrain.upper[point_idx];
                let l1 = game.world.terrain.lower[point_idx];
                let u2 = game.world.terrain.upper[(point_idx + 1) % circ];
                let l2 = game.world.terrain.lower[(point_idx + 1) % circ];
                let s1 = game.world.terrain.sea[point_idx];
                let s2 = game.world.terrain.sea[(point_idx + 1) % circ];

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
        };

        // Draw Trebuchet
        {
            let base = self.render_space.to_screen(game.trebuchet.position);
            let pivot = vec2(base.x, base.y + game.trebuchet.height);
            let arm_s = game.trebuchet.armsling_point() + pivot;
            let arm_w = game.trebuchet.armweight_point() + pivot;
            let s = game.trebuchet.sling_point() + pivot;
            let w = game.trebuchet.weight_point() + pivot;

            draw_line(base.x, base.y, pivot.x, pivot.y, 0.1, BROWN);
            draw_line(arm_s.x, arm_s.y, arm_w.x, arm_w.y, 0.1, YELLOW);
            draw_line(s.x, s.y, arm_s.x, arm_s.y, 0.01, GRAY);
            draw_line(w.x, w.y, arm_w.x, arm_w.y, 0.1, BLACK);

            // let p = self.v_projectile() + s;
            // draw_line(s.x, s.y, p.x, p.y, 0.05, PINK);
        };

        // Placeholder player
        let player_pos = self.render_space.to_screen(game.player.position);
        draw_circle(player_pos.x, player_pos.y, 0.08, PINK);

        self.render_space.draw();

        // Set Default Camera
        set_default_camera();

        // Draw Game on Screen
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
    }
}
