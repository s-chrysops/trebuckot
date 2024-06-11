use macroquad::{miniquad::conf::Icon, prelude::ImageFormat, texture::Image};

fn load_extract(bytes: &[u8]) -> Vec<u8> {
    Image::from_file_with_format(bytes, Some(ImageFormat::Png))
        .unwrap()
        .get_image_data()
        .iter()
        .flat_map(|p| *p)
        .collect()
}

pub fn set() -> Icon {
    Icon {
        small:  load_extract(include_bytes!("../../assets/ui/icon_16.png"))
            .try_into()
            .unwrap(),
        medium: load_extract(include_bytes!("../../assets/ui/icon_32.png"))
            .try_into()
            .unwrap(),
        big:    load_extract(include_bytes!("../../assets/ui/icon_64.png"))
            .try_into()
            .unwrap(),
    }
}
