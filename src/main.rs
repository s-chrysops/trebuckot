use dev_info::*;
use game::*;
use gui::*;
use macroquad::prelude::*;
use physics::*;
use render::*;

mod dev_info;
mod game;
mod gui;
mod physics;
mod render;
mod utils;

#[derive(Debug)]
pub enum GameError {
    MQError(macroquad::Error),
    NSError(nanoserde::DeJsonErr),
}
impl From<macroquad::Error> for GameError {
    fn from(error: macroquad::Error) -> GameError {
        GameError::MQError(error)
    }
}
impl From<nanoserde::DeJsonErr> for GameError {
    fn from(error: nanoserde::DeJsonErr) -> GameError {
        GameError::NSError(error)
    }
}

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
        icon: Some(render::icon::set()),
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), GameError> {
    set_pc_assets_folder("assets");
    let mut game = Game::init().await?;
    let mut gui = Gui::init().await?;
    let mut physics = Physics::init().await;
    let mut render = Render::init().await?;

    let mut dev_info = DevInfo::init();
    loop {
        gui.update(&mut game);
        physics.update(&mut game);
        render.update(&game);

        render.draw(&game);
        dev_info.draw(&game, &render);

        next_frame().await;
    }
}

#[cfg(test)]
mod important_test {
    struct Ami {
        cute: bool,
    }

    #[test]
    fn ami_cute() {
        let ami = Ami { cute: true };
        assert!(ami.cute);
    }
}
