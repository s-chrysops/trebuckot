use crate::{get_screen, Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn landed(game: &mut Game) -> Scene {
    let window_size = vec2(800.0, 600.0);
    let window_pos = (get_screen() - window_size) / 2.0;
    let button_size = vec2(200.0, 60.0);

    let mut next_scene = None;
    widgets::Window::new(hash!(), window_pos, window_size)
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(), |ui| {
            // for stat in game.stats.as_vec() {
            //     ui.label(None, &stat.field)
            // }
            // ui.separator();

            // let thing = "Research Earned";
            // // measure_text(&thing, font, 48, 1.0);
            // ui.label(window_size / 2.0 + vec2(-170.0, 120.0), thing);
            if widgets::Button::new("Next Day")
                .position((window_size - button_size) / 2.0 + vec2(0.0, 240.0))
                .size(button_size)
                .ui(ui)
            {
                game.next_day();
                next_scene = Some(Scene::PreLaunch)
            }
        });

    match next_scene {
        Some(scene) => scene,
        None => Scene::Landed,
    }
}
