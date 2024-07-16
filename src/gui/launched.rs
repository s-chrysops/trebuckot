use crate::{Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use super::SceneAssets;

const BUTTON_SIZE: Vec2 = vec2(240.0, 80.0);

pub fn launched(assets: &SceneAssets, game: &mut Game) -> Scene {
    root_ui().push_skin(&assets.skin);

    let mut next_scene = None;

    if widgets::Button::new("Pause")
        .position(vec2(screen_width() - BUTTON_SIZE.x, 0.0))
        .size(BUTTON_SIZE)
        .ui(&mut root_ui())
        || game.state == GameState::Paused
    {
        game.state = GameState::Paused;
        next_scene = Some(Scene::Paused);
    }

    if game.state == GameState::Landed {
        next_scene = Some(Scene::Landed);
    }

    root_ui().pop_skin();

    match next_scene {
        Some(scene) => scene,
        None => Scene::Launched,
    }
}
