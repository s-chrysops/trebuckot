use crate::{get_screen, Game, Scene};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub async fn upgrades(mut tab: u32, mut tech: Option<usize>, game: &mut Game) -> Scene {
    let close_button_size = vec2(200.0, 40.0);
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
                match tech {
                    Some(tech) => {
                        let tech_name = &game.tech_tree.names[tech];
                        ui.label(None, tech_name);
                        ui.label(None, &game.tech_tree.get(tech_name).unwrap().description)
                    }
                    None => ui.label(None, "test")
                }
            }
            // Trebuchet
            1 => ui.label(None, &format!("{:?}", game.state)),
            // Player
            2 => {}
            // Logs
            3 => {}
            _ => {}
        };
        if widgets::Button::new("Close")
            .position(vec2(
                (screen_width() * 0.25 - close_button_size.x) / 2.0,
                screen_height() - close_button_size.y - 60.0,
            ))
            .size(close_button_size)
            .ui(ui)
        {
            next_scene = Some(Scene::PreLaunch);
        };
    });

    if tab == 0 {
        widgets::Window::new(
            hash!(),
            vec2(screen_width() * 0.25, 60.0),
            vec2(screen_width() * 0.75, screen_height() - 60.0),
        )
        .titlebar(false)
        .movable(false)
        .ui(&mut root_ui(), |ui| {
            for name in &game.tech_tree.names {
                let layout = game.tech_tree.spot(name).unwrap();
                if widgets::Button::new(name.clone())
                    .position(vec2(layout.0 as f32 * 100.0, layout.1 as f32 * 100.0))
                    .size(Vec2::splat(80.0))
                    .ui(ui){
                        tech = game.tech_tree.idx(name)
                    }
            }
        });
    }

    match next_scene {
        Some(scene) => scene,
        None => Scene::Upgrades(tab, tech),
    }
}
