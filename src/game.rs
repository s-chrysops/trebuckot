use crate::{player::Player, resources::*, trebuchet::Trebuchet, world::*};
use macroquad::prelude::*;

const START_POINT: I64Vec2 = i64vec2(0, 1_631_092_932);

#[allow(dead_code)]
pub enum Era {
    Cardboard,
    Wood,
    Steel,
    Space,
}

pub struct Stat {
    pub field: String,
    pub value: f32,
    pub unit:  String,
}

#[derive(Default)]
pub struct Stats {
    pub time:         f32,
    pub distance:     f32,
    pub max_altitude: f32,
    pub max_speed:    f32,
}

impl Stats {
    pub fn as_vec(&self) -> Vec<Stat> {
        vec![
            Stat {field: "Time".to_string(), value: self.time, unit: "s".to_string()},
            Stat {field: "Distance".to_string(), value: self.distance, unit: "m".to_string()},
            Stat {field: "Max Altitude".to_string(), value: self.max_altitude, unit: "m".to_string()},
            Stat {field: "Max Speed".to_string(), value: self.max_speed, unit: "m/s".to_string()},
        ]
    }
    pub fn crunch(&self) -> u32 {
        ((self.distance * 0.1) + (self.max_altitude * 0.3) + (self.max_speed * 0.6)) as u32
    }
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    Paused,
    PreLaunch,
    Launched,
    Landed,
}

pub struct Game {
    pub state: GameState,

    pub day:   u32,
    pub stats: Stats,

    pub world:     World,
    pub trebuchet: Trebuchet,
    pub player:    Player,
    pub resources: Resources,
}

impl Game {
    pub async fn init() -> Game {
        use terrain::TerrainClass as TC;
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

        Game {
            state: GameState::Paused,

            day: 0,
            stats: Stats::default(),

            world,
            trebuchet,
            player,
            resources,
        }
    }

    pub fn next_day(&mut self) {
        self.resources.research += self.stats.crunch();
        self.stats = Stats::default();
        self.state = GameState::PreLaunch;
        self.trebuchet.reset();
        self.player.position = self.trebuchet.projectile_position();
        self.day += 1;
    }
}
