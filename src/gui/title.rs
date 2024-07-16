use crate::{get_screen, Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use super::{PreLaunchState, SceneAssets};

#[derive(Debug, Clone, Copy, Default)]
pub struct TitleState {
    submenu: Option<TitleSubMenu>,
    scale:   Option<UniverseScale>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UniverseScale {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy)]
enum TitleSubMenu {
    NewGame,
    Data,
    Credits,
}

const MAINMENU_SIZE: Vec2 = vec2(200.0, 240.0);
const NEWGAME_SIZE: Vec2 = vec2(400.0, 360.0);
const MENU_BUTTON_SIZE: Vec2 = vec2(200.0, 60.0);
const SCALE_BUTTON_SIZE: Vec2 = vec2(400.0, 80.0);
const MENU_OFFSET: Vec2 = vec2(0., 180.0);

pub fn title(assets: &SceneAssets, mut state: TitleState, game: &mut Game) -> Scene {
    root_ui().push_skin(&assets.skin);

    let mut next_scene = None;

    widgets::Texture::new(assets.textures.get("title").unwrap().clone())
        .position(0.5 * (get_screen() - vec2(600.0, 400.0)))
        .size(600.0, 200.0)
        .ui(&mut root_ui());

    match state.submenu {
        None => {
            let mainmenu_position = 0.5 * (get_screen() - MAINMENU_SIZE) + MENU_OFFSET;
            widgets::Group::new(hash!(), MAINMENU_SIZE)
                .position(mainmenu_position)
                .ui(&mut root_ui(), |ui| {
                    if widgets::Button::new("NEW GAME")
                        .size(MENU_BUTTON_SIZE)
                        .ui(ui)
                    {
                        state.submenu = Some(TitleSubMenu::NewGame);
                    }
                    if widgets::Button::new("DATA").size(MENU_BUTTON_SIZE).ui(ui) {
                        state.submenu = Some(TitleSubMenu::Data);
                    }
                    if widgets::Button::new("SETTINGS")
                        .size(MENU_BUTTON_SIZE)
                        .ui(ui)
                    {
                        next_scene = Some(Scene::Settings(Box::new(Scene::Title(state))));
                    }
                    if widgets::Button::new("CREDITS")
                        .size(MENU_BUTTON_SIZE)
                        .ui(ui)
                    {
                        state.submenu = Some(TitleSubMenu::Credits);
                    }
                });
        }

        Some(TitleSubMenu::NewGame) => {
            let newgame_position = 0.5 * (get_screen() - NEWGAME_SIZE) + MENU_OFFSET;

            widgets::Group::new(hash!(), NEWGAME_SIZE)
                .position(newgame_position)
                .ui(&mut root_ui(), |ui| {
                    if widgets::Button::new("SMALL")
                        .size(SCALE_BUTTON_SIZE)
                        .selected(state.scale == Some(UniverseScale::Small))
                        .ui(ui)
                    {
                        state.scale = Some(UniverseScale::Small);
                    }

                    if widgets::Button::new("MEDIUM")
                        .size(SCALE_BUTTON_SIZE)
                        .selected(state.scale == Some(UniverseScale::Medium))
                        .ui(ui)
                    {
                        state.scale = Some(UniverseScale::Medium);
                    }

                    if widgets::Button::new("BIG")
                        .size(SCALE_BUTTON_SIZE)
                        .selected(state.scale == Some(UniverseScale::Large))
                        .ui(ui)
                    {
                        state.scale = Some(UniverseScale::Large);
                    }

                    {
                        let text = match state.scale {
                            None => "Select scale of the Universe",
                            Some(UniverseScale::Small) => "Universe 100 times smaller",
                            Some(UniverseScale::Medium) => "Universe 10 times smaller",
                            Some(UniverseScale::Large) => "Universe at real life scale",
                        };

                        let align = (NEWGAME_SIZE.x - ui.calc_size(text).x) / 2.0;
                        ui.label(None, "");
                        ui.same_line(align);
                        ui.label(None, text);
                    }

                    if widgets::Button::new("START").size(MENU_BUTTON_SIZE).ui(ui)
                        && state.scale.is_some()
                    {
                        game.settings.scale = match state.scale.unwrap() {
                            UniverseScale::Small => 0.01,
                            UniverseScale::Medium => 0.1,
                            UniverseScale::Large => 1.0,
                        };
                        game.new_game();
                        next_scene = Some(Scene::PreLaunch(PreLaunchState::default()));
                    }
                    ui.same_line(MENU_BUTTON_SIZE.x - 0.0001); // I hate this so much
                    if widgets::Button::new("BACK").size(MENU_BUTTON_SIZE).ui(ui) {
                        state.submenu = None;
                    }
                });
        }

        Some(TitleSubMenu::Data) => {}

        Some(TitleSubMenu::Credits) => {}
    }

    root_ui().pop_skin();

    match next_scene {
        Some(scene) => scene,
        None => Scene::Title(state),
    }
}
