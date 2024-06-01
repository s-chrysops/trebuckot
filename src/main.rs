use macroquad::prelude::*;
use game::*;
use render::*;
use ui::*;

mod game;
mod physics;
mod player;
mod render;
mod resources;
mod trebuchet;
mod ui;
mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: "Trebuckot".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1280,
        window_height: 720,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            framebuffer_alpha: false,
            swap_interval: None,
            ..Default::default()
        },
        icon: Some(ui::icon::set()),
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let mut game = Game::init().await;
    let mut render = Render::init().await;
    let mut ui = UI::init();

    let mut avg_fps = 0.0;
    let mut avg_frame = 0.0;
    let sample_size = 60;
    let mut fps_samples: Vec<i32> = Vec::with_capacity(sample_size);
    let mut frame_samples: Vec<f32> = Vec::with_capacity(sample_size);

    loop {
        match game.state {
            GameState::MainMenu => {
                ui.main_menu(&mut game);
            }
            GameState::Paused => {
                ui.pause_menu(&mut game);
            }
            GameState::Landed => {
                ui.landed_screen(&mut game);
            }
            _ => {
                let ami = 1337;
                let cute = 1337;
                assert_eq!(ami, cute);
            }
        }
        game.update();
        render.update(&game);
        render.draw(&game);

        fps_samples.push(get_fps());
        frame_samples.push(get_frame_time());
        if fps_samples.len() == sample_size {
            avg_fps = fps_samples.iter().sum::<i32>() as f32 / sample_size as f32;
            avg_frame = frame_samples.iter().sum::<f32>() / sample_size as f32;
            fps_samples.clear();
            frame_samples.clear();
        }

        // Get world position from mouse
        let cursor = render
            .camera
            .screen_to_world(vec2(mouse_position().0, mouse_position().1));

        let dev_info = [
            format!("average FPS: {:.2} ({:.2} ms)", avg_fps, avg_frame * 1000.0),
            format!(
                "mouse position (pixels) = ({:+.2}, {:+.2})",
                cursor.x, cursor.y
            ),
            format!("screen size: {:?}", get_screen().to_string()),
            format!(
                "camera zoom = {:.2} ({:.2},{:.2})",
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
            format!("launch time: {}", game.time_launch),
        ];

        let mut spacing = 15.0;
        for line in dev_info.iter() {
            draw_text(line.as_str(), 10.0, spacing, 20.0, BLACK);
            spacing += 20.0
        }

        next_frame().await;
    }
}
