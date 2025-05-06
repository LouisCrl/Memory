use std::io;
use crate::model::tiles::{Tile, TileState, Tiles};

pub struct Console;

impl Console {
    pub fn correspond(tile: &Tile) -> char {
        match tile {
            Tile::A => 'A',
            Tile::B => 'B',
            Tile::C => 'C',
            Tile::D => 'D',
        }
    }

    pub fn printed_board(tiles: &Tiles, coords: &[(usize, usize)]) -> Vec<Vec<char>> {
        let mut board = Vec::new();
        for r in 0..tiles.tiles().len() {
            let mut lines = Vec::new();
            for c in 0..tiles.tiles()[r].len() {
                match tiles.tiles_state()[r][c] {
                    TileState::Discovered => lines.push(Console::correspond(&tiles.tiles()[r][c])),
                    TileState::NotDiscovered => {
                        if coords.contains(&(r, c)) {
                            lines.push(Console::correspond(&tiles.tiles()[r][c]));
                        }else {
                            lines.push(' ');
                        }
                    },
                }
            }
            board.push(lines);
        }
        board
    }

    pub fn print(tiles: &Tiles, coords: &[(usize, usize)]) {
        let mut i: usize = 0;
        for lines in Console::printed_board(tiles, coords) {
            i += 1;
            println!(" {}", " ###".repeat(lines.len()));
            print!("{}", i);
            for col in &lines {
                print!(" #{}#", col);
            }
            println!();
            println!(" {}", " ###".repeat(lines.len()));
            if i < tiles.tiles().len() {
                println!(" {}", "    ".repeat(lines.len()));
            } else {
                let mut footer = String::from("  ");
                for index in 0..tiles.tiles()[0].len() {
                    footer.push(' ');
                    footer.push((b'A' + index as u8) as char);
                    footer.push(' ');
                    if index < tiles.tiles()[0].len() - 1 {
                        footer.push(' ');
                    }
                }
                println!("{}", footer);
            }
        }
        println!();
    }

    pub fn choosing_tile(n: u8) -> (usize, usize) {
        loop {
            println!("Choice {}:", n);

            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("Failed to read line");
            c = c.trim().to_string();

            let coord = Console::get_coor(&c);
            if coord.0 {
                return coord.1;
            }else {
                println!("Invalid choice");
            }
        }
    }

    pub fn get_coor(c: &str) -> (bool, (usize, usize)) {
        match c {
            "a1" => (true, (0, 0)),
            "a2" => (true, (1, 0)),
            "a3" => (true, (2, 0)),
            "a4" => (true, (3, 0)),
            "b1" => (true, (0, 1)),
            "b2" => (true, (1, 1)),
            "b3" => (true, (2, 1)),
            "b4" => (true, (3, 1)),
            "c1" => (true, (0, 2)),
            "c2" => (true, (1, 2)),
            "c3" => (true, (2, 2)),
            "c4" => (true, (3, 2)),
            "d1" => (true, (0, 3)),
            "d2" => (true, (1, 3)),
            "d3" => (true, (2, 3)),
            "d4" => (true, (3, 3)),
            _ => (false, (0, 0)),
        }
    }
}