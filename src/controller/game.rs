use crate::view::view::View;
use crate::model::tiles::{Tile, Tiles};

#[derive(Debug, PartialEq)]
pub enum GameState {
    Ongoing,
    End,
}

pub struct Game {
    view: View,
    board: Tiles,
    state: GameState,
}

impl Game {
    pub fn init_console() -> Self {
        Game {
            view: View::Console,
            board: Tiles::init(),
            state: GameState::Ongoing,
        }
    }

    pub fn view(&self) -> &View {
        &self.view
    }

    pub fn board(&self) -> &Tiles {
        &self.board
    }

    pub fn mut_board(&mut self) -> &mut Tiles {
        &mut self.board
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn play(&mut self) -> bool {
        if self.state == GameState::Ongoing {
            let tiles = self.select_tiles();

            if !self.check_different_choices(&tiles.0, &tiles.1) {
                return false;
            }

            if self.board().check_equals_tiles(&self.get_tile(tiles.0), &self.get_tile(tiles.1)) {
                self.mut_board().change_tile(tiles.0);
                self.mut_board().change_tile(tiles.1);
            }
        }
        self.state = if self.check_end() {
            GameState::End
        }else {
            GameState::Ongoing
        };
        true
    }

    pub fn select_tiles(&self) -> ((usize, usize), (usize, usize)) {
        let mut c1: (usize, usize);
        let mut c2: (usize, usize);

        loop {
            c1 = self.view.choosing_tile(1);
            if self.board.check_possible(&c1) {
                break
            }
        }
        self.view.print(&self.board, &[c1]);
        loop {
            c2 = self.view.choosing_tile(2);
            if self.board.check_possible(&c2) {
                break
            }
        }
        self.view.print(&self.board, &[c1, c2]);

        (c1, c2)
    }

    pub fn check_different_choices(&self, c1: &(usize, usize), c2: &(usize, usize)) -> bool {
        if c1 != c2{
            return true;
        }
        false
    }

    pub fn get_tile(&self, (c1, c2): (usize, usize)) -> Tile {
        self.board().tiles()[c1][c2].clone()
    }

    pub fn check_end(&self) -> bool {
        if self.board().check_all_discovered() {
            return true;
        }
        false
    }
}