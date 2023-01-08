use raylib::prelude::Vector2;

use crate::tile::Tile;

#[derive(Debug)]
pub struct GameState {
    pub grid: Vec<Vec<u32>>,
    pub tiles: Vec<Option<Tile>>,
    pub score: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![0; 3]; 3],
            tiles: vec![None; 9],
            score: 0,
        }
    }

    pub fn update(&mut self) {
        // Reset the tiles, more work to clean than to rearrange
        self.tiles = vec![None; 9];

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

    pub fn iterate_grid(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                self.grid[x][y] += 1;
            }
        }
    }
}