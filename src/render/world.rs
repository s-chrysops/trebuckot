use super::render_space::RenderSpace;
use crate::world::World;
use macroquad::prelude::*;

const TERRAIN_DEPTH: f32 = 100_000.0;

pub fn draw_world(render_space: &RenderSpace, world: &World) {
    let circ = world.terrain.circ;
    let terrain_idx = world.get_terrain_idx_beneath(render_space.position);

    let range = ((render_space.radius.powi(2) - world.get_altitude(render_space.position).powi(2))
        .sqrt()
        / 1000.0) as usize;

    let l_bound = (range + terrain_idx) % circ;
    let r_bound = (circ + terrain_idx - range) % circ;

    let active_indicies: Vec<usize> = if r_bound > l_bound {
        (r_bound..circ).chain(0..l_bound).collect()
    } else {
        (r_bound..l_bound).collect()
    };

    active_indicies.into_iter().for_each(|index| {
        let next_index = (index + 1) % circ;

        let surface_a = world.get_terrain_at(index);
        let surface_b = world.get_terrain_at(next_index);

        let bottom_a = world.get_sealevel_at(index, Some(TERRAIN_DEPTH));
        let bottom_b = world.get_sealevel_at(next_index, Some(TERRAIN_DEPTH));

        let sealevel_a = world.get_sealevel_at(index, None);
        let sealevel_b = world.get_sealevel_at(next_index, None);

        // Draw water
        draw_triangle(
            render_space.to_screen(sealevel_a),
            render_space.to_screen(sealevel_b),
            render_space.to_screen(bottom_a),
            BLUE,
        );
        draw_triangle(
            render_space.to_screen(bottom_a),
            render_space.to_screen(bottom_b),
            render_space.to_screen(sealevel_b),
            DARKBLUE,
        );

        draw_triangle(
            render_space.to_screen(surface_a),
            render_space.to_screen(surface_b),
            render_space.to_screen(bottom_a),
            GREEN,
        );
        draw_triangle(
            render_space.to_screen(bottom_a),
            render_space.to_screen(bottom_b),
            render_space.to_screen(surface_b),
            DARKGREEN,
        );
    });
}
