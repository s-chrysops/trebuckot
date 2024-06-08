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
mod player;
mod render;
mod resources;
mod trebuchet;
mod utils;
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
        icon: Some(gui::icon::set()),
        sample_count: 0,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    let mut game = Game::init().await;
    let mut physics = Physics::init();
    let mut render = Render::init().await;
    let mut gui = Gui::init().await;
    let mut dev_info = DevInfo::init();

    loop {
        gui.update().await;
        physics.update(&mut game);
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
