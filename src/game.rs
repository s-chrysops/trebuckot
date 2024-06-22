use macroquad::prelude::*;

use player::*;
use resources::*;
use settings::*;
use stats::*;
use tech::*;
use trebuchet::*;
use world::*;

use crate::GameError;
// use upgrades::*;

pub mod player;
mod resources;
mod settings;
mod stats;
mod tech;
pub mod trebuchet;
mod upgrades;
pub mod world;

const START_POINT: I64Vec2 = i64vec2(0, 1_631_092_964);

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
    pub tech_tree: TechTree, 

    pub settings: Settings,
}

impl Game {
    pub async fn init() -> Result<Game, GameError> {
        let world = World::new(
            IVec2::ZERO,
            I64Vec2::ZERO,
            6_371_000.0,
            5.972e+24,
            WorldClass::Minshara,
            None,
        );

        let mut trebuchet = Trebuchet::init(START_POINT).build().await?;
        trebuchet.reset();
        let player = Player::new(trebuchet.projectile_position());

        Ok(Game {
            state: GameState::Paused,

            day: 0,
            stats: Stats::default(),

            world,
            trebuchet,
            player,
            resources: Resources::default(),
            tech_tree: TechTree::init().await?,

            settings: Settings::default(),
        })
    }

    pub fn new_game(&mut self) {
        use terrain::TerrainClass as TC;
        // BEarth
        let terra = [
            (TC::Ocean, 4000),  // Batlantic Ocean
            (TC::Plain, 8000),  // North Bamerica
            (TC::Ocean, 12800), // Bacific Ocean
            (TC::Sands, 3000),  // Baustralia
            (TC::Ocean, 1000),  // Bindian Ocean
            (TC::Rocky, 9000),  // Beurasia
            (TC::Ocean, 130),   // Benglish Channel
            (TC::Hills, 500),   // Great Beantain
            (TC::Ocean, 1600),  // Batlantic cont.
        ];

        self.world = World::new(
            IVec2::ZERO,
            I64Vec2::ZERO,
            6_371_000.0,
            5.972e+24,
            WorldClass::Minshara,
            Some(&terra),
        );
        self.state = GameState::PreLaunch;
    }

    pub fn next_day(&mut self) {
        self.resources.research += self.stats.crunch();
        self.stats = Stats::default();
        self.state = GameState::PreLaunch;
        self.trebuchet.reset();
        self.player.position = self.trebuchet.projectile_position();
        self.player.rotation = 0.0;
        self.day += 1;
    }
}
