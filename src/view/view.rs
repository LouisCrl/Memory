use crate::model::tiles::Tiles;
use crate::view::console::Console;

pub enum View {
    Console,
    //Graphic,
}

impl View {
    pub fn print(&self, tiles: &Tiles, coords: &[(usize, usize)]) {
        match self {
            View::Console => Console::print(tiles, coords),
            //View::Graphic => Graphic::print(tiles, coords),
        }
    }

    pub fn choosing_tile(&self, n: u8) -> (usize, usize) {
        match self {
            View::Console => Console::choosing_tile(n),
            //View::Graphic => Graphic::choosing_tile(n),
        }
    }
}