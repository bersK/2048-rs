use raylib::prelude::Vector2;

use crate::rl::Moves;
use crate::tile::Tile;

pub const GRID_SIZE: usize = 4;
pub const GRID_TILES: usize = GRID_SIZE * GRID_SIZE;
pub type GridRow = [u32; GRID_SIZE];
pub type GridArray = [GridRow; GRID_SIZE];

#[derive(Debug)]
pub struct GameState {
    pub grid: GridArray,
    pub tiles: [Option<Tile>; GRID_TILES],
    pub score: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            grid: [[0; GRID_SIZE]; GRID_SIZE],
            tiles: [None; GRID_TILES],
            score: 0,
        }
    }

    pub fn update(&mut self) {
        // Reset the tiles, more work to clean than to rearrange
        self.tiles = [None; GRID_TILES];

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let score = self.grid[y][x];
                self.tiles[Tile::get_idx_1d(x, y)] = Some(Tile::new(Vector2{x: x as f32, y: y as f32}, score));
            }
        }
    }

    pub fn move_grid(&mut self, mv: Moves) {
        match mv {
            Moves::Left => self.move_left(),
            Moves::Right => self.move_right(),
            Moves::Down => self.move_down(),
            Moves::Up => self.move_up(),
            Moves::None => {},
        }

        self.update();
    }

    // fn combine_tiles(&self, arr: &mut Vec<u32>) {
    //     let temp: [u32; 4] = [0; 4];
    //     let curr: usize = 0;
    //     while curr < arr.len() - 1 {
    //         if arr[curr] == arr[curr + 1] {
    //             arr[curr] = arr[curr] * 2;
    //             arr[curr + 1] = 0;
    //         }
    //     }
    // }
    /// Get a compressed array of the grid in the direction of the move
    pub fn get_value(table: &mut GridArray) -> &mut u32 {
        return &mut table[0][0];
    }

    pub fn compressed_array(&self, table: &GridArray, mut x: u32, mut y: u32, x_step: u32, y_step: u32) -> GridRow {
        let mut temp: GridRow = [0; 4];
        let mut idx: usize = 0;
        for _ in 0..GRID_SIZE {
            if table[y as usize][x as usize] != 0 {
                temp[idx] = table[y as usize][x as usize];
                idx += 1;
            }
            x = x + x_step;
            y = y + y_step;
        }
        return temp;
    }

    fn move_left(&mut self) {
        for _x in 0..GRID_SIZE {
            // let mut compressed_arr = self.get_compressed_array(&self.grid, x, 0, 1, 0);
        }

    }

    fn move_right(&mut self) {

    }

    fn move_down(&mut self) {

    }

    fn move_up(&mut self) {

    }

}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}