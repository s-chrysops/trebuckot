use crate::{game::Game, get_screen, render::Render};
use macroquad::prelude::*;

pub struct DevInfo {
    avg_fps:     f32,
    avg_frame:   f32,
    sample_size: usize,

    fps_samples:   Vec<i32>,
    frame_samples: Vec<f32>,
}

impl DevInfo {
    pub fn init() -> Self {
        let sample_size = 60;
        let fps_samples: Vec<i32> = Vec::with_capacity(sample_size);
        let frame_samples: Vec<f32> = Vec::with_capacity(sample_size);

        Self {
            avg_fps: 0.0,
            avg_frame: 0.0,
            sample_size,
            fps_samples,
            frame_samples,
        }
    }

    pub fn draw(&mut self, game: &Game, render: &Render) {
        self.fps_samples.push(get_fps());
        self.frame_samples.push(get_frame_time());
        if self.fps_samples.len() == self.sample_size {
            self.avg_fps = self.fps_samples.iter().sum::<i32>() as f32 / self.sample_size as f32;
            self.avg_frame = self.frame_samples.iter().sum::<f32>() / self.sample_size as f32;
            self.fps_samples.clear();
            self.frame_samples.clear();
        }

        // Get world position from mouse
        let cursor = render
            .camera
            .screen_to_world(vec2(mouse_position().0, mouse_position().1));

        let dev_info = [
            format!(
                "average FPS: {:.2} ({:.2} ms)",
                self.avg_fps,
                self.avg_frame * 1000.0
            ),
            format!(
                "mouse position (pixels) = ({:+.2}, {:+.2})",
                cursor.x, cursor.y
            ),
            format!("screen size: {:?}", get_screen().to_string()),
            format!(
                "camera zoom = {:.1} ({:.2},{:.2})",
                (screen_width() * render.camera.zoom.x).recip() * 2000.0,
                render.camera.zoom.x.recip(),
                render.camera.zoom.y.recip(),
            ),
            format!(
                "player position (meters) = ({:+.2}, {:+.2})",
                game.player.position.x / 256,
                game.player.position.y / 256,
            ),
            format!(
                "player altitude (meters) = {:+.2}",
                game.world.get_altitude(game.player.position)
            ),
            format!(
                "player velocity (m/s)= {:+.2} ({:+.2},{:+.2})",
                game.player.velocity.length(),
                game.player.velocity.x,
                game.player.velocity.y
            ),
            format!(
                "render space position: {:+.2}, {:+.2}",
                render.render_space.position.x as f32 / 256.0,
                render.render_space.position.y as f32 / 256.0,
            ),
            format!("closest terrain index: {}", game.world.get_terrain_idx_beneath(render.render_space.position)),
            format!("launch time: {:.2}", game.stats.time),
            format!("   distance: {:.2}", game.stats.distance),
            format!("max_altitude {:.2}", game.stats.max_altitude),
            format!("  max_speed: {:.2}", game.stats.max_speed),
            format!("gravity: {}", game.world.get_grativy(game.player.position).length())
        ];

        let mut spacing = screen_height() + 10.0 - 20.0 * dev_info.len() as f32;
        for line in dev_info.iter() {
            draw_text(line.as_str(), 10.0, spacing, 20.0, BLACK);
            spacing += 20.0
        }
    }
}
