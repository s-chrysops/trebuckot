use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

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

    if widgets::Button::new("T")
        .position((get_screen() - button_size).with_x(0.0))
        .size(button_size)
        .ui(&mut root_ui())
    {
        next_scene = Some(Scene::Upgrades(2, None))
    }

    match next_scene {
        Some(scene) => scene,
        None => Scene::PreLaunch,
    }
}
