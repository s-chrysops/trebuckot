use crate::{get_screen, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn settings(last_scene: Box<Scene>) -> Scene {
    let window_size = vec2(600.0, 400.0);
    let window_pos = (get_screen() - window_size) / 2.0;

    let mut next_scene = None;
    // root_ui().push_skin(&self.settings_skin);
    widgets::Window::new(hash!(), window_pos, window_size).ui(&mut root_ui(), |ui| {
        if widgets::Button::new("Close").position(window_size / 2.0).ui(ui) {
            next_scene = Some(last_scene.clone());
        }
    });
    // root_ui().pop_skin();

    if let Some(next_scene) = next_scene {
        *next_scene
    } else {
        Scene::Settings(last_scene)
    }
}
