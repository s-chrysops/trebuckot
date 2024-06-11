use crate::{Game, GameState};
use macroquad::ui::root_ui;

use gui_resources::GuiResources;
use landed::landed;
use paused::paused;
use prelaunch::prelaunch;
use settings::settings;
use title::title;

mod gui_resources;
mod landed;
mod paused;
mod prelaunch;
mod settings;
mod title;

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
    scene:     Scene,
    resources: GuiResources,
}

impl Gui {
    pub async fn init() -> Result<Gui, macroquad::Error> {
        let gui_resources = GuiResources::init().await?;
        Ok(Gui {
            scene:     Scene::MainMenu,
            resources: gui_resources,
        })
    }

    pub async fn update(&mut self, game: &mut Game) {
        self.scene = match &self.scene {
            Scene::MainMenu => {
                root_ui().push_skin(&self.resources.title_skin);
                title(game).await
            }
            Scene::Data => todo!(),
            Scene::Credits => todo!(),
            Scene::Paused => paused(game).await,
            Scene::PreLaunch => {
                root_ui().push_skin(&self.resources.prelaunch_skin);
                prelaunch(game).await
            }
            Scene::Launched => match game.state {
                GameState::Paused => Scene::Paused,
                GameState::Landed => Scene::Landed,
                _ => Scene::Launched,
            },
            Scene::Landed => {
                root_ui().push_skin(&self.resources.landed_skin);
                landed(game).await
            }
            Scene::Settings(last_scene) => {
                root_ui().push_skin(&self.resources.settings_skin);
                settings(last_scene.clone()).await
            }
        };
        root_ui().pop_skin();
    }
}
