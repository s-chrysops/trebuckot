use crate::{get_screen, Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use super::SceneAssets;

const WINDOW_SIZE: Vec2 = vec2(800.0, 600.0);
const BUTTON_SIZE: Vec2 = vec2(200.0, 60.0);

pub fn settings(assets: &SceneAssets, mut last_scene: Box<Scene>, game: &mut Game) -> Scene {
    root_ui().push_skin(&assets.skin);
    let window_pos = (get_screen() - WINDOW_SIZE) / 2.0;

    let mut next_scene = None;
    widgets::Window::new(hash!(), window_pos, WINDOW_SIZE)
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(), |ui| {
            widgets::Checkbox::new(hash!())
                .label("Autosave")
                .ui(ui, &mut game.settings.autosave);
            if widgets::Button::new("Close")
                .position((WINDOW_SIZE - BUTTON_SIZE) / 2.0 + vec2(0.0, 240.0))
                .size(BUTTON_SIZE)
                .ui(ui)
            {
                next_scene = Some(std::mem::take(&mut last_scene));
            }
        });

    root_ui().pop_skin();

    match next_scene {
        Some(next_scene) => *next_scene,
        None => Scene::Settings(last_scene),
    }
}
