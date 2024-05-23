use super::*;
use macroquad::ui::{hash, root_ui, widgets};

pub fn get_screen() -> Vec2 {
    vec2(screen_width(), screen_height())
}

// meters to i64 coordinates
pub fn to_i64coords(f32coords: Vec2) -> I64Vec2 {
    I64Vec2::new(
        (f32coords.x * 256.0).round() as i64,
        (f32coords.y * 256.0).round() as i64,
    )
}

// i64 coordinates to meters
pub fn to_f32coords(i64coords: I64Vec2) -> Vec2 {
    Vec2::new((i64coords.x as f32) / 256.0, (i64coords.y as f32) / 256.0)
}

pub fn to_angle(coords: Vec2) -> f32 {
    let mut theta = coords.y.atan2(coords.x);
    if theta < 0.0 {
        theta += consts::TAU;
    }
    theta
}

#[derive(PartialEq, Debug)]
enum GameState {
    MainMenu,
    Paused,
    PreLaunch,
    Launched,
    Landed,
    Scene,
}

pub struct Game {
    pub time_frame:  f32,
    pub time_launch: f32,

    pub camera:        Camera2D,
    pub render_space:  RenderSpace,
    pub render_target: RenderTarget,

    state:      GameState,
    pub player: Player,
    pub world:  World,

    pub trebuchet: Trebuchet,
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
        let mut camera = Camera2D::from_display_rect(camera_rect);
        camera.zoom *= 100.0;
        camera.render_target = Some(render_target.clone());
        set_camera(&camera);

        let render_space = RenderSpace::new();

        // BEarth
        let world = World::new(
            IVec2::new(0, 0),
            I64Vec2::new(0, 0),
            6_371_000.0,
            5.972e+24,
            vec![0.0; 40030],
            WorldClass::Minshara,
        );

        let t_arm = (8.0, 2.0, 12.0);
        let t_weight = (2.0, 100.0);
        let t_sling = 7.0;
        let trebuchet = Trebuchet::new(
            I64Vec2::new(0, 1_630_976_000),
            5.0,
            t_arm,
            t_weight,
            t_sling,
            0.3,
        );

        Self {
            time_frame: 0.0,
            time_launch: 0.0,
            camera,
            render_space,
            render_target,
            state: GameState::MainMenu,
            player: Player::new(trebuchet.projectile_position()),
            world,
            trebuchet,
        }
    }

    pub fn run(&mut self) {
        match self.state {
            GameState::MainMenu => {
                widgets::Window::new(hash!(), vec2(0.0, 0.0), get_screen())
                    .titlebar(false)
                    .movable(false)
                    .ui(&mut root_ui(), |ui| {
                        if widgets::Button::new("START")
                            .position(get_screen() / 2.0 - vec2(100.0, 25.0))
                            .size(vec2(200.0, 50.0))
                            .ui(ui)
                        {
                            self.state = GameState::PreLaunch;
                        }
                    });
            }

            GameState::Paused => {
                widgets::Popup::new(hash!(), get_screen()).ui(&mut root_ui(), |ui| {
                    if widgets::Button::new("CONTINUE")
                        .position(get_screen() / 2.0 - vec2(100.0, 25.0))
                        .size(vec2(200.0, 50.0))
                        .ui(ui)
                    {
                        self.state = GameState::Launched;
                    }
                });
            }

            GameState::PreLaunch => {
                self.render_space.position = self.player.position;
                if is_key_down(KeyCode::Space) {
                    self.state = GameState::Launched;
                }
            }

            GameState::Launched => {
                // Basic movement
                if is_key_down(KeyCode::W) {
                    self.player.acceleration.y += self.player.move_speed;
                }
                if is_key_down(KeyCode::S) {
                    self.player.acceleration.y -= self.player.move_speed;
                }
                if is_key_down(KeyCode::A) {
                    self.player.acceleration.x -= self.player.move_speed;
                }
                if is_key_down(KeyCode::D) {
                    self.player.acceleration.x += self.player.move_speed;
                }
                if is_key_down(KeyCode::Escape) {
                    self.state = GameState::Paused;
                }

                match mouse_wheel() {
                    (_x, y) if y != 0.0 => {
                        self.camera.zoom *= 10.0_f32.powf(y.signum() / 4.0);
                    }
                    _ => (),
                }

                self.time_frame += get_frame_time();
                while self.time_frame > PHYSICS_TICK {
                    if self.trebuchet.state == TrebuchetState::Stage3 {
                        do_physics(self, PHYSICS_TICK);
                    }
                    self.trebuchet.run(PHYSICS_TICK);
                    self.time_launch += PHYSICS_TICK;
                    self.time_frame -= PHYSICS_TICK;
                }
                if self.trebuchet.state != TrebuchetState::Stage3 {
                    self.player.position = self.trebuchet.projectile_position();
                    self.player.velocity = self.trebuchet.v_projectile();
                    self.render_space.position = self.trebuchet.position;
                } else {
                    self.render_space.position = self.player.position;
                }
            }

            GameState::Landed => {
                let ami = 1337;
                let cute = 1337;
                assert_eq!(ami, cute);
            }

            GameState::Scene => {
                let ami = 1337;
                let cute = 1337;
                assert_eq!(ami, cute);
            }
        }

        draw(self);
        self.camera.rotation =
            90.0 - to_angle(to_f32coords(self.player.position - self.world.position)).to_degrees();
    }
}
