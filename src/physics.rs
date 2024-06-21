use crate::{utils::*, Game, GameState};
use macroquad::prelude::*;
// use std::f32::consts;

const PHYSICS_TICK: f32 = 0.001;

pub struct Physics {
    time_acc:            f32,
    player_displacement: Vec2,
}

impl Physics {
    pub async fn init() -> Physics {
        Physics {
            time_acc:            0.0,
            player_displacement: Vec2::ZERO,
        }
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
                game.player.acceleration +=
                    Vec2::from_angle(game.player.rotation) * game.player.move_speed;
            }
            if is_key_down(KeyCode::S) {
                game.player.acceleration -=
                    Vec2::from_angle(game.player.rotation) * game.player.move_speed;
            }
            if is_key_down(KeyCode::A) {
                game.player.rotation += 0.001;
            }
            if is_key_down(KeyCode::D) {
                game.player.rotation -= 0.001;
            }

            if !game.trebuchet.run(PHYSICS_TICK) {
                game.player.position = game.trebuchet.projectile_position();
                game.player.velocity = game.trebuchet.v_projectile();
                game.player.rotation =
                    (game.trebuchet.sling_point() - game.trebuchet.armsling_point()).to_angle();
                continue;
            }

            // do_physics(game, PHYSICS_TICK);

            game.player.acceleration += game.world.grativy_at(game.player.position);
            let displacement = (game.player.velocity * PHYSICS_TICK)
                + 0.5 * game.player.acceleration * PHYSICS_TICK.powi(2);
            self.player_displacement += displacement;

            let next_position = game.player.position + to_i64coords(self.player_displacement);
            let next_gravity = game.world.grativy_at(next_position);
            game.player.velocity += 0.5 * (game.player.acceleration + next_gravity) * PHYSICS_TICK;
            game.player.acceleration = Vec2::ZERO;

            game.stats.time += PHYSICS_TICK;
            game.stats.distance += displacement.length();
            game.stats.max_altitude = game
                .stats
                .max_altitude
                .max(game.world.altitude_at(game.player.position));
            game.stats.max_speed = game.stats.max_speed.max(game.player.velocity.length());

            if let Some(point) = ground_collision(game, displacement) {
                game.player.position = point;
                game.player.velocity = Vec2::ZERO;
                game.state = GameState::Landed;
                self.player_displacement = Vec2::ZERO;
                break;
            }
        }

        let final_displacement: I64Vec2;
        (final_displacement, self.player_displacement) =
            to_i64coords_with_rem(self.player_displacement);
        game.player.position += final_displacement;
    }
}

fn ground_collision(game: &Game, displacement: Vec2) -> Option<I64Vec2> {
    let circ = game.world.height_map.len();
    let terrain_index = game.world.terrain_index_beneath(game.player.position);
    let terrain_a = game.world.surface(terrain_index);
    let terrain_b = game.world.surface((terrain_index + 1) % circ);

    let next_position = game.player.position + to_i64coords(displacement);

    // If player over terrain at next position
    if orientation(terrain_a, terrain_b, next_position) == Orientation::Clockwise {
        return None;
    }
    get_intersection(terrain_a, terrain_b, game.player.position, next_position)
}

fn _do_physics(game: &mut Game, tick: f32) {
    game.player.acceleration += game.world.grativy_at(game.player.position);

    let displacement =
        (game.player.velocity * tick) + 0.5 * game.player.acceleration * tick.powi(2);
    let next_position = game.player.position + to_i64coords(displacement);

    let circ = game.world.height_map.len();
    let terrain_index = game.world.terrain_index_beneath(game.player.position);
    let terrain_a = game.world.surface(terrain_index);
    let terrain_b = game.world.surface((terrain_index + 1) % circ);

    // If player under terrain at next position
    if orientation(terrain_a, terrain_b, next_position) != Orientation::Clockwise {
        game.player.position =
            get_intersection(terrain_a, terrain_b, game.player.position, next_position)
                .unwrap_or(game.player.position);
        game.player.velocity = Vec2::ZERO;
        game.player.acceleration = Vec2::ZERO;

        game.state = GameState::Landed;

        return;
    }

    game.player.position = next_position;
    let next_acceleration = game.world.grativy_at(next_position);
    game.player.velocity += 0.5 * (game.player.acceleration + next_acceleration) * PHYSICS_TICK;
    game.player.acceleration = Vec2::ZERO;

    game.stats.time += PHYSICS_TICK;
    game.stats.distance += displacement.length();
    game.stats.max_altitude = game
        .stats
        .max_altitude
        .max(game.world.altitude_at(game.player.position));
    game.stats.max_speed = game.stats.max_speed.max(game.player.velocity.length())
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Clockwise,
    AntiClockwise,
    Colinear,
}

/// Orientation of ordered points
fn orientation(p: I64Vec2, q: I64Vec2, r: I64Vec2) -> Orientation {
    match (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y) {
        o if o > 0 => Orientation::Clockwise,
        o if o < 0 => Orientation::AntiClockwise,
        0 => Orientation::Colinear,
        _ => unreachable!(),
    }
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

fn to_i64coords_with_rem(f32coords: Vec2) -> (I64Vec2, Vec2) {
    let i64coords = (f32coords * 256.0).floor().as_i64vec2();
    (i64coords, f32coords - i64coords.as_vec2() / 256.0)
}

#[cfg(test)]
mod physics_test {
    use super::to_i64coords_with_rem;
    use macroquad::math::{I64Vec2, Vec2};

    #[macroquad::test("Test")]
    async fn i64remainder() {
        let mut ami = Vec2::splat(0.5);
        let mut cute = (I64Vec2::splat(128), Vec2::ZERO);
        assert_eq!(to_i64coords_with_rem(ami), cute);

        let love = Vec2::splat(1.0 / 512.0);
        ami += love;
        cute.1 += love;
        assert_eq!(to_i64coords_with_rem(ami), cute);
    }
}
