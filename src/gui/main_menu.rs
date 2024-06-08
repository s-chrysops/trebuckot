use crate::{get_screen, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn main_menu() -> Scene {
    let buttons_size = vec2(200.0, 60.0);
    let buttons_pos = (get_screen() - buttons_size) / 2.0 + vec2(0.0, buttons_size.y);
    // let title_texture = Texture2D::from_file_with_format(
    //     include_bytes!("../../assets/ui/title.png"),
    //     Some(ImageFormat::Png),
    // );

    let mut next_scene = None;
    // widgets::Texture::new(title_texture)
    //     .size(600.0, 200.0)
    //     .position((get_screen() - vec2(600.0, 400.0)) / 2.0)
    //     .ui(&mut root_ui());
    widgets::Window::new(hash!(), Vec2::ZERO, get_screen())
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            widgets::Group::new(hash!(), vec2(200.0, 240.0))
                .position(buttons_pos)
                .ui(ui, |ui| {
                    if widgets::Button::new("START").size(buttons_size).ui(ui) {
                        next_scene = Some(Scene::Playing);
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
        });

    if let Some(next_scene) = next_scene {
        next_scene
    } else {
        Scene::MainMenu
    }
}
