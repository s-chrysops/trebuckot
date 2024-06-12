use macroquad::prelude::*;
use crate::{world::World, utils::*};
use super::render_space::RenderSpace;

const TERRAIN_DEPTH: f32 = 100_000.0;

pub fn draw_world(render_space: &RenderSpace, world: &World) {
    let surface = &world.terrain.surface;
    let circ = world.terrain.circ;
    let radius_bot = world.radius - TERRAIN_DEPTH;
    let terrain_idx = world.get_terrain_idx_beneath(render_space.position);

    let l_scan = surface
        .iter()
        .cycle()
        .skip(terrain_idx)
        .position(|p| !render_space.within(*p))
        .unwrap();
    let r_scan = surface
        .iter()
        .rev()
        .cycle()
        .skip(circ - terrain_idx)
        .position(|p| !render_space.within(*p))
        .unwrap();
    let l_bound = (l_scan + terrain_idx) % circ;
    let r_bound = (circ + terrain_idx - r_scan) % circ;

    let active: Vec<usize> = if r_bound > l_bound {
        (r_bound..circ).chain(0..l_bound).collect()
    } else {
        (r_bound..l_bound).collect()
    };

    active.into_iter().for_each(|current_idx| {
        let next_idx = (current_idx + 1) % circ;
        let u1 = world.terrain.surface[current_idx];
        let u2 = world.terrain.surface[next_idx];

        let l1 = to_i64coords(polar_to_cartesian(
            radius_bot,
            current_idx as f32 * 1000.0 / world.radius,
        )) + world.position;
        let l2 = to_i64coords(polar_to_cartesian(
            radius_bot,
            next_idx as f32 * 1000.0 / world.radius,
        )) + world.position;

        let s1 = to_i64coords(polar_to_cartesian(
            world.radius,
            current_idx as f32 * 1000.0 / world.radius,
        )) + world.position;
        let s2 = to_i64coords(polar_to_cartesian(
            world.radius,
            next_idx as f32 * 1000.0 / world.radius,
        )) + world.position;

        // Draw water
        draw_triangle(
            render_space.to_screen(s1),
            render_space.to_screen(s2),
            render_space.to_screen(l1),
            BLUE,
        );
        draw_triangle(
            render_space.to_screen(l1),
            render_space.to_screen(l2),
            render_space.to_screen(s2),
            DARKBLUE,
        );

        // Draw terrain
        draw_triangle(
            render_space.to_screen(u1),
            render_space.to_screen(u2),
            render_space.to_screen(l1),
            GREEN,
        );
        draw_triangle(
            render_space.to_screen(l1),
            render_space.to_screen(l2),
            render_space.to_screen(u2),
            DARKGREEN,
        );
    });
}