use crate::{get_screen, Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use super::{PreLaunchState, SceneAssets};

const WINDOW_SIZE: Vec2 = vec2(800.0, 600.0);
const BUTTON_SIZE: Vec2 = vec2(200.0, 60.0);
const BUTTON_OFFSET: Vec2 = vec2(0.0, 240.0);

pub fn landed(assets: &SceneAssets, game: &mut Game) -> Scene {
    root_ui().push_skin(&assets.skin);
    let window_pos = (get_screen() - WINDOW_SIZE) / 2.0;

    let mut next_scene = None;
    widgets::Window::new(hash!(), window_pos, WINDOW_SIZE)
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(), |ui| {
            // for stat in game.stats.as_vec() {
            //     ui.label(None, &stat.field)
            // }
            // ui.separator();

            // let thing = "Research Earned";
            // let calc = ui.calc_size(thing);
            // ui.label(WINDOW_SIZE / 2.0 + vec2(-170.0, 120.0), thing);
            if widgets::Button::new("Next Day")
                .position((WINDOW_SIZE - BUTTON_SIZE) / 2.0 + BUTTON_OFFSET)
                .size(BUTTON_SIZE)
                .ui(ui)
            {
                game.next_day();
                next_scene = Some(Scene::PreLaunch(PreLaunchState::default()))
            }
        });

    root_ui().pop_skin();

    match next_scene {
        Some(scene) => scene,
        None => Scene::Landed,
    }
}
