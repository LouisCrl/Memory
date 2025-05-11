//this file is all the tile management
use rand::seq::SliceRandom;
use rand::seq::IndexedRandom;

//this enum is every tiles
#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Circle1,
    Circle2,
    Circle3,
    Circle4,
    Circle5,
    Circle6,
    Circle7,
    Circle8,
    Circle9,
    Stick1,
    Stick2,
    Stick3,
    Stick4,
    Stick5,
    Stick6,
    Stick7,
    Stick8,
    Stick9,
    Character1,
    Character2,
    Character3,
    Character4,
    Character5,
    Character6,
    Character7,
    Character8,
    Character9,
}

impl Tile {
    //gives all variants of tiles in a static array
    pub fn all_variants() -> &'static [Tile] {
        &[
            Tile::Circle1,
            Tile::Circle2,
            Tile::Circle3,
            Tile::Circle4,
            Tile::Circle5,
            Tile::Circle6,
            Tile::Circle7,
            Tile::Circle8,
            Tile::Circle9,
            Tile::Stick1,
            Tile::Stick2,
            Tile::Stick3,
            Tile::Stick4,
            Tile::Stick5,
            Tile::Stick6,
            Tile::Stick7,
            Tile::Stick8,
            Tile::Stick9,
            Tile::Character1,
            Tile::Character2,
            Tile::Character3,
            Tile::Character4,
            Tile::Character5,
            Tile::Character6,
            Tile::Character7,
            Tile::Character8,
            Tile::Character9,
        ]
    }
}

//this enum gives the TileState
#[derive(Debug, Clone, PartialEq)]
pub enum TileState {
    Discovered,
    NotDiscovered,
}

//this struct are the tiles
pub struct Tiles {
    tiles: Vec<Vec<Tile>>,
    tiles_state: Vec<Vec<TileState>>,
}

impl Tiles {
    //initialisation of the struct
    pub fn init(nb_pairs: usize) -> Self{
        let generated_tiles = Self::generate_tiles(nb_pairs);
        Tiles {
            tiles: generated_tiles.clone(),
            tiles_state: vec![vec![TileState::NotDiscovered; generated_tiles[0].len()]; generated_tiles.len()],
        }
    }

    //return tiles
    pub fn tiles(&self) -> &Vec<Vec<Tile>> {
        &self.tiles
    }

    //return tiles_state
    pub fn tiles_state(&self) -> &Vec<Vec<TileState>> {
        &self.tiles_state
    }

    //generate a board of random tiles
    pub fn generate_tiles(pairs: usize) -> Vec<Vec<Tile>> {
        let total_tiles = pairs * 2;
        let mut flat_tiles = Vec::new();

        let mut rng = rand::rng();
        let tiles_variants = Tile::all_variants().to_vec();

        for _ in 0..pairs {
            let selected_tile = tiles_variants.choose(&mut rng).expect("List is empty");
            flat_tiles.push(selected_tile.clone());
            flat_tiles.push(selected_tile.clone());
        }

        Self::shuffle_tiles(&mut flat_tiles);

        let cols = (total_tiles as f64).sqrt().ceil() as usize;
        let rows = (total_tiles + cols - 1) / cols;

        let mut tiles = Vec::new();
        for r in 0..rows {
            let mut row = Vec::new();
            for c in 0..cols {
                if r * cols + c < total_tiles {
                    row.push(flat_tiles[r * cols + c].clone());
                }
            }
            tiles.push(row);
        }
        tiles
    }

    //shuffle the board
    pub fn shuffle_tiles(tiles: &mut Vec<Tile>) {
        let mut rng = rand::rng();
        tiles.shuffle(&mut rng);
    }

    //check if a Tile is Discovered
    pub fn check_possible(&self, (c1, c2): &(usize, usize)) -> bool {
        self.tiles_state[*c1][*c2] != TileState::Discovered
    }

    //change the state of a tile
    pub fn change_tile(&mut self, (c1, c2): (usize, usize)) {
        self.tiles_state[c1][c2] = TileState::Discovered;
    }

    //check if the 2 choosed tiles are the same
    pub fn check_equals_tiles(&self, c1: &Tile, c2: &Tile) -> bool {
        if c1 == c2{
            return true;
        }
        false
    }

    //check if all tiles are discovered
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
