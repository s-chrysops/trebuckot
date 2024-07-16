use crate::{get_screen, Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};

use super::SceneAssets;

const RESOURCE_BAR_SIZE: f32 = 60.0;
const ACTION_BUTTON_SIZE: Vec2 = vec2(200.0, 40.0);
const TECH_BUTTON_SIZE: Vec2 = Vec2::splat(80.0);

const TABS: [&str; 4] = ["Research", "Trebuchet", "Player", "Logs"];
const TAB_RESEARCH: u32 = 0;
const TAB_TREBUCHET: u32 = 1;
const TAB_PLAYER: u32 = 2;
const TAB_LOGS: u32 = 3;

#[derive(Debug, Clone, Copy, Default)]
pub struct PreLaunchState {
    upgrades: bool,
    tab:      u32,
    tech:     Option<usize>,
}

pub fn prelaunch(assets: &SceneAssets, mut state: PreLaunchState, game: &mut Game) -> Scene {
    root_ui().push_skin(&assets.skin);

    let window_width = 0.25 * screen_width();

    let mut next_scene = None;

    if state.upgrades {
        widgets::Window::new(
            hash!(),
            Vec2::ZERO.with_y(RESOURCE_BAR_SIZE),
            vec2(window_width, screen_height() - RESOURCE_BAR_SIZE),
        )
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(), |ui| {
            match widgets::Tabbar::new(hash!(), vec2(window_width, RESOURCE_BAR_SIZE), &TABS)
                .selected_tab(Some(&mut state.tab))
                .ui(ui)
            {
                TAB_RESEARCH => match state.tech {
                    Some(tech_index) => tech_info(ui, game, tech_index),
                    None => ui.label(None, "No tech selected"),
                },
                TAB_TREBUCHET => ui.label(None, &format!("{:?}", game.state)),
                TAB_PLAYER => {}
                TAB_LOGS => {}
                _ => unreachable!(),
            };

            if widgets::Button::new("Close")
                .position(vec2(
                    0.5 * (window_width - ACTION_BUTTON_SIZE.x),
                    screen_height() - ACTION_BUTTON_SIZE.y - RESOURCE_BAR_SIZE,
                ))
                .size(ACTION_BUTTON_SIZE)
                .ui(ui)
            {
                // game.state = GameState::PreLaunch;
                // next_scene = Some(Scene::PreLaunch);
                state.upgrades = false;
            };
        });

        if state.tab == TAB_RESEARCH {
            state.tech = tech_selector(game, state.tech);
        }
    } else {
        if widgets::Button::new("LAUNCH")
            .position(get_screen() - ACTION_BUTTON_SIZE)
            .size(ACTION_BUTTON_SIZE)
            .ui(&mut root_ui())
        {
            game.state = GameState::Launched;
            next_scene = Some(Scene::Launched)
        };

        if widgets::Button::new("T")
            .position((get_screen() - ACTION_BUTTON_SIZE).with_x(0.0))
            .size(ACTION_BUTTON_SIZE)
            .ui(&mut root_ui())
        {
            // game.state = GameState::Paused;
            state.upgrades = true;
        }
    }

    root_ui().pop_skin();

    match next_scene {
        Some(scene) => scene,
        None => Scene::PreLaunch(state),
    }
}

fn tech_selector(game: &Game, mut selected_tech: Option<usize>) -> Option<usize> {
    let group_size = vec2(0.75 * screen_width(), screen_height() - RESOURCE_BAR_SIZE);
    let group_pos = vec2(0.25 * screen_width(), RESOURCE_BAR_SIZE);

    widgets::Group::new(hash!(), group_size)
        .position(group_pos)
        .ui(&mut root_ui(), |ui| {
            let scroll = ui.scroll();

            let mut canvas = ui.canvas();

            let mut lines = Vec::<(Vec2, Vec2)>::new();
            let mut backs = Vec::<Rect>::with_capacity(game.tech_tree.names.len());
            let mut buttons = Vec::<widgets::Button>::with_capacity(game.tech_tree.names.len());

            for index in 0..game.tech_tree.names.len() {
                let tech_position = game.tech_tree.spots[index] * group_size * vec2(1.0, 2.0);

                if let Some(requs) = &game.tech_tree.requs[index] {
                    for requ in requs {
                        let requ_index = game.tech_tree.get_index(requ);
                        let requ_position =
                            game.tech_tree.spots[requ_index] * group_size * vec2(1.0, 2.0);
                        lines.push((tech_position, requ_position));
                    }
                }

                backs.push(Rect::new(
                    tech_position.x + group_pos.x,
                    tech_position.y + group_pos.y + scroll.y,
                    80.0,
                    80.0,
                ));

                buttons.push(
                    widgets::Button::new(game.tech_tree.icons[index].clone())
                        .position(tech_position)
                        .size(TECH_BUTTON_SIZE),
                );
            }

            for (child_pos, parent_pos) in lines {
                canvas.line(
                    child_pos + group_pos + scroll + 0.5 * TECH_BUTTON_SIZE,
                    parent_pos + group_pos + scroll + 0.5 * TECH_BUTTON_SIZE,
                    BLACK,
                )
            }

            for back in backs {
                canvas.rect(back, BLACK, WHITE);
            }

            for (index, button) in buttons.into_iter().enumerate() {
                if button.ui(ui) {
                    selected_tech = Some(index);
                }
            }

            ui.label(vec2(0.0, 2.0 * screen_height() - 80.0), "test");
        });

    selected_tech
}

fn tech_info(ui: &mut Ui, game: &mut Game, tech_index: usize) {
    let window_width = 0.25 * screen_width();
    let action_pos = vec2(
        0.5 * (window_width - ACTION_BUTTON_SIZE.x),
        screen_height() - ACTION_BUTTON_SIZE.y - 120.0,
    );
    let tech_name = &game.tech_tree.names[tech_index];
    let tech_desc = &game.tech_tree.descs[tech_index];
    let tech_cost = &game.tech_tree.costs[tech_index];

    ui.label(None, tech_name);
    ui.label(None, " ");
    print_multiline(ui, tech_desc, window_width);

    if !game.tech_tree.available(tech_index) {
        ui.label(None, " ");
        ui.label(None, "Required:");
        for parent in game.tech_tree.requs[tech_index].as_ref().unwrap() {
            ui.label(None, parent);
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
        .size(ACTION_BUTTON_SIZE)
        .ui(ui)
        && &game.resources.research >= tech_cost
    {
        game.tech_tree.obtained[tech_index] = true;
        game.resources.research -= tech_cost;
    }
}

fn print_multiline(ui: &mut Ui, text: &str, width: f32) {
    let mut acc: f32 = 0.0;
    ui.label(None, ""); // start at new line
    for word in text.split_inclusive(' ') {
        let word_size = ui.calc_size(word).x;
        if acc + word_size > width {
            ui.label(None, word); // new line
            acc = 0.0;
        } else {
            ui.same_line(acc);
            ui.label(None, word);
        }
        acc += word_size;
    }
}
