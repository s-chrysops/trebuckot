use macroquad::{
    material::{load_material, Material, MaterialParams},
    miniquad::{ShaderSource, UniformType},
    text::{load_ttf_font, Font},
    texture::{load_texture, Texture2D},
};

use crate::GameError;

pub struct RenderAssets {
    pub font: Font,

    texture_names: Vec<Box<str>>,
    textures:      Vec<Texture2D>,

    pub terrain_material: Material,
}

impl RenderAssets {
    pub async fn init() -> Result<RenderAssets, GameError> {
        let textures_to_load = [
            "bucko.png",
            "plushie_test.png",
            "hud/resources_cardboard.png",
            "trebuchet/cardboard_weight.png",
        ];

        let mut texture_names = Vec::with_capacity(textures_to_load.len());
        let mut textures = Vec::with_capacity(textures_to_load.len());

        for path in textures_to_load {
            let name = path
                .split('/')
                .last()
                .and_then(|s| s.strip_suffix(".png"))
                .unwrap();
            texture_names.push(name.into());
            textures.push(load_texture(path).await?)
        }

        let terrain_material = load_material(
            ShaderSource::Glsl {
                vertex:   TERRAIN_VERTEX,
                fragment: TERRAIN_FRAGMENT,
            },
            MaterialParams {
                uniforms: vec![
                    ("EdgeColor".to_string(), UniformType::Float4),
                    ("InnerColor".to_string(), UniformType::Float4),
                ],
                ..Default::default()
            },
        )?;

        Ok(RenderAssets {
            font: load_ttf_font("VT323.ttf").await?,
            texture_names,
            textures,
            terrain_material,
        })
    }

    pub fn get(&self, texture_name: &str) -> &Texture2D {
        let index = self
            .texture_names
            .iter()
            .position(|name| *texture_name == **name)
            .expect("Texture name not found!");
        &self.textures[index]
    }
}

const TERRAIN_VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    uv = texcoord;
    gl_Position = Projection * Model * vec4(position, 1);
}
"#;

const TERRAIN_FRAGMENT: &str = r#"#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
    precision highp float;
#else
    precision mediump float;
#endif

varying vec2 uv;

uniform vec4 EdgeColor;
uniform vec4 InnerColor;

void main() {
    vec4 black = vec4(0.0, 0.0, 0.0, 1.0);

    float transition1 = smoothstep(0.0, 0.0001, uv.y);
    float transition2 = smoothstep(0.0, 0.8, uv.y);

    vec4 finalColor = mix(EdgeColor, InnerColor, transition1);
    finalColor = mix(finalColor, black, transition2);

    gl_FragColor = finalColor;
}
"#;
