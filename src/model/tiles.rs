use rand::seq::SliceRandom;
use rand::seq::IndexedRandom;

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    A,
    B,
    C,
    D,
}

impl Tile {
    pub fn all_variants() -> &'static [Tile] {
        &[Tile::A, Tile::B, Tile::C, Tile::D]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileState {
    Discovered,
    NotDiscovered,
}

pub struct Tiles {
    tiles: Vec<Vec<Tile>>,
    tiles_state: Vec<Vec<TileState>>,
}

impl Tiles {
    pub fn init() -> Self{
        let generated_tiles = Self::generate_tiles();
        Tiles {
            tiles: generated_tiles.clone(),
            tiles_state: vec![vec![TileState::NotDiscovered; generated_tiles[0].len()]; generated_tiles.len()],
        }
    }

    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    pub fn tiles_state(&self) -> &Vec<Vec<TileState>> {
        &self.tiles_state
    }

    pub fn generate_tiles() -> Vec<Vec<Tile>> {
        const PAIRS: usize = 8;
        let total_tiles = PAIRS * 2;
        let mut flat_tiles = Vec::new();

        let mut rng = rand::rng();
        let tiles_variants = Tile::all_variants().to_vec();

        for _ in 0..PAIRS {
            let selected_tile = tiles_variants.choose(&mut rng).expect("List is empty");
            flat_tiles.push(selected_tile.clone());
            flat_tiles.push(selected_tile.clone());
        }

        Self::shuffle_tiles(&mut flat_tiles);

        let cols = (total_tiles as f64).sqrt().ceil() as usize;
        let rows = total_tiles / cols;

        let mut tiles = Vec::new();
        for r in 0..rows {
            let mut row = Vec::new();
            for c in 0..cols {
                row.push(flat_tiles[r * cols + c].clone());
            }
            tiles.push(row);
        }
        tiles
    }

    pub fn shuffle_tiles(tiles: &mut Vec<Tile>) {
        let mut rng = rand::rng();
        tiles.shuffle(&mut rng);
    }

    pub fn check_possible(&self, (c1, c2): &(usize, usize)) -> bool {
        self.tiles_state[*c1][*c2] != TileState::Discovered
    }

    pub fn change_tile(&mut self, (c1, c2): (usize, usize)) {
        self.tiles_state[c1][c2] = TileState::Discovered;
    }

    pub fn check_equals_tiles(&self, c1: &Tile, c2: &Tile) -> bool {
        if c1 == c2{
            return true;
        }
        false
    }

    pub fn check_all_discovered(&self) -> bool {
        for row in &self.tiles_state {
            for tile in row {
                if *tile == TileState::NotDiscovered {
                    return false;
                }
            }
        }
        true
    }
}