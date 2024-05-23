use super::*;

const VIEW_RADIUS: f32 = 30000.0; // meters

pub fn i64coords_to_screen(game: &Game, point: I64Vec2) -> Vec2 {
    to_f32coords(point - game.render_space.position) + game.camera.target
}

#[derive(Default)]
pub struct RenderSpace {
    pub position: I64Vec2,
    radius:       f32,
}

impl RenderSpace {
    pub fn new() -> Self {
        Self {
            position: I64Vec2::new(0, 0),
            radius:   VIEW_RADIUS * 256.0,
        }
    }
    pub fn within(&self, point: I64Vec2) -> bool {
        (self.position.distance_squared(point) as f32).sqrt() < self.radius
    }

    pub fn draw(&self) {
        draw_circle_lines(
            GAME_SIZE_X / 2.0,
            GAME_SIZE_Y / 2.0,
            self.radius / 256.0,
            50.0,
            RED,
        );
    }
}

pub fn draw(game: &Game) {
    set_camera(&game.camera);

    //Draw & Clear Background
    clear_background(SKYBLUE);

    // Render Terrain
    //let surface = &game.world.terrain.upper;
    let c = game.world.terrain.circumference;
    let terrain_idx = terrain_idx_beneath(game);
    /*
    let l_bound = (surface
        .iter()
        .cycle()
        .skip(terrain_idx)
        .position(|p| !game.render_space.within(p))
        .unwrap()
        + terrain_idx)
        % c;
    let r_bound = (terrain_idx + c
        - surface
            .iter()
            .rev()
            .cycle()
            .skip(c - terrain_idx)
            .position(|p| !game.render_space.within(p))
            .unwrap())
        % c;
    */
    //println!("{}, {}", l_bound, r_bound);
    let mut active_terrain_idx = vec![terrain_idx];

    // how to double your lines of code with iters

    let mut i = (terrain_idx + 1) % c;
    while game.render_space.within(game.world.terrain.upper[i]) {
        active_terrain_idx.push(i);
        i = (i + 1) % c;
    }
    i = (terrain_idx + c - 1) % c;
    while game.render_space.within(game.world.terrain.upper[i]) {
        active_terrain_idx.push(i);
        i = (i + c - 1) % c;
    }

    active_terrain_idx.into_iter().for_each(|point_idx| {
        let u1 = game.world.terrain.upper[point_idx];
        let l1 = game.world.terrain.lower[point_idx];
        let u2 = game.world.terrain.upper[(point_idx + 1) % c];
        let l2 = game.world.terrain.lower[(point_idx + 1) % c];

        draw_triangle(
            i64coords_to_screen(game, u1),
            i64coords_to_screen(game, u2),
            i64coords_to_screen(game, l1),
            GREEN,
        );
        draw_triangle(
            i64coords_to_screen(game, l1),
            i64coords_to_screen(game, l2),
            i64coords_to_screen(game, u2),
            DARKGREEN,
        );
    });

    game.trebuchet.draw(game);
    // Placeholder player
    let player_pos = i64coords_to_screen(game, game.player.position);
    draw_circle(player_pos.x, player_pos.y, 0.08, PINK);

    let closest_point = i64coords_to_screen(game, game.world.terrain.upper[terrain_idx]);
    draw_circle(closest_point.x, closest_point.y, 6.0, RED);
    game.render_space.draw();

    draw_to_screen(game);
}

pub fn draw_to_screen(game: &Game) {
    // Set Default Camera
    set_default_camera();

    // Draw Game on Screen
    clear_background(BLACK);
    draw_texture_ex(
        &game.render_target.texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            flip_y: false,
            ..Default::default()
        },
    );
}
