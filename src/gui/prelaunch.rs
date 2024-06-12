use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn prelaunch(game: &mut Game) -> Scene {
    let button_size = vec2(400.0, 120.0);

    let mut next_scene = None;

    if widgets::Button::new("LAUNCH")
        .position(get_screen() - button_size)
        .size(button_size)
        .ui(&mut root_ui())
    {
        game.state = GameState::Launched;
        next_scene = Some(Scene::Launched)
    };

    if widgets::Button::new("TECH")
        .position(get_screen().with_x(0.0) - vec2(0.0, button_size.y))
        .size(button_size)
        .ui(&mut root_ui())
    {
        widgets::Window::new(hash!(), Vec2::ZERO, get_screen())
            .titlebar(false)
            .movable(false)
            .ui(&mut root_ui(), |ui| {
                widgets::Button::new("test")
                    .ui(ui);
            });
    }

    match next_scene {
        Some(scene) => scene,
        None => Scene::PreLaunch,
    }
}
