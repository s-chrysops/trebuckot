use super::*;

pub fn do_physics(game: &mut Game, tick: f32) {
    let terrain_a = game.world.terrain.upper[{
        let slice = game.world.terrain.circumference as f32 / consts::TAU;
        let angle = to_angle(to_f32coords(
            game.render_space.position - game.world.position,
        ));
        (slice * angle % game.world.terrain.circumference as f32) as usize
    }];
    let terrain_b = game.world.terrain.upper
        [(({
            let slice = game.world.terrain.circumference as f32 / consts::TAU;
            let angle = to_angle(to_f32coords(
                game.render_space.position - game.world.position,
            ));
            (slice * angle % game.world.terrain.circumference as f32) as usize
        }) + 1) % game.world.terrain.circumference];

    // Apply gravity if player above terrain
    if orientation(terrain_a, terrain_b, game.player.position) == 1 {
        game.player.acceleration += game.player.get_grativy(&game.world);
    }

    let displacement =
        to_i64coords((game.player.velocity * tick) + 0.5 * game.player.acceleration * tick.powi(2));

    if do_intersect(
        terrain_a,
        terrain_b,
        game.player.position,
        game.player.position + displacement,
    ) {
        game.player.position = get_intersection(
            terrain_a,
            terrain_b,
            game.player.position,
            game.player.position + displacement,
        )
        .unwrap();
        game.player.velocity = vec2(0.0, 0.0);
    } else {
        game.player.position += displacement;

        // Leapfrog intergration
        let new_acceleration = game.player.get_grativy(&game.world);
        game.player.velocity += 0.5 * (game.player.acceleration + new_acceleration) * tick;
    }

    game.player.acceleration = vec2(0.0, 0.0);
}

// General case do line segments (a, b), (c, d) intersect
// Does NOT check for Special case (colinearity)
fn do_intersect(a: I64Vec2, b: I64Vec2, c: I64Vec2, d: I64Vec2) -> bool {
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);
    o1 != o2 && o3 != o4
}

// Orientation of ordered points
// clockwise        ->  1
// anti-clockwise   -> -1
// colinear         ->  0
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
        None
    } else {
        let dets = I64Vec2::new(a.perp_dot(b), c.perp_dot(d)).as_dvec2();
        let x = dets.perp_dot(xdiff) / div;
        let y = dets.perp_dot(ydiff) / div;
        Some(I64Vec2::new(x.round() as i64, y.round() as i64))
    }
}
