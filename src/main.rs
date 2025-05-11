mod view;
mod model;
mod controller;

use macroquad::prelude::*;
use controller::game::Game;

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

#[macroquad::main(window_conf)]
async fn main() {
    Game::run_game().await;
}