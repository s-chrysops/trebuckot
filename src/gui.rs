use crate::{Game, GameError};

use landed::landed;
use launched::launched;
use paused::paused;
use prelaunch::*;
use scene_assets::*;
use settings::settings;
use title::*;

mod landed;
mod launched;
mod paused;
mod prelaunch;
mod scene_assets;
mod settings;
mod title;

#[derive(Default, Debug, Clone)]
pub enum Scene {
    #[default]
    None,
    Title(TitleState),
    Paused,
    PreLaunch(PreLaunchState),
    Launched,
    Landed,
    Settings(Box<Scene>),
}

pub struct Gui {
    scene:  Scene,
    assets: Vec<SceneAssets>,
}

impl Gui {
    pub async fn init() -> Result<Gui, GameError> {
        Ok(Gui {
            scene:  Scene::Title(TitleState::default()),
            assets: scene_assets::init().await?,
        })
    }

    pub fn update(&mut self, game: &mut Game) {
        self.scene = match std::mem::take(&mut self.scene) {
            Scene::None => unreachable!(),
            Scene::Title(state) => title(&self.assets[TITLE], state, game),
            Scene::Paused => paused(&self.assets[PAUSED], game),
            Scene::PreLaunch(state) => prelaunch(&self.assets[PRELAUNCH], state, game),
            Scene::Launched => launched(&self.assets[LAUNCHED], game),
            Scene::Landed => landed(&self.assets[LANDED], game),
            Scene::Settings(last_scene) => settings(&self.assets[SETTINGS], last_scene, game),
        };
    }
}
