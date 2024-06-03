use crate::{
    physics::*,
    player::Player,
    resources::*,
    trebuchet::{Trebuchet, TrebuchetState},
    world::*,
};
use ::glam::I64Vec2;
use core::f32::consts;
use macroquad::prelude::*;

const START_POINT: I64Vec2 = I64Vec2::new(0, 1_630_976_000);
const PHYSICS_TICK: f32 = 0.001;

// meters to i64 coordinates
pub fn to_i64coords(f32coords: Vec2) -> I64Vec2 {
    I64Vec2::new(
        (f32coords.x * 256.0).round() as i64,
        (f32coords.y * 256.0).round() as i64,
    )
}

// i64 coordinates to meters
pub fn to_meters(i64coords: I64Vec2) -> Vec2 {
    Vec2::new((i64coords.x as f32) / 256.0, (i64coords.y as f32) / 256.0)
}

pub fn to_angle(coords: Vec2) -> f32 {
    let mut theta = coords.y.atan2(coords.x);
    if theta < 0.0 {
        theta += consts::TAU;
    }
    theta
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum GameState {
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
    pub launched:    bool,

    pub state:     GameState,
    pub trebuchet: Trebuchet,
    pub world:     World,

    pub day:       u32,
    pub resources: Resources,
    pub player:    Player,
}

impl Game {
    pub async fn init() -> Self {
        // BEarth
        let world = World::new(
            IVec2::default(),
            I64Vec2::new(0, 0),
            6_371_000.0,
            5.972e+24,
            WorldClass::Minshara,
        );

        let mut trebuchet = Trebuchet::init(START_POINT).build();
        trebuchet.reset();
        let player = Player::new(trebuchet.projectile_position());
        let resources = Resources::default();

        Self {
            state: GameState::MainMenu,
            time_frame: 0.0,
            time_launch: 0.0,
            launched: false,
            day: 0,

            world,
            trebuchet,
            player,
            resources,
        }
    }

    pub fn update(&mut self) {
        if is_key_released(KeyCode::Space) {
            self.launched = true;
            self.state = GameState::Launched;
        }

        if !self.launched {
            return;
        }

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

        self.time_frame += get_frame_time();
        while self.time_frame > PHYSICS_TICK {
            self.trebuchet.run(PHYSICS_TICK);
            if let TrebuchetState::Stage3 = self.trebuchet.state {
                do_physics(self, PHYSICS_TICK);
            } else {
                self.player.position = self.trebuchet.projectile_position();
                self.player.velocity = self.trebuchet.v_projectile();
            }

            self.time_launch += PHYSICS_TICK;
            self.time_frame -= PHYSICS_TICK;
        }
    }

    pub fn reset(&mut self) {
        self.time_launch = 0.0;
        self.launched = false;
        self.state = GameState::PreLaunch;
        self.trebuchet.reset();
        self.player.position = self.trebuchet.projectile_position();
        self.day += 1;
    }
}
