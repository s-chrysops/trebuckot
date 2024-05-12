use macroquad::prelude::*;

mod game;
pub use game::*;

mod world;
pub use world::*;

mod entity;
pub use entity::*;

mod player;
pub use player::*;

const GAME_SIZE_X: f32 = 1920.0;
const GAME_SIZE_Y: f32 = 1080.0;

#[macroquad::main("Trebuckot")]
async fn main() {
    let mut game = Game::init().await;

    loop {
        game.run();

        next_frame().await;
    }
}
