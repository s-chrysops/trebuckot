use crate::{Game, GameState, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};

const BUTTON_SIZE: Vec2 = vec2(200.0, 40.0);

#[derive(Debug, Clone, Copy, Default)]
pub struct UpgradesState {
    tab:  u32,
    tech: Option<usize>,
}

// #[derive(Debug, Clone, Copy, Default, PartialEq)]
// enum UpgradesTab {
//     #[default]
//     Research,
//     Trebuchet,
//     Player,
//     Logs,
// }

// impl From<u32> for UpgradesTab {
//     fn from(value: u32) -> Self {
//         match value {
//             0 => UpgradesTab::Research,
//             1 => UpgradesTab::Trebuchet,
//             2 => UpgradesTab::Player,
//             3 => UpgradesTab::Logs,
//             _ => unreachable!(),
//         }
//     }
// }

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
        // let mut tab_proxy = 0;
        match widgets::Tabbar::new(hash!(), vec2(window_width, 60.0), &tabs)
            .selected_tab(Some(&mut state.tab))
            .ui(ui)
        {
            // Research
            0 => match state.tech {
                Some(tech_index) => tech_menu(ui, game, tech_index),
                None => ui.label(None, "No tech selected"),
            },
            // Trebuchet
            1 => ui.label(None, &format!("{:?}", game.state)),
            // Player
            2 => {}
            // Logs
            3 => {}
            _ => unreachable!(),
        };
        if widgets::Button::new("Close")
            .position(vec2(
                (window_width - BUTTON_SIZE.x) / 2.0,
                screen_height() - BUTTON_SIZE.y - 60.0,
            ))
            .size(BUTTON_SIZE)
            .ui(ui)
        {
            game.state = GameState::PreLaunch;
            next_scene = Some(Scene::PreLaunch);
        };
        // state.tab = UpgradesTab::from(tab_proxy);
    });

    // Tech Tree Selector
    if state.tab == 0 {
        let button_size = Vec2::splat(80.0);
        let window_size = vec2(screen_width() * 0.75, 2.0 * screen_height() - 60.0);
        let offset = vec2(screen_width() * 0.25, 60.0);
        widgets::Group::new(hash!(), window_size)
            .position(vec2(window_width, 60.0))
            .ui(&mut root_ui(), |ui| {
                let scroll = ui.scroll();
                for index in (0..game.tech_tree.names.len()).rev() {
                    let mut canvas = ui.canvas();
                    let tech_position = game.tech_tree.spots[index] * window_size;
                    if let Some(requs) = &game.tech_tree.requs[index] {
                        for requ in requs {
                            let requ_index = game.tech_tree.get_index(requ);
                            let requ_position = game.tech_tree.spots[requ_index] * window_size;
                            canvas.line(
                                tech_position + offset + scroll + button_size / 2.0,
                                requ_position + offset + scroll + button_size / 2.0,
                                BLACK,
                            );
                        }
                    }

                    canvas.rect(
                        Rect::new(
                            tech_position.x + offset.x,
                            tech_position.y + offset.y + scroll.y,
                            80.0,
                            80.0,
                        ),
                        BLACK,
                        WHITE,
                    );

                    if widgets::Button::new(game.tech_tree.icons[index].clone())
                        .position(tech_position)
                        .size(button_size)
                        .ui(ui)
                    {
                        state.tech = Some(index)
                    }
                }
                ui.label(vec2(0.0, 3.0 * screen_height() - 80.0), "test")
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
        .size(BUTTON_SIZE)
        .ui(ui)
        && &game.resources.research >= tech_cost
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
