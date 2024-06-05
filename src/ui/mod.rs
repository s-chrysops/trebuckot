use crate::game::{Game, GameState};
use crate::get_screen;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

pub mod icon;
// mod main_menu;

// enum Menu {
//     Main,
//     Pause,
//     Landed,
//     Settings,
// }

pub struct UI {
    main_menu_skin: Skin,
    settings_skin:  Skin,
    settings_on:    bool,
}

impl UI {
    pub fn init() -> Self {
        let black75 = Image::gen_image_color(1, 1, color_u8!(0, 0, 0, 64));

        let main_menu_skin = {
            let window_style = root_ui()
                .style_builder()
                .background(black75.clone())
                .build();
            //let button_style = root_ui().style_builder().build();
            Skin {
                window_style,
                //button_style,
                margin: 0.0,
                ..root_ui().default_skin()
            }
        };

        let settings_skin = {
            let group_style = root_ui()
                .style_builder()
                .background(black75.clone())
                .build();
            Skin {
                group_style,
                ..root_ui().default_skin()
            }
        };
        
        Self {
            main_menu_skin,
            settings_skin,
            settings_on: false,
        }
    }

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

    pub fn settings(&mut self) {
        let window_size = vec2(600.0, 400.0);
        let window_pos = (get_screen() - window_size) / 2.0;
        root_ui().push_skin(&self.settings_skin);
        root_ui().popup(hash!(), get_screen(), |ui| {
            if widgets::Button::new("Close").position(window_pos).ui(ui) {
                self.settings_on = false;
            }
        });
        root_ui().pop_skin();
    }

    pub fn pause_menu(&self, game: &mut Game) {
        widgets::Popup::new(hash!(), get_screen()).ui(&mut root_ui(), |ui| {
            if widgets::Button::new("CONTINUE")
                .position(get_screen() / 2.0 - vec2(100.0, 25.0))
                .size(vec2(200.0, 50.0))
                .ui(ui)
            {
                game.state = GameState::Launched;
            }
        });
    }

    pub fn landed_menu(&self, game: &mut Game) {
        widgets::Popup::new(hash!(), get_screen()).ui(&mut root_ui(), |ui| {
            if widgets::Button::new("Restart")
                .position(get_screen() / 2.0 - vec2(100.0, 25.0))
                .size(vec2(200.0, 50.0))
                .ui(ui)
            {
                game.reset();
            }
        });
    }
}
