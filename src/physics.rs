use crate::{trebuchet, utils::*, Game, GameState};
use macroquad::prelude::*;

const PHYSICS_TICK: f32 = 0.001;

pub struct Physics {
    time_acc: f32,
}

impl Physics {
    pub async fn init() -> Physics {
        Physics { time_acc: 0.0 }
    }

    pub fn update(&mut self, game: &mut Game) {
        if is_key_down(KeyCode::Escape) && game.state == GameState::Launched {
            game.state = GameState::Paused;
        }
        if game.state != GameState::Launched {
            return;
        }

        self.time_acc += get_frame_time();
        while self.time_acc > PHYSICS_TICK {
            self.time_acc -= PHYSICS_TICK;

            // Basic movement
            if is_key_down(KeyCode::W) {
                game.player.acceleration.y += game.player.move_speed;
            }
            if is_key_down(KeyCode::S) {
                game.player.acceleration.y -= game.player.move_speed;
            }
            if is_key_down(KeyCode::A) {
                game.player.acceleration.x -= game.player.move_speed;
            }
            if is_key_down(KeyCode::D) {
                game.player.acceleration.x += game.player.move_speed;
            }

            game.trebuchet.run(PHYSICS_TICK);
            if game.trebuchet.state != trebuchet::TrebuchetState::Stage3 {
                game.player.position = game.trebuchet.projectile_position();
                game.player.velocity = game.trebuchet.v_projectile();
                continue;
            }

            do_physics(game, PHYSICS_TICK);
        }
    }
}

fn do_physics(game: &mut Game, tick: f32) {
    let terrain_idx = game.world.get_terrain_idx_beneath(game.player.position);
    let terrain_a = game.world.terrain.surface[terrain_idx];
    let terrain_b = game.world.terrain.surface[(terrain_idx + 1) % game.world.terrain.circ];

    // Apply gravity if player above terrain
    if orientation(terrain_a, terrain_b, game.player.position) == 1 {
        game.player.acceleration += game.world.get_grativy(game.player.position);
    }

    let displacement =
        to_i64coords((game.player.velocity * tick) + 0.5 * game.player.acceleration * tick.powi(2));
    let next_position = game.player.position + displacement;

    if orientation(terrain_a, terrain_b, next_position) != 1 {
        game.player.position =
            get_intersection(terrain_a, terrain_b, game.player.position, next_position)
                .unwrap_or(game.player.position);
        game.player.velocity = Vec2::ZERO;
        game.player.acceleration = Vec2::ZERO;

        game.state = GameState::Landed;

        return;
    }

    game.player.position = next_position;

    // Leapfrog intergration
    let next_acceleration = game.world.get_grativy(next_position);
    game.player.velocity += 0.5 * (game.player.acceleration + next_acceleration) * tick;

    game.player.acceleration = Vec2::ZERO;

    game.stats.time += PHYSICS_TICK;
    game.stats.distance += to_meters(displacement).length();
    game.stats.max_altitude = game
        .stats
        .max_altitude
        .max(game.world.get_altitude(game.player.position));
    game.stats.max_speed = game.stats.max_speed.max(game.player.velocity.length())
}

// General case do line segments (a, b), (c, d) intersect
// Does NOT check for Special case (colinearity)
// fn do_intersect(a: I64Vec2, b: I64Vec2, c: I64Vec2, d: I64Vec2) -> bool {
//     let o1 = orientation(a, b, c);
//     let o2 = orientation(a, b, d);
//     let o3 = orientation(c, d, a);
//     let o4 = orientation(c, d, b);
//     o1 != o2 && o3 != o4
// }

/// Orientation of ordered points
///
/// clockwise        ->  1
///
/// anti-clockwise   -> -1
///
/// colinear         ->  0
fn orientation(p: I64Vec2, q: I64Vec2, r: I64Vec2) -> i8 {
    ((q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)).signum() as i8
}

// | |a-b| a-b |
// | |c-d| c-d |
// -------------
//  | a-b c-d |
fn get_intersection(a: I64Vec2, b: I64Vec2, c: I64Vec2, d: I64Vec2) -> Option<I64Vec2> {
    let xdiff = I64Vec2::new(a.x - b.x, c.x - d.x).as_dvec2();
    let ydiff = I64Vec2::new(a.y - b.y, c.y - d.y).as_dvec2();
    let div = xdiff.perp_dot(ydiff);
    if div == 0.0 {
        return None;
    }
    let dets = I64Vec2::new(a.perp_dot(b), c.perp_dot(d)).as_dvec2();
    let x = dets.perp_dot(xdiff) / div;
    let y = dets.perp_dot(ydiff) / div;
    Some(I64Vec2::new(x.round() as i64, y.round() as i64))
}
