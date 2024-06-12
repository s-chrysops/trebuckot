use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn paused(game: &mut Game) -> Scene {
    let buttons_size = vec2(200.0, 60.0);
    let buttons_pos = (get_screen() - buttons_size) / 2.0;

    let mut next_scene = None;
    widgets::Group::new(hash!(), vec2(200.0, 120.0))
        .position(buttons_pos)
        .ui(&mut root_ui(), |ui| {
            if widgets::Button::new("Continue").size(buttons_size).ui(ui) {
                game.state = GameState::Launched;
                next_scene = Some(Scene::Launched);
            }
            if widgets::Button::new("Settings").size(buttons_size).ui(ui) {
                next_scene = Some(Scene::Settings(Box::new(Scene::Paused)));
            }
        });

    match next_scene {
        Some(next_scene) => next_scene,
        None => Scene::Paused,
    }
}
