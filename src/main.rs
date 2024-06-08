use dev_info::*;
use game::*;
use macroquad::prelude::*;
use physics::*;
use render::*;
use ui::*;

mod dev_info;
mod game;
mod physics;
mod player;
mod render;
mod resources;
mod trebuchet;
mod ui;
mod world;
mod utils;

const PHYSICS_TICK: f32 = 0.001;

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
    let mut dev_info = DevInfo::init();

    let mut time_acc = 0.0;
    
    loop {
        match game.state {
            GameState::MainMenu => {
                ui.main_menu(&mut game);
            }
            GameState::Paused => {
                ui.pause_menu(&mut game);
            }
            GameState::PreLaunch => {
                if is_key_released(KeyCode::Space) {
                    game.state = GameState::Launched;
                }
            }
            GameState::Launched => {
                time_acc += get_frame_time();
                while time_acc > PHYSICS_TICK {
                    // Basic movement
                    if is_key_down(KeyCode::W) {
                        game.player.acceleration.y += game.player.move_speed;
                    }
                    if is_key_down(KeyCode::S) {
                        game.player.acceleration.y -= game.player.move_speed;
                    }
                    if is_key_down(KeyCode::A) {
                        game.player.acceleration.x -= game.player.move_speed;
                    }
                    if is_key_down(KeyCode::D) {
                        game.player.acceleration.x += game.player.move_speed;
                    }
                    if is_key_down(KeyCode::Escape) {
                        game.state = GameState::Paused;
                    }

                    game.trebuchet.run(PHYSICS_TICK);
                    if let trebuchet::TrebuchetState::Stage3 = game.trebuchet.state {
                        do_physics(&mut game, PHYSICS_TICK);
                    } else {
                        game.player.position = game.trebuchet.projectile_position();
                        game.player.velocity = game.trebuchet.v_projectile();
                    }

                    game.time_launch += PHYSICS_TICK;
                    time_acc -= PHYSICS_TICK;
                }
            }
            GameState::Landed => {
                ui.landed_menu(&mut game);
            }
        }

        render.update(&game);
        render.draw(&game);
        dev_info.draw(&game, &render);

        next_frame().await;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let ami = 1337;
        let cute = 1337;
        assert_eq!(ami, cute);
    }
}
