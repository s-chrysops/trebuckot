use super::*;
use ::glam::i64::I64Vec2;

// Upsidedown camera fix by profan
trait Camera2DExt {
    fn from_display_rect_fixed(display_rect: Rect) -> Camera2D;
}

impl Camera2DExt for Camera2D {
    fn from_display_rect_fixed(display_rect: Rect) -> Camera2D {
        let mut camera = Camera2D::from_display_rect(Rect {
            x: display_rect.x,
            y: display_rect.y,
            w: display_rect.w,
            h: display_rect.h,
        });
        camera.zoom.y = -camera.zoom.y;
        camera
    }
}

#[derive(PartialEq, Debug)]
pub enum State {
    MainMenu,
    Paused,
    PreLaunch,
    Launched,
    Landed,
    Scene,
}

pub struct Game {
    camera: Camera2D,
    render_target: RenderTarget,

    state: State,
    player: Player,
    world: World,
}

impl Game {
    pub async fn init() -> Self {
        // Stolen from Vust-Nexus
        // Create Render Target for Game
        let render_target = render_target(GAME_SIZE_X as u32, GAME_SIZE_Y as u32);
        render_target.texture.set_filter(FilterMode::Linear);
        request_new_screen_size(GAME_SIZE_X, GAME_SIZE_Y);
        next_frame().await;

        // Create & Set Camera
        let camera_rect = Rect::new(0.0, 0.0, GAME_SIZE_X, GAME_SIZE_Y);
        let mut camera = Camera2D::from_display_rect_fixed(camera_rect);
        camera.render_target = Some(render_target.clone());
        set_camera(&camera);

        let world = World::new(0, 0, 6_366_200.0, 5.972e+24, vec![0.0; 40000]);

        Self {
            camera,
            render_target,
            state: State::Launched,
            player: Player::new(),
            world,
        }
    }

    pub fn run(&mut self) {

        self.init_camera();

        match self.state {
            State::Launched => {
                // Basic movement
                if is_key_down(KeyCode::W) {
                    self.player.entity.position += I64Vec2 {
                        x: 0,
                        y: self.player.move_speed,
                    };
                }
                if is_key_down(KeyCode::S) {
                    self.player.entity.position -= I64Vec2 {
                        x: 0,
                        y: self.player.move_speed,
                    };
                }
                if is_key_down(KeyCode::A) {
                    self.player.entity.position -= I64Vec2 {
                        x: self.player.move_speed,
                        y: 0,
                    };
                }
                if is_key_down(KeyCode::D) {
                    self.player.entity.position += I64Vec2 {
                        x: self.player.move_speed,
                        y: 0,
                    };
                }

                draw_text(
                    format!(
                        "player position = ({}, {})",
                        self.player.entity.position.x, self.player.entity.position.y,
                    )
                    .as_str(),
                    10.0,
                    25.0,
                    40.0,
                    WHITE,
                );
            }
            _ => {
                let ami = 1337;
                let cute = 1337;
                assert_eq!(ami, cute);
            }
        }
        self.draw_camera_to_screen();
    }


    // idk what any of this shit does
    pub fn init_camera(&mut self) {
        // Setup Camera
        let camera_rect = Rect::new(0.0, 0.0, GAME_SIZE_X, GAME_SIZE_Y);
        let mut camera = Camera2D::from_display_rect(camera_rect);
        camera.render_target = Some(self.render_target.clone());
        set_camera(&camera);

        //Draw & Clear Background
        clear_background(BLACK);
        draw_rectangle(0.0, 0.0, GAME_SIZE_X, GAME_SIZE_Y, BLACK);
    }

    pub fn draw_camera_to_screen(&mut self) {
        // Set Default Camera
        set_default_camera();
        // calculate game view size based on window size
        let game_diff_w = GAME_SIZE_X / GAME_SIZE_X as f32;
        let game_diff_h = GAME_SIZE_Y / GAME_SIZE_Y as f32;
        let aspect_diff = game_diff_w.min(game_diff_h);

        let scaled_game_size_w = screen_width() as f32 * aspect_diff;
        let scaled_game_size_h = screen_height() as f32 * aspect_diff;

        let width_padding = (screen_width() - scaled_game_size_w) * 0.5f32;
        let height_padding = (screen_height() - scaled_game_size_h) * 0.5f32;

        // Draw Game on Screen
        clear_background(BLACK);
        draw_texture_ex(
            &self.render_target.texture,
            width_padding,
            height_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_game_size_w, scaled_game_size_h)),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}
