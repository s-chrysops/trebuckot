use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn paused(game: &mut Game) -> Scene {
    let button_size = vec2(200.0, 50.0);
    let mut next_scene = None;
    widgets::Popup::new(hash!(), get_screen()).ui(&mut root_ui(), |ui| {
        if widgets::Button::new("CONTINUE")
            .position((get_screen() - button_size) / 2.0)
            .size(button_size)
            .ui(ui)
        {
            game.state = GameState::Launched;
            next_scene = Some(Scene::Launched);
        }
        if widgets::Button::new("SETTINGS")
            .position((get_screen() - button_size) / 2.0 + vec2(0.0, button_size.y))
            .size(button_size)
            .ui(ui)
        {
            next_scene = Some(Scene::Settings(Box::new(Scene::Paused)));
        }
    });

    match next_scene {
        Some(next_scene) => next_scene,
        None => Scene::Paused,
    }
}
