use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use super::SceneAssets;

const BUTTON_SIZE: Vec2 = vec2(200.0, 60.0);

pub fn paused(assets: &SceneAssets, game: &mut Game) -> Scene {
    root_ui().push_skin(&assets.skin);
    let group_pos = (get_screen() - BUTTON_SIZE) / 2.0;

    let mut next_scene = None;
    widgets::Group::new(hash!(), vec2(200.0, 120.0))
        .position(group_pos)
        .ui(&mut root_ui(), |ui| {
            if widgets::Button::new("Continue").size(BUTTON_SIZE).ui(ui) {
                game.state = GameState::Launched;
                next_scene = Some(Scene::Launched);
            }
            if widgets::Button::new("Settings").size(BUTTON_SIZE).ui(ui) {
                next_scene = Some(Scene::Settings(Box::new(Scene::Paused)));
            }
        });

    root_ui().pop_skin();

    match next_scene {
        Some(next_scene) => next_scene,
        None => Scene::Paused,
    }
}
