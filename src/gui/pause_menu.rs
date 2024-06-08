use crate::{get_screen, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn pause_menu() -> Scene {
    let mut next_scene = None;
    widgets::Popup::new(hash!(), get_screen()).ui(&mut root_ui(), |ui| {
        if widgets::Button::new("CONTINUE")
            .position(get_screen() / 2.0 - vec2(100.0, 25.0))
            .size(vec2(200.0, 50.0))
            .ui(ui)
        {
            next_scene = Some(Scene::Playing);
        }
    });

    if let Some(scene) = next_scene {
        scene
    } else {
        Scene::Pause
    }
}
