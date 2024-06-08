use crate::{player::Player, resources::*, trebuchet::Trebuchet, world::*};
use core::f32::consts;
use macroquad::prelude::*;
use terrain::TerrainClass;

const START_POINT: I64Vec2 = i64vec2(0, 1_631_100_000);

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

#[derive(PartialEq, Debug)]
pub enum GameState {
    MainMenu,
    Paused,
    PreLaunch,
    Launched,
    Landed,
}

pub struct Game {
    pub time_launch: f32,

    pub state:     GameState,
    pub trebuchet: Trebuchet,
    pub world:     World,

    pub day:       u32,
    pub resources: Resources,
    pub player:    Player,
}

impl Game {
    pub async fn init() -> Self {
        use TerrainClass as TC;
        // BEarth
        let terra = [
            TC::Ocean(4000),  // Batlantic Ocean
            TC::Plain(8000),  // North Bamerica
            TC::Ocean(12800), // Bacific Ocean
            TC::Desert(3000), // Baustralia
            TC::Ocean(1000),  // Bindian Ocean
            TC::Rocky(9000),  // Beurasia
            TC::Ocean(130),   // Benglish Channel
            TC::Hills(500),   // Great Beantain
            TC::Ocean(1600),  // Batlantic cont.
        ];

        let world = World::new(
            IVec2::ZERO,
            I64Vec2::ZERO,
            6_371_000.0,
            5.972e+24,
            WorldClass::Minshara,
            Some(&terra),
        );

        let mut trebuchet = Trebuchet::init(START_POINT).build();
        trebuchet.reset();
        let player = Player::new(trebuchet.projectile_position());
        let resources = Resources::default();

        Self {
            state: GameState::MainMenu,
            time_launch: 0.0,
            day: 0,

            world,
            trebuchet,
            player,
            resources,
        }
    }

    pub fn reset(&mut self) {
        self.time_launch = 0.0;
        self.state = GameState::PreLaunch;
        self.trebuchet.reset();
        self.player.position = self.trebuchet.projectile_position();
        self.day += 1;
    }
}
