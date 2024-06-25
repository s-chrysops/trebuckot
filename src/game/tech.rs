#![allow(clippy::question_mark)]
use crate::GameError;
use macroquad::{
    file::load_string,
    math::{vec2, Vec2},
    texture::{load_texture, Texture2D},
};
use nanoserde::DeJson;

#[derive(DeJson)]
struct Tech {
    pub name: String,
    pub cost: u32,
    pub spot: usize,
    pub desc: String,
    pub requ: Option<Vec<String>>,
    pub icon: Option<String>,
}

#[derive(Clone)]
pub struct TechTree {
    pub names:    Vec<Box<str>>,
    pub costs:    Vec<u32>,
    pub spots:    Vec<Vec2>,
    pub descs:    Vec<Box<str>>,
    pub requs:    Vec<Option<Vec<Box<str>>>>,
    pub icons:    Vec<Texture2D>,
    pub obtained: Vec<bool>,
}

impl TechTree {
    pub async fn init() -> Result<TechTree, GameError> {
        let json = load_string("techtree.json").await?;
        let import: Vec<Tech> = DeJson::deserialize_json(&json)?;

        let default_texture = load_texture("bucko.png").await?;

        let size = import.len();
        let mut names = Vec::with_capacity(size);
        let mut costs = Vec::with_capacity(size);
        let mut level = Vec::with_capacity(size);
        let mut descs = Vec::with_capacity(size);
        let mut requs = Vec::with_capacity(size);
        let mut icons = Vec::with_capacity(size);

        for tech in import.into_iter() {
            names.push(tech.name.into());
            costs.push(tech.cost);
            level.push(tech.spot);
            descs.push(tech.desc.into());
            requs.push(tech.requ.map(|rs| rs.into_iter().map(Into::into).collect()));
            icons.push(match tech.icon {
                Some(path) => load_texture(&path).await?,
                None => default_texture.clone(),
            });
        }

        let max_level = *level.iter().max().unwrap();
        let per_level: Vec<f32> = (0..=max_level)
            .map(|c| (level.iter().filter(|l| &c == *l).count() + 1) as f32)
            .collect();
        let mut count: Vec<f32> = vec![0.0; max_level + 1];
        let spots = level
            .into_iter()
            .map(|l| {
                count[l] += 1.0;
                vec2(
                    count[l] / per_level[l],
                    (l + 1) as f32 / (max_level + 2) as f32,
                )
            })
            .collect();

        Ok(TechTree {
            names,
            costs,
            spots,
            descs,
            requs,
            icons,
            obtained: vec![false; size],
        })
    }

    pub fn get_index(&self, tech_name: &str) -> usize {
        self.names
            .iter()
            .position(|name| *tech_name == **name)
            .expect("No tech with that name")
    }

    pub fn available(&self, index: usize) -> bool {
        match &self.requs[index] {
            Some(parents) => parents.iter().all(|parent| self.have(parent)),
            None => true,
        }
    }

    pub fn have(&self, tech_name: &str) -> bool {
        let index = self.get_index(tech_name);
        self.obtained[index]
    }
}

#[cfg(test)]
mod test {
    use super::TechTree;
    use macroquad::file::set_pc_assets_folder;

    #[macroquad::test("Test")]
    async fn tech_test() {
        set_pc_assets_folder("assets");
        let mut tech_tree = TechTree::init().await.unwrap();

        let tech_index = tech_tree.get_index("Bucko Lab I");
        println!("{}", tech_tree.have("Bucko Lab I"));
        println!("{}", tech_tree.have("test2"));
        tech_tree.obtained[tech_index] = true;
        println!("{}", tech_tree.have("Bucko Lab I"));
        println!("{}", tech_tree.have("test2"));
    }
}
