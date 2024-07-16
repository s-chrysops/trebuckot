use std::collections::HashMap;

use macroquad::{
    color::*,
    color_u8,
    math::RectOffset,
    texture::{load_texture, Image, Texture2D},
    ui::{root_ui, Skin},
};

use crate::GameError;

pub struct SceneAssets {
    pub skin:     Skin,
    pub textures: HashMap<&'static str, Texture2D>,
}

pub const TITLE: usize = 0;
pub const PAUSED: usize = 1;
pub const PRELAUNCH: usize = 2;
pub const LAUNCHED: usize = 3;
pub const LANDED: usize = 4;
pub const SETTINGS: usize = 5;

pub async fn init() -> Result<Vec<SceneAssets>, GameError> {
    let vt323_bytes = include_bytes!("../../assets/VT323.ttf");

    let title_skin = {
        let label_style = root_ui()
            .style_builder()
            .font(vt323_bytes)?
            .font_size(24)
            .text_color(WHITE)
            .margin(macroquad::math::RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .build();
        let button_style = root_ui()
            .style_builder()
            .font(vt323_bytes)?
            .font_size(36)
            .text_color(WHITE)
            .background(Image::empty())
            .color_selected(RED)
            .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .build();
        let window_style = root_ui()
            .style_builder()
            .build();
        let group_style = root_ui()
            .style_builder()
            .color(color_u8!(0, 0, 0, 0))
            .build();
        Skin {
            label_style,
            button_style,
            window_style,
            group_style,
            margin: 0.0,
            ..root_ui().default_skin()
        }
    };

    let mut title_textures = HashMap::new();
    title_textures.insert("title", load_texture("ui/title.png").await?);

    let paused_skin = {
        let button_style = root_ui()
            .style_builder()
            .font(vt323_bytes)?
            .font_size(36)
            .text_color(WHITE)
            .background(Image::empty())
            .build();
        let window_style = root_ui()
            .style_builder()
            .color(color_u8!(0, 0, 0, 0))
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
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../../assets/Silkscreen.ttf"))?
            .font_size(24)
            .text_color(WHITE)
            // .margin(macroquad::math::RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .build();
        let button_style = root_ui()
            .style_builder()
            .font(vt323_bytes)?
            .font_size(36)
            .text_color(WHITE)
            .background(Image::empty())
            .build();
        let window_style = root_ui()
            .style_builder()
            // .margin(macroquad::math::RectOffset::new(10.0, 10.0, 10.0, 10.0))
            .color(color_u8!(0, 0, 0, 64))
            .build();
        let group_style = root_ui()
            .style_builder()
            .color(color_u8!(0, 0, 0, 64))
            .build();
        Skin {
            label_style,
            button_style,
            window_style,
            group_style,
            margin: 0.0,
            ..root_ui().default_skin()
        }
    };

    let launched_skin = {
        Skin {
            ..root_ui().default_skin()
        }
    };

    let landed_skin = {
        // let label_style = root_ui()
        //     .style_builder()
        //     .font(vt323)?
        //     .font_size(48)
        //     .text_color(WHITE)
        //     .margin(RectOffset::new(60.0, 60.0, 40.0, 0.0))
        //     .build();
        let button_style = root_ui()
            .style_builder()
            .background(Image::empty())
            .font(vt323_bytes)?
            .font_size(36)
            .text_color(WHITE)
            .build();
        let window_style = root_ui()
            .style_builder()
            .color(color_u8!(0, 0, 0, 0))
            .build();
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
            .font(vt323_bytes)?
            .font_size(36)
            .text_color(WHITE)
            .background(Image::empty())
            .build();
        let window_style = root_ui()
            .style_builder()
            .color(color_u8!(0, 0, 0, 64))
            .build();
        Skin {
            button_style,
            window_style,
            ..root_ui().default_skin()
        }
    };

    Ok(vec![
        SceneAssets {
            skin:     title_skin,
            textures: title_textures,
        },
        SceneAssets {
            skin:     paused_skin,
            textures: HashMap::new(),
        },
        SceneAssets {
            skin:     prelaunch_skin,
            textures: HashMap::new(),
        },
        SceneAssets {
            skin:     launched_skin,
            textures: HashMap::new(),
        },
        SceneAssets {
            skin:     landed_skin,
            textures: HashMap::new(),
        },
        SceneAssets {
            skin:     settings_skin,
            textures: HashMap::new(),
        },
    ])
}
