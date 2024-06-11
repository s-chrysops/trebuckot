use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn main_menu(game: &mut Game) -> Scene {
    let buttons_size = vec2(200.0, 60.0);
    let buttons_pos = (get_screen() - buttons_size) / 2.0 + vec2(0.0, buttons_size.y);
    // let title_texture = Texture2D::from_file_with_format(
    //     include_bytes!("../../assets/ui/title.png"),
    //     Some(ImageFormat::Png),
    // );

    let mut next_scene = None;
    // Using a blank window and its background as a texture widget :P
    widgets::Window::new(hash!(), (get_screen() - vec2(600.0, 400.0)) / 2.0, vec2(600.0, 200.0))
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(),|_| {});
    widgets::Group::new(hash!(), vec2(200.0, 240.0))
        .position(buttons_pos)
        .ui(&mut root_ui(), |ui| {
            if widgets::Button::new("START").size(buttons_size).ui(ui) {
                game.state = GameState::PreLaunch;
                next_scene = Some(Scene::PreLaunch);
            }
            if widgets::Button::new("DATA").size(buttons_size).ui(ui) {
                next_scene = Some(Scene::Data);
            }
            if widgets::Button::new("SETTINGS").size(buttons_size).ui(ui) {
                next_scene = Some(Scene::Settings(Box::new(Scene::MainMenu)));
            }
            if widgets::Button::new("CREDITS").size(buttons_size).ui(ui) {
                next_scene = Some(Scene::Credits);
            }
        });
        // });

    match next_scene {
        Some(scene) => scene,
        None => Scene::MainMenu,
    }
}
