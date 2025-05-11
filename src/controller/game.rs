//this file is the program to run the game
use macroquad::prelude::*;
use crate::view::ui::{UI, GREY_PURPLE};

//this enum give the actual GameState
#[derive(Debug, PartialEq)]
pub enum GameState {
    Ongoing,
    End,
}

//this enum give the choice after the endgame method
#[derive(Debug, PartialEq)]
pub enum EndGameChoice {
    Again,
    Quit,
}

pub struct Game;

impl Game {
    //run the game
    pub async fn run_game() {
        loop {
            let nb_pairs = UI::menu().await;

            if nb_pairs != 0 {
                UI::render_game(nb_pairs).await;

                match UI::endgame().await {
                    EndGameChoice::Again => {
                        while is_mouse_button_down(MouseButton::Left) {
                            clear_background(GREY_PURPLE);
                            next_frame().await;
                        }
                        continue;
                    }
                    EndGameChoice::Quit => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
}
