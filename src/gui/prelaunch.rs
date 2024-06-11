use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn prelaunch(game: &mut Game) -> Scene {
    let button_size = vec2(400.0, 120.0);

    let mut next_scene = None;
    widgets::Window::new(hash!(), Vec2::ZERO, get_screen())
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(), |ui| {
            if widgets::Button::new("LAUNCH")
                .position(get_screen() - button_size)
                .size(button_size)
                .ui(ui)
            {
                game.state = GameState::Launched;
                next_scene = Some(Scene::Launched)
            };
        });

    match next_scene {
        Some(scene) => scene,
        None => Scene::PreLaunch,
    }
}
