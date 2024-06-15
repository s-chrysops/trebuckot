use crate::{Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Ui};

const BUTTON_SIZE: Vec2 = vec2(200.0, 40.0);

pub async fn upgrades(mut tab: u32, mut selected_tech: Option<usize>, game: &mut Game) -> Scene {
    let tabs = ["Research", "Trebuchet", "Player", "Logs"];

    // let mut next_tab = tab;
    // let ui = &mut root_ui();
    let mut next_scene = None;
    widgets::Window::new(
        hash!(),
        Vec2::ZERO.with_y(60.0),
        vec2(screen_width() * 0.25, screen_height() - 60.0),
    )
    .titlebar(false)
    .movable(false)
    .ui(&mut root_ui(), |ui| {
        match widgets::Tabbar::new(hash!(), vec2(screen_width() * 0.25, 60.0), &tabs)
            .selected_tab(Some(&mut tab))
            .ui(ui)
        {
            // Research
            0 => {
                match selected_tech {
                    Some(tech_index) => tech_menu(ui, game, tech_index),
                    None => ui.label(None, "No tech selected"),
                }
            }
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
                (screen_width() * 0.25 - BUTTON_SIZE.x) / 2.0,
                screen_height() - BUTTON_SIZE.y - 60.0,
            ))
            .size(BUTTON_SIZE)
            .ui(ui)
        {
            next_scene = Some(Scene::PreLaunch);
        };
    });

    // Tech Tree Selector
    if tab == 0 {
        widgets::Group::new(hash!(), vec2(screen_width() * 0.75, screen_height() - 60.0))
            .position(vec2(screen_width() * 0.25, 60.0))
            .ui(&mut root_ui(), |ui| {
                for index in 0..game.tech_tree.names.len() {
                    let layout = game.tech_tree.spots[index];
                    if widgets::Button::new(game.tech_tree.icons[index].clone())
                        .position(vec2(layout.0 as f32 * 100.0, layout.1 as f32 * 100.0))
                        .size(Vec2::splat(80.0))
                        .ui(ui)
                    {
                        selected_tech = Some(index)
                    }
                }
            });
    }

    match next_scene {
        Some(scene) => scene,
        None => Scene::Upgrades(tab, selected_tech),
    }
}

fn tech_menu(ui: &mut Ui, game: &mut Game, tech_index: usize) {
    let action_pos = vec2(
        (screen_width() * 0.25 - BUTTON_SIZE.x) / 2.0,
        screen_height() - BUTTON_SIZE.y - 120.0,
    );
    let tech_name = game.tech_tree.names[tech_index].as_str();
    let tech_desc = game.tech_tree.descs[tech_index].as_str();
    let tech_cost = game.tech_tree.costs[tech_index];

    ui.label(None, tech_name);
    ui.label(None, " ");
    print_multiline(ui, tech_desc, screen_width() * 0.25);

    if !game.tech_tree.available(tech_index) {
        ui.label(None, " ");
        ui.label(None, "Required:");
        for parent in game.tech_tree.parents[tech_index].as_ref().unwrap(){
            ui.label(None, parent.as_str());
        }
        return;
    }

    if game.tech_tree.obtained[tech_index] {
        ui.label(None, " ");
        ui.label(None, "Obtained!");
        return;
    }

    if widgets::Button::new(format!("RESEARCH {}", tech_cost))
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
