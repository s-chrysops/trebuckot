use crate::game::{get_screen,Game, GameState};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

pub struct MainMenu {
    skin: Skin,
}

impl MainMenu {
    pub fn main_menu(&mut self, game: &mut Game) {
        let buttons_size = vec2(200.0, 60.0);
        let buttons_pos = (get_screen() - buttons_size) / 2.0 + vec2(0.0, buttons_size.y);
        let title_texture = Texture2D::from_file_with_format(
            include_bytes!("../../assets/ui/title.png"),
            Some(ImageFormat::Png),
        );

        root_ui().push_skin(&self.main_menu_skin);
        root_ui().window(hash!(), Vec2::default(), get_screen(), |ui| {
            widgets::Texture::new(title_texture)
                .size(600.0, 200.0)
                .position((get_screen() - vec2(600.0, 400.0)) / 2.0)
                .ui(ui);
            widgets::Group::new(hash!(), vec2(200.0, 240.0))
                .position(buttons_pos)
                .ui(ui, |ui| {
                    if widgets::Button::new("START").size(buttons_size).ui(ui) {
                        game.state = GameState::PreLaunch;
                    }
                    if widgets::Button::new("DATA").size(buttons_size).ui(ui) {
                        print!("Data");
                    }
                    if widgets::Button::new("SETTINGS").size(buttons_size).ui(ui) {
                        self.settings_on = true;
                    }
                    if widgets::Button::new("CREDITS").size(buttons_size).ui(ui) {
                        print!("Credits");
                    }
                });
        });
        root_ui().pop_skin();
        if self.settings_on {
            self.settings()
        }
    }
}