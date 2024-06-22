use crate::{Game, GameError, GameState};
use macroquad::ui::root_ui;

use gui_assets::GuiAssets;
use landed::landed;
use paused::paused;
use prelaunch::prelaunch;
use settings::settings;
use title::*;
use upgrades::*;

mod gui_assets;
mod landed;
mod paused;
mod prelaunch;
mod settings;
mod title;
mod upgrades;

#[derive(Debug, Clone)]
pub enum Scene {
    MainMenu(TitleState),
    Paused,
    PreLaunch,
    Upgrades(UpgradesState),
    Launched,
    Landed,
    Settings(Box<Scene>),
}

pub struct Gui {
    scene:  Scene,
    assets: GuiAssets,
}

impl Gui {
    pub async fn init() -> Result<Gui, GameError> {
        let assets = GuiAssets::init().await?;
        Ok(Gui {
            scene: Scene::MainMenu(TitleState::default()),
            assets,
        })
    }

    pub async fn update(&mut self, game: &mut Game) {
        self.scene = match &self.scene {
            Scene::MainMenu(state) => {
                root_ui().push_skin(&self.assets.title_skin);
                title(*state, game).await
            }
            // Scene::Data => todo!(),
            // Scene::Credits => todo!(),
            Scene::Paused => {
                root_ui().push_skin(&self.assets.paused_skin);
                paused(game).await
            }
            Scene::PreLaunch => {
                root_ui().push_skin(&self.assets.prelaunch_skin);
                prelaunch(game).await
            }
            Scene::Upgrades(state) => {
                root_ui().push_skin(&self.assets.upgrades_skin);
                upgrades(*state, game).await
            }
            Scene::Launched => match game.state {
                GameState::Paused => Scene::Paused,
                GameState::Landed => Scene::Landed,
                _ => Scene::Launched,
            },
            Scene::Landed => {
                root_ui().push_skin(&self.assets.landed_skin);
                landed(game).await
            }
            Scene::Settings(last_scene) => {
                root_ui().push_skin(&self.assets.settings_skin);
                settings(game, last_scene.clone()).await
            }
        };
        root_ui().pop_skin();
    }
}
