use blitzkit::start;

mod ball;
mod input;
mod player;
mod pong_game;
mod state;
mod system;
mod util;
use pong_game::PongGame;

fn main() {
    let pong_game = PongGame::new();
    start("Pong", Box::new(pong_game));
}
