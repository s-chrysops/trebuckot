use crate::{get_screen, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn settings(last_scene: Box<Scene>) -> Scene {
    let window_size = vec2(800.0, 600.0);
    let window_pos = (get_screen() - window_size) / 2.0;
    let button_size = vec2(200.0, 60.0);

    let mut next_scene = None;
    // root_ui().push_skin(&self.settings_skin);
    widgets::Window::new(hash!(), window_pos, window_size)
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(), |ui| {
        if widgets::Button::new("Close")
            .position((window_size - button_size) / 2.0 + vec2(0.0, 240.0))
            .size(button_size)
            .ui(ui)
        {
            next_scene = Some(last_scene.clone());
        }
    });
    // root_ui().pop_skin();

    match next_scene {
        Some(next_scene) => *next_scene,
        None => Scene::Settings(last_scene),
    }
}
