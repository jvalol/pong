use blitzkit::start;

mod ball;
mod input;
mod player;
mod pong_game;
mod state;
mod system;
mod util;
use pong_game::PongGame;

/// Whether this run is only here to be photographed, for `refresh-screenshots`
/// in the project above.
pub fn staged() -> bool {
    std::env::args().any(|arg| arg == "--screenshot")
}

fn main() {
    let pong_game = PongGame::new();
    start("Pong", Box::new(pong_game));
}
