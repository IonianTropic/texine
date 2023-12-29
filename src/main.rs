use game::Game;
use display::Display;

mod game;
mod display;

fn main() {
    Game::new().run()
}
