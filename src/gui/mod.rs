use crate::game::{Game, GameState};
use crate::get_screen;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

use main_menu::main_menu;
use pause_menu::pause_menu;
use settings::settings;

pub mod icon;
mod main_menu;
mod pause_menu;
mod settings;

#[derive(Debug, Clone)]
pub enum Scene {
    MainMenu,
    Data,
    Credits,
    Playing,
    Pause,
    Landed,
    Settings(Box<Scene>),
}

pub struct Gui {
    scene:          Scene,
    main_menu_skin: Skin,
    settings_skin:  Skin,
}

impl Gui {
    pub async fn init() -> Self {
        let black75 = Image::gen_image_color(1, 1, color_u8!(0, 0, 0, 64));

        let main_menu_skin = {
            let window_style = root_ui()
                .style_builder()
                .background(
                    Image::from_file_with_format(
                        include_bytes!("../../assets/ui/title_back.png"),
                        Some(ImageFormat::Png),
                    )
                    .unwrap(),
                )
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
            scene: Scene::MainMenu,
            main_menu_skin,
            settings_skin,
        }
    }

    pub async fn update(&mut self) {
        self.scene = match &self.scene {
            Scene::MainMenu => {
                root_ui().push_skin(&self.main_menu_skin);
                main_menu().await
            }
            Scene::Data => todo!(),
            Scene::Credits => todo!(),
            Scene::Playing => {
                Scene::Playing
            }
            Scene::Pause => pause_menu().await,
            Scene::Landed => todo!(),
            Scene::Settings(last_scene) => {
                root_ui().push_skin(&self.settings_skin);
                settings(last_scene.clone()).await
            }
        };
        root_ui().pop_skin();
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
