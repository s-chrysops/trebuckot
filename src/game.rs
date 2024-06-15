use macroquad::prelude::*;

use player::*;
use resources::*;
use stats::*;
use tech::*;
use trebuchet::*;
use world::*;
// use upgrades::*;

mod player;
mod resources;
mod stats;
mod tech;
pub mod trebuchet;
mod upgrades;
pub mod world;

const START_POINT: I64Vec2 = i64vec2(0, 1_631_092_932);

#[allow(dead_code)]
pub enum Era {
    Cardboard,
    Wood,
    Steel,
    Space,
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
    pub tech_tree: TechTree,
}

impl Game {
    pub async fn init() -> Result<Game, macroquad::Error> {
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
        let tech_tree = TechTree::init().await?;

        Ok(Game {
            state: GameState::Paused,

            day: 0,
            stats: Stats::default(),

            world,
            trebuchet,
            player,
            resources,
            tech_tree,
        })
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
