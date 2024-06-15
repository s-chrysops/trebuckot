#![allow(clippy::question_mark)]
use crate::GameError;
use macroquad::{
    file::load_string,
    texture::{load_texture, Texture2D},
};
use nanoserde::DeJson;

#[derive(DeJson)]
struct Tech {
    pub name:    String,
    pub cost:    u32,
    pub spot:    (u8, u8),
    pub desc:    String,
    pub parents: Option<Vec<String>>,
    pub icon:    Option<String>,
}

#[derive(Clone)]
pub struct TechTree {
    pub names:    Vec<String>,
    pub costs:    Vec<u32>,
    pub spots:    Vec<(u8, u8)>,
    pub descs:    Vec<String>,
    pub parents:  Vec<Option<Vec<String>>>,
    pub obtained: Vec<bool>,
    pub icons:    Vec<Texture2D>,
}

impl TechTree {
    pub async fn init() -> Result<TechTree, GameError> {
        let json = load_string("techtree.json").await?;
        let import: Vec<Tech> = DeJson::deserialize_json(&json)?;

        let default_texture = load_texture("bucko.png").await?;

        let size = import.len();
        let mut names = Vec::with_capacity(size);
        let mut costs = Vec::with_capacity(size);
        let mut spots = Vec::with_capacity(size);
        let mut descs = Vec::with_capacity(size);
        let mut parents = Vec::with_capacity(size);
        let mut icons = Vec::with_capacity(size);

        for tech in import.into_iter() {
            names.push(tech.name);
            costs.push(tech.cost);
            spots.push(tech.spot);
            descs.push(tech.desc);
            parents.push(tech.parents);
            icons.push(match tech.icon {
                Some(path) => load_texture(&path).await?,
                None => default_texture.clone(),
            });
        }

        Ok(TechTree {
            names,
            costs,
            spots,
            descs,
            parents,
            obtained: vec![false; size],
            icons,
        })
    }

    pub fn get_index(&self, tech_name: &str) -> usize {
        self.names
            .iter()
            .position(|name| tech_name == name)
            .expect("No tech with that name")
    }

    // pub fn icon(&self, tech_name: &str) -> &Texture2D {
    //     let index = self.get_index(tech_name);
    //     &self.icons[index]
    // }

    // pub fn spot(&self, tech_name: &str) -> Option<&(u8, u8)> {
    //     self.spots.get(self.get_index(tech_name))
    // }

    pub fn available(&self, index: usize) -> bool {
        // let index = self.get_index(tech_name);
        match &self.parents[index] {
            Some(parents) => parents.iter().all(|parent| self.have(parent)),
            None => true,
        }
    }

    pub fn have(&self, tech_name: &str) -> bool {
        let index = self.get_index(tech_name);
        self.obtained[index]
    }

    // pub fn buy(&mut self, tech_name: &str) {
    //     let index = self.get_index(tech_name);
    //     self.obtained[index] = true;
    // }
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
