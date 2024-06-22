use crate::{Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};

const BUTTON_SIZE: Vec2 = vec2(200.0, 40.0);

#[derive(Debug, Clone, Copy, Default)]
pub struct UpgradesState {
    tab:  UpgradesTab,
    tech: Option<usize>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum UpgradesTab {
    #[default]
    Research,
    Trebuchet,
    Player,
    Logs,
}

impl From<u32> for UpgradesTab {
    fn from(value: u32) -> Self {
        match value {
            0 => UpgradesTab::Research,
            1 => UpgradesTab::Trebuchet,
            2 => UpgradesTab::Player,
            3 => UpgradesTab::Logs,
            _ => unreachable!(),
        }
    }
}

pub async fn upgrades(mut state: UpgradesState, game: &mut Game) -> Scene {
    let tabs = ["Research", "Trebuchet", "Player", "Logs"];
    let window_width = screen_width() / 4.0;

    // let mut next_tab = tab;
    // let ui = &mut root_ui();
    let mut next_scene = None;
    widgets::Window::new(
        hash!(),
        Vec2::ZERO.with_y(60.0),
        vec2(window_width, screen_height() - 60.0),
    )
    .titlebar(false)
    .movable(false)
    .ui(&mut root_ui(), |ui| {
        match UpgradesTab::from(
            widgets::Tabbar::new(hash!(), vec2(window_width, 60.0), &tabs).ui(ui),
        ) {
            // Research
            UpgradesTab::Research => match state.tech {
                Some(tech_index) => tech_menu(ui, game, tech_index),
                None => ui.label(None, "No tech selected"),
            },
            // Trebuchet
            UpgradesTab::Trebuchet => ui.label(None, &format!("{:?}", game.state)),
            // Player
            UpgradesTab::Player => {}
            // Logs
            UpgradesTab::Logs => {}
        };
        if widgets::Button::new("Close")
            .position(vec2(
                (window_width - BUTTON_SIZE.x) / 2.0,
                screen_height() - BUTTON_SIZE.y - 60.0,
            ))
            .size(BUTTON_SIZE)
            .ui(ui)
        {
            next_scene = Some(Scene::PreLaunch);
        };
    });

    // Tech Tree Selector
    if state.tab == UpgradesTab::Research {
        widgets::Group::new(hash!(), vec2(screen_width() * 0.75, screen_height() - 60.0))
            .position(vec2(window_width, 60.0))
            .ui(&mut root_ui(), |ui| {
                for index in 0..game.tech_tree.names.len() {
                    let layout = game.tech_tree.spots[index];
                    if widgets::Button::new(game.tech_tree.icons[index].clone())
                        .position(vec2(layout.0 as f32 * 100.0, layout.1 as f32 * 100.0))
                        .size(Vec2::splat(80.0))
                        .ui(ui)
                    {
                        state.tech = Some(index)
                    }
                }
            });
    }

    match next_scene {
        Some(scene) => scene,
        None => Scene::Upgrades(state),
    }
}

fn tech_menu(ui: &mut Ui, game: &mut Game, tech_index: usize) {
    let window_width = screen_width() / 4.0;
    let action_pos = vec2(
        (window_width - BUTTON_SIZE.x) / 2.0,
        screen_height() - BUTTON_SIZE.y - 120.0,
    );
    let tech_name = game.tech_tree.names[tech_index].as_str();
    let tech_desc = game.tech_tree.descs[tech_index].as_str();
    let tech_cost = game.tech_tree.costs[tech_index];

    ui.label(None, tech_name);
    ui.label(None, " ");
    print_multiline(ui, tech_desc, window_width);

    if !game.tech_tree.available(tech_index) {
        ui.label(None, " ");
        ui.label(None, "Required:");
        for parent in game.tech_tree.parents[tech_index].as_ref().unwrap() {
            ui.label(None, parent.as_str());
        }
        return;
    }

    ui.label(None, " ");
    if game.tech_tree.obtained[tech_index] {
        ui.label(None, "Obtained!");
        return;
    } else {
        ui.label(None, &format!("Cost: {}", tech_cost));
    }

    if widgets::Button::new("RESEARCH")
        .position(action_pos)
        .size(BUTTON_SIZE)
        .ui(ui)
        && game.resources.research >= tech_cost
    {
        game.tech_tree.obtained[tech_index] = true;
        game.resources.research -= tech_cost;
    }
}

fn print_multiline(ui: &mut Ui, text: &str, width: f32) {
    let space = ui.calc_size(" ").x;
    let mut acc: f32 = 0.0;
    ui.label(None, ""); // start at new line
    for word in text.split(' ') {
        let word_size = ui.calc_size(word).x;
        if acc + word_size > width {
            ui.label(None, word); // new line
            acc = 0.0;
        } else {
            ui.same_line(acc);
            ui.label(None, word);
        }
        acc += word_size + space;
    }
}
