// use macroquad::prelude::*;
use macroquad::{
    color::*,
    color_u8,
    texture::Image,
    ui::{root_ui, Skin},
};

use crate::GameError;

pub struct GuiAssets {
    pub title_skin:     Skin,
    pub paused_skin:    Skin,
    pub prelaunch_skin: Skin,
    pub upgrades_skin:  Skin,
    pub landed_skin:    Skin,
    pub settings_skin:  Skin,
}

impl GuiAssets {
    pub async fn init() -> Result<GuiAssets, GameError> {
        let title_skin = {
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
                    None,
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

        let paused_skin = {
            let button_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/VT323.ttf"))?
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
            let button_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/VT323.ttf"))?
                .font_size(144)
                .text_color(WHITE)
                .background(Image::empty())
                .build();
            Skin {
                button_style,
                ..root_ui().default_skin()
            }
        };

        let upgrades_skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/Silkscreen.ttf"))?
                .font_size(24)
                .text_color(WHITE)
                // .margin(RectOffset::new(0.0, 0.0, 10.0, 10.0))
                .build();
            let button_style = root_ui()
                .style_builder()
                .font(include_bytes!("../../assets/VT323.ttf"))?
                .font_size(36)
                .text_color(WHITE)
                .background(Image::empty())
                .build();
            let window_style = root_ui()
                .style_builder()
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
                .font(include_bytes!("../../assets/VT323.ttf"))?
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

        Ok(GuiAssets {
            title_skin,
            paused_skin,
            prelaunch_skin,
            upgrades_skin,
            landed_skin,
            settings_skin,
        })
    }
}
