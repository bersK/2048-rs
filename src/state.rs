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
            grid: vec![vec![0; 4]; 4],
            tiles: vec![None; 16],
            score: 0,
        }
    }

    pub fn update(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                self.grid[x][y] += 1;
            }
        }
        todo!()
    }
}