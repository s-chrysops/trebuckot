use crate::{Game, GameState};
use macroquad::prelude::*;

pub fn draw_hud(game: &Game, font: &Font) {
    match game.state {
        GameState::PreLaunch => {
            // let margin_x = screen_width() / 16.0;
            let margin_y = 48.0;
            let spacing = screen_width() / 6.0;

            let params = TextParams {
                font: Some(font),
                font_size: 48,
                color: WHITE,
                ..Default::default()
            };

            for (i, resource) in game.resources.as_vec().iter().enumerate() {
                if *resource == 0 {
                    continue;
                }
                let fmt = resource.to_string();
                let fmt_width = measure_text(&fmt, Some(font), 48, 1.0).width;
                draw_text_ex(
                    &fmt,
                    (spacing * (i + 1) as f32) - fmt_width,
                    margin_y,
                    params.clone(),
                )
            }
        }

        GameState::Launched => {
            let margin_x = screen_width() / 64.0;
            let margin_y = screen_height() / 32.0;

            let speed_params = TextParams {
                font: Some(font),
                font_size: 48,
                color: WHITE,
                ..Default::default()
            };

            let altitude_params = TextParams {
                font: Some(font),
                font_size: 24,
                color: WHITE,
                ..Default::default()
            };

            let speed = format!("{:.2}m/s", game.player.velocity.length());
            let altitude = format!("{:.0}m", game.world.get_altitude(game.player.position));
            draw_text_ex(&speed, margin_x, margin_y + 24.0, speed_params);
            draw_text_ex(&altitude, margin_x, margin_y + 60.0, altitude_params);
        }

        GameState::Landed => {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                color_u8!(0, 0, 0, 64),
            );

            let margin_x = screen_width() / 4.0;
            let margin_y = screen_height() / 4.0;
            let spacing = 60.0;
            let params = TextParams {
                font: Some(font),
                font_size: 48,
                color: WHITE,
                ..Default::default()
            };

            for (i, stat) in game.stats.as_vec().iter().enumerate() {
                let stat_fmt = format!("{:.2}{}", stat.value, stat.unit);
                let stat_width = measure_text(&stat_fmt, Some(font), 48, 1.0).width;
                draw_text_ex(
                    &stat.field,
                    margin_x,
                    (spacing * i as f32) + margin_y,
                    params.clone(),
                );
                draw_text_ex(
                    &stat_fmt,
                    screen_width() - margin_x - stat_width,
                    (spacing * i as f32) + margin_y,
                    params.clone(),
                );
            }

            let re = "Research Earned";
            let points = game.stats.crunch().to_string();
            let re_width = measure_text(re, Some(font), 48, 1.0).width;
            let points_width = measure_text(&points, Some(font), 48, 1.0).width;
            draw_text_ex(
                re,
                (screen_width() - re_width) / 2.0,
                screen_height() - margin_y - spacing,
                params.clone(),
            );
            draw_text_ex(
                &points,
                (screen_width() - points_width) / 2.0,
                screen_height() - margin_y,
                params.clone(),
            );
        }

        GameState::Paused => {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                color_u8!(0, 0, 0, 64),
            );
        }
    }
}
