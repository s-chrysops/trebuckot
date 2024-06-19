use super::render_space::RenderSpace;
use crate::world::World;
use macroquad::prelude::*;

const TERRAIN_DEPTH: f32 = -50_000.0;
const MAX_SEA_DEPTH: f32 = -10_000.0;

pub fn draw_world(render_space: &RenderSpace, world: &World, material: &Material) {
    let circ = world.terrain.circ;
    let terrain_idx = world.get_terrain_idx_beneath(render_space.position);

    let range = ((render_space.radius.powi(2) - world.get_altitude(render_space.position).powi(2))
        .sqrt()
        / 1000.0) as usize
        + 24;

    let l_bound = (range + terrain_idx) % circ;
    let r_bound = (circ + terrain_idx - range) % circ;

    let active_indicies: Vec<usize> = if r_bound > l_bound {
        (r_bound..circ).chain(0..l_bound).collect()
    } else {
        (r_bound..l_bound).collect()
    };

    for index in active_indicies.iter() {
        let next_index = (index + 1) % circ;

        if world.terrain.height_map[*index].is_sign_positive()
            && world.terrain.height_map[next_index].is_sign_positive()
        {
            continue;
        }
        
        let surface_a = world.get_from_sealevel(*index, 0.0);
        let surface_b = world.get_from_sealevel(next_index, 0.0);

        let bottom_a = world.get_from_sealevel(*index, MAX_SEA_DEPTH);
        let bottom_b = world.get_from_sealevel(next_index, MAX_SEA_DEPTH);

        draw_quadrilateral(
            render_space.to_screen(surface_b),
            render_space.to_screen(surface_a),
            render_space.to_screen(bottom_a),
            render_space.to_screen(bottom_b),
            BLUE,
        );
    }

    for index in active_indicies.iter() {
        let next_index = (index + 1) % circ;

        let surface_a = world.get_terrain(*index);
        let surface_b = world.get_terrain(next_index);

        let bottom_a = world.get_from_sealevel(*index, TERRAIN_DEPTH);
        let bottom_b = world.get_from_sealevel(next_index, TERRAIN_DEPTH);

        gl_use_material(material);
        material.set_uniform("EdgeColor", vec4(0.253, 0.924, 0.039, 1.0));
        material.set_uniform("InnerColor", vec4(0.273, 0.168, 0.148, 1.0));
        draw_quadrilateral(
            render_space.to_screen(surface_b),
            render_space.to_screen(surface_a),
            render_space.to_screen(bottom_a),
            render_space.to_screen(bottom_b),
            GREEN,
        );
        gl_use_default_material();
    }
}

// Draw quadrilateral from four points starting at the top-left corner proceeding clockwise
fn draw_quadrilateral(a: Vec2, b: Vec2, c: Vec2, d: Vec2, color: Color) {
    let context = unsafe { get_internal_gl() };

    let vertices = [
        Vertex::new(a.x, a.y, 0., 0.0, 0.0, color),
        Vertex::new(b.x, b.y, 0., 1.0, 0.0, color),
        Vertex::new(c.x, c.y, 0., 1.0, 1.0, color),
        Vertex::new(d.x, d.y, 0., 0.0, 1.0, color),
    ];
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

    context.quad_gl.texture(None);
    context.quad_gl.draw_mode(DrawMode::Triangles);
    context.quad_gl.geometry(&vertices, &indices);
}
