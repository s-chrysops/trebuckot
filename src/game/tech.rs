use macroquad::file::load_string;
use nanoserde::DeJson;
// use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, DeJson)]
pub struct Tech {
    pub description: String,
    pub cost:        u32,

    pub parent: Option<String>,

    #[nserde(default)]
    pub obtained: bool,
}

// impl Tech {
//     pub fn new(description: String, cost: u32, level: u8, parent: Option<String>) -> Self {
//         Self {
//             description,
//             cost,
//             level,
//             parent,
//             obtained: false,
//         }
//     }

//     pub fn available(&self) -> bool {
//         match &self.parent {
//             Some(parent) => parent.obtained,
//             None => false,
//         }
//     }
// }

#[derive(DeJson)]
struct TechPrime {
    pub name: String,
    pub spot: (u8, u8),
    pub info: Tech,
}

pub struct TechTree {
    pub names: Vec<String>,
    spots:     Vec<(u8, u8)>,
    techs:     Vec<Tech>,
}

impl TechTree {
    pub async fn init() -> Result<TechTree, macroquad::Error> {
        // let mut tech_tree = HashMap::new();

        // let techs = vec![
        //     ("test0", Tech::new("description".to_owned(), 100, 0, None)),
        //     ("test1", Tech::new("description".to_owned(), 100, 0, Some("test1".to_owned()))),
        //     ("test2", Tech::new("description".to_owned(), 100, 0, None)),
        //     ("test3", Tech::new("description".to_owned(), 100, 0, None)),
        // ];

        // let json = r#"{
        //     "test0": {"description": "blubblubblub", "cost": 100, "level": 0},
        //     "test1": {"description": "blabblabblab", "cost": 100, "level": 0, "parent": "test0"},
        // }"#;

        let json = load_string("techtree.json").await?;
        let import: Vec<TechPrime> = DeJson::deserialize_json(&json).unwrap();

        let size = import.len();
        let mut names = Vec::with_capacity(size);
        let mut spots = Vec::with_capacity(size);
        let mut techs = Vec::with_capacity(size);

        import.into_iter().for_each(|prime| {
            names.push(prime.name);
            spots.push(prime.spot);
            techs.push(prime.info);
        });

        // let map = HashMap::from_iter(import.into_iter().map(|tech| (tech.name, tech.info)));
        // TODO: add Custom Error enum to wrap macroquad and nanoserde errors

        // let tech_tree = HashMap::from_iter(
        //     techs
        //         .into_iter()
        //         .map(|(name, tech)| (name.to_owned(), tech)),
        // );

        Ok(TechTree {
            names,
            spots,
            techs,
        })
    }

    pub fn idx(&self, tech_name: &str) -> Option<usize> {
        self.names.iter().position(|name| tech_name == name)
    }

    pub fn get(&self, tech_name: &str) -> Option<&Tech> {
        // self.techs.get(tech_name)
        // let index = self.names.iter().position(|name| tech_name == name)?;
        self.techs.get(self.idx(tech_name)?)
    }

    pub fn spot(&self, tech_name: &str) -> Option<&(u8, u8)> {
        self.spots.get(self.idx(tech_name)?)
    }

    // pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Tech> {
    //     self.techs.iter()
    // }

    // pub fn tech_is_available(&self, tech_name: &str) -> bool {
    //     let tech = self.get(tech_name).expect("No tech with that name");
    //     match &tech.parent {
    //         Some(parent) => self.get(parent).unwrap().obtained,
    //         None => true,
    //     }
    // }

    pub fn is_available(&self, tech_name: &str) -> bool {
        self.get(tech_name)
            .map(|tech| {
                tech.parent
                    .as_ref()
                    .map_or(true, |parent| self.get(parent).unwrap().obtained)
            })
            .unwrap_or_else(|| panic!("No tech with that name"))
    }
}

#[cfg(test)]
mod test {
    use macroquad::file::set_pc_assets_folder;

    use super::TechTree;

    #[macroquad::test("Test")]
    async fn tech_test() {
        set_pc_assets_folder("assets");
        let tech_tree = TechTree::init().await.unwrap();

        for (k, v) in tech_tree.names.iter().zip(&tech_tree.techs) {
            println!("{:?}, {:?}", k, v);
        }

        println!("{}", tech_tree.is_available("test0"));

        println!("{}", tech_tree.is_available("test1"));
    }
}
