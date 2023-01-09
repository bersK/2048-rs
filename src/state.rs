use raylib::prelude::Vector2;

use crate::rl::Moves;
use crate::tile::Tile;

#[derive(Debug)]
pub struct GameState {
    pub grid: [[u32; 3]; 3],
    pub tiles: [Option<Tile>; 9],
    pub score: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            grid: [[0; 3]; 3],
            tiles: [None; 9],
            score: 0,
        }
    }

    pub fn update(&mut self) {
        // Reset the tiles, more work to clean than to rearrange
        self.tiles = [None; 9];

        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                let score = self.grid[x][y];
                if score > 0 {
                    let idx = Tile::generate_tile_idx(x, y);
                    self.tiles[idx] = Some(Tile::new(Vector2{x: x as f32, y: y as f32}, score));
                }
            }
        }
    }

    pub fn move_grid(&mut self, mv: Moves) {
        if mv == Moves::Up {
            println!("{:?}", mv);
        }

        self.update();
    }


    pub fn iterate_grid(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                self.grid[x][y] += 1;
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}