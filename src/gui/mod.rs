use crate::game::Game;
use crate::GameState;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

use landed_ui::landed_menu;
use main_menu::main_menu;
use pause_menu::pause_menu;
use prelaunch_ui::prelaunch_ui;
use settings::settings;

pub mod icon;
mod landed_ui;
mod main_menu;
mod pause_menu;
mod prelaunch_ui;
mod settings;

#[derive(Debug, Clone)]
pub enum Scene {
    MainMenu,
    Data,
    Credits,
    Paused,
    PreLaunch,
    Launched,
    Landed,
    Settings(Box<Scene>),
}

pub struct Gui {
    scene:          Scene,
    main_menu_skin: Skin,
    prelaunch_skin: Skin,
    landed_skin:    Skin,
    settings_skin:  Skin,
}

impl Gui {
    pub async fn init() -> Result<Gui, macroquad::Error> {
        let black75 = Image::gen_image_color(1, 1, color_u8!(0, 0, 0, 64));

        let main_menu_skin = {
            let button_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/VT323.ttf"))?
                .font_size(36)
                .text_color(WHITE)
                .background(Image::empty())
                .build();
            let window_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/ui/title.png"),
                    Some(ImageFormat::Png),
                )?)
                .build();
            let group_style = root_ui()
                .style_builder()
                .color(color_u8!(0, 0, 0, 0))
                .build();
            Skin {
                button_style,
                window_style,
                group_style,
                margin: 0.0,
                ..root_ui().default_skin()
            }
        };

        let prelaunch_skin = {
            let button_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/VT323.ttf"))?
                .font_size(72)
                .text_color(WHITE)
                .background(Image::empty())
                .build();
            let window_style = root_ui().style_builder().background(Image::empty()).build();
            Skin {
                button_style,
                window_style,
                ..root_ui().default_skin()
            }
        };

        let landed_skin = {
            let button_style = root_ui()
                .style_builder()
                .background(Image::empty())
                .font(include_bytes!("../../assets/VT323.ttf"))?
                .font_size(36)
                .text_color(WHITE)
                .build();
            // let label_style = root_ui()
            //     .style_builder()
            //     .font(include_bytes!("../../assets/VT323.ttf"))?
            //     .font_size(48)
            //     .text_color(WHITE)
            //     .margin(RectOffset::new(60.0, 60.0, 40.0, 0.0))
            //     .build();
            let window_style = root_ui().style_builder().background(Image::empty()).build();
            Skin {
                // label_style,
                button_style,
                window_style,
                ..root_ui().default_skin()
            }
        };

        let settings_skin = {
            let button_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/VT323.ttf"))?
                .font_size(36)
                .text_color(WHITE)
                .background(Image::empty())
                .build();
            let window_style = root_ui()
                .style_builder()
                .background(black75.clone())
                .build();
            Skin {
                button_style,
                window_style,
                ..root_ui().default_skin()
            }
        };

        Ok(Gui {
            scene: Scene::MainMenu,
            main_menu_skin,
            prelaunch_skin,
            landed_skin,
            settings_skin,
        })
    }

    pub async fn update(&mut self, game: &mut Game) {
        self.scene = match &self.scene {
            Scene::MainMenu => {
                root_ui().push_skin(&self.main_menu_skin);
                main_menu(game).await
            }
            Scene::Data => todo!(),
            Scene::Credits => todo!(),
            Scene::Paused => pause_menu(game).await,
            Scene::PreLaunch => {
                root_ui().push_skin(&self.prelaunch_skin);
                prelaunch_ui(game).await
            }
            Scene::Launched => match game.state {
                GameState::Paused => Scene::Paused,
                GameState::Landed => Scene::Landed,
                _ => Scene::Launched,
            },
            Scene::Landed => {
                root_ui().push_skin(&self.landed_skin);
                landed_menu(game).await
            }
            Scene::Settings(last_scene) => {
                root_ui().push_skin(&self.settings_skin);
                settings(last_scene.clone()).await
            }
        };
        root_ui().pop_skin();
    }
}
