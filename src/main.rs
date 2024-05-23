use ::glam::i64::I64Vec2;
use core::f32::consts;
use macroquad::prelude::*;

mod game;
pub use game::*;

mod trebuchet;
pub use trebuchet::*;

mod world;
pub use world::*;

mod player;
pub use player::*;

mod physics;
pub use physics::*;

mod draw;
pub use draw::*;

const GAME_SIZE_X: f32 = 1280.0;
const GAME_SIZE_Y: f32 = 720.0;
const PHYSICS_TICK: f32 = 0.001;

#[macroquad::main("Trebuckot")]
async fn main() {
    let mut game = Game::init().await;

    let mut avg_fps = 0.0;
    let mut avg_frame = 0.0;
    let mut fps_samples: Vec<i32> = Vec::new();
    let mut frame_samples: Vec<f32> = Vec::new();
    let sample_size = 60;

    loop {
        game.run();

        fps_samples.push(get_fps());
        frame_samples.push(get_frame_time());
        if fps_samples.len() == sample_size {
            avg_fps = fps_samples.iter().sum::<i32>() as f32 / sample_size as f32;
            avg_frame = frame_samples.iter().sum::<f32>() / sample_size as f32;
            fps_samples.clear();
            frame_samples.clear();
        }

        // Get world position from mouse
        let cursor = game
            .camera
            .screen_to_world(vec2(mouse_position().0, mouse_position().1));
        let v_proj = game.trebuchet.v_projectile();

        let dev_info = [
            format!("average FPS: {:.3} ({:.3} ms)", avg_fps, avg_frame * 1000.0),
            format!(
                "mouse position (pixels) = ({:+.2}, {:+.2})",
                cursor.x, cursor.y
            ),
            format!(
                "camera zoom = {:.1} ({},{})",
                2000.0 / (GAME_SIZE_X * game.camera.zoom.x),
                1.0 / game.camera.zoom.x,
                1.0 / game.camera.zoom.y
            ),
            format!(
                "player position (meters) = ({:+.2}, {:+.2})",
                game.player.position.x / 256,
                game.player.position.y / 256,
            ),
            format!(
                "player altitude (meters) = {:+.2}",
                game.player.get_altitude(&game.world)
            ),
            format!(
                "player velocity (m/s)= {:+.2} ({:+.2},{:+.2})",
                game.player.velocity.length(),
                game.player.velocity.x,
                game.player.velocity.y
            ),
            format!("launch time: {}", game.time_launch),
            format!("projectile velocity {}, {}", v_proj.x, v_proj.y),
            format!("projectile angle {}", to_angle(v_proj)),
            format!("projectile magnitude {}", v_proj.length()),
            format!("{:?}", get_screen()),
        ];

        let mut spacing = 15.0;
        for line in dev_info.iter() {
            draw_text(line.as_str(), 10.0, spacing, 20.0, BLACK);
            spacing += 20.0
        }

        next_frame().await;
    }
}
