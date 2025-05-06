mod model;
mod view;
mod controller;

use controller::game::{Game, GameState};

fn main() {
    let mut game = Game::init_console();

    game.view().print(&game.board(), &[]);

    while *game.state() == GameState::Ongoing {
        if game.play() {
            println!("Turn played");
            game.view().print(&game.board(), &[]);
        }else {
            println!("Turn not played");
        }
    }
    println!("Game over");
}
