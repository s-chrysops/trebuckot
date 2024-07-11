use crate::{get_screen, Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

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

const BUTTON_SIZE: Vec2 = vec2(200.0, 60.0);
const MAINMENU_SIZE: Vec2 = vec2(200.0, 240.0);
const NEWGAME_SIZE: Vec2 = vec2(400.0, 360.0);

pub async fn title(mut state: TitleState, game: &mut Game) -> Scene {
    // let title_texture = Texture2D::from_file_with_format(
    //     include_bytes!("../../assets/ui/title.png"),
    //     Some(ImageFormat::Png),
    // );

    let mut next_scene = None;
    // Using a blank window and its background as a texture widget :P
    widgets::Window::new(
        hash!(),
        (get_screen() - vec2(600.0, 400.0)) / 2.0,
        vec2(600.0, 200.0),
    )
    .titlebar(false)
    .movable(false)
    .ui(&mut root_ui(), |_| {});

    match state.submenu {
        None => {
            let mainmenu_position = (get_screen() - MAINMENU_SIZE) / 2.0 + vec2(0.0, 180.0);
            widgets::Group::new(hash!(), MAINMENU_SIZE)
                .position(mainmenu_position)
                .ui(&mut root_ui(), |ui| {
                    if widgets::Button::new("NEW GAME").size(BUTTON_SIZE).ui(ui) {
                        state.submenu = Some(TitleSubMenu::NewGame);
                    }
                    if widgets::Button::new("DATA").size(BUTTON_SIZE).ui(ui) {
                        // next_scene = Some(Scene::Data);
                        state.submenu = Some(TitleSubMenu::Data);
                    }
                    if widgets::Button::new("SETTINGS").size(BUTTON_SIZE).ui(ui) {
                        next_scene = Some(Scene::Settings(Box::new(Scene::MainMenu(state))));
                    }
                    if widgets::Button::new("CREDITS").size(BUTTON_SIZE).ui(ui) {
                        state.submenu = Some(TitleSubMenu::Credits);
                    }
                });
        }
        Some(TitleSubMenu::NewGame) => {
            let newgame_position = (get_screen() - NEWGAME_SIZE) / 2.0 + vec2(0.0, 180.0);
            let scale_select_size = vec2(400.0, 80.0);

            widgets::Group::new(hash!(), NEWGAME_SIZE)
                .position(newgame_position)
                .ui(&mut root_ui(), |ui| {
                    if widgets::Button::new("SMALL")
                        .size(scale_select_size)
                        .selected(state.scale == Some(UniverseScale::Small))
                        .ui(ui)
                    {
                        state.scale = Some(UniverseScale::Small);
                    }
                    if widgets::Button::new("MEDIUM")
                        .size(scale_select_size)
                        .selected(state.scale == Some(UniverseScale::Medium))
                        .ui(ui)
                    {
                        state.scale = Some(UniverseScale::Medium);
                    }
                    if widgets::Button::new("BIG")
                        .size(scale_select_size)
                        .selected(state.scale == Some(UniverseScale::Large))
                        .ui(ui)
                    {
                        state.scale = Some(UniverseScale::Large);
                    }

                    let text = match state.scale {
                        Some(UniverseScale::Small) => "Universe 100 times smaller",
                        Some(UniverseScale::Medium) => "Universe 10 times smaller",
                        Some(UniverseScale::Large) => "Universe at real life scale",
                        None => "Select scale of the Universe",
                    };
                    let align = (NEWGAME_SIZE.x - ui.calc_size(text).x) / 2.0;
                    ui.label(None, "");
                    ui.same_line(align);
                    ui.label(None, text);

                    if widgets::Button::new("START").size(BUTTON_SIZE).ui(ui)
                        && state.scale.is_some()
                    {
                        game.settings.scale = match state.scale.unwrap() {
                            UniverseScale::Small => 0.01,
                            UniverseScale::Medium => 0.1,
                            UniverseScale::Large => 1.0,
                        };
                        game.new_game();
                        next_scene = Some(Scene::PreLaunch);
                    }
                    ui.same_line(BUTTON_SIZE.x - 0.0001); // I hate this so much
                    if widgets::Button::new("BACK").size(BUTTON_SIZE).ui(ui) {
                        state.submenu = None;
                    }
                });
        }
        Some(TitleSubMenu::Data) => {}
        Some(TitleSubMenu::Credits) => {}
    }

    match next_scene {
        Some(scene) => scene,
        None => Scene::MainMenu(state),
    }
}
