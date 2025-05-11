//this file is the main.rs file with the main function in it
mod view;
mod model;
mod controller;

use macroquad::prelude::*;
use controller::game::Game;

//setup the window for macroquad
fn window_conf() -> Conf {
    Conf {
        window_title: "Memory".to_string(),
        fullscreen: false,
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

//main function necessary in every Rust programs
#[macroquad::main(window_conf)]
async fn main() {
    Game::run_game().await;
}