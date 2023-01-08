use raylib::prelude::{
    Rectangle,
    RaylibDraw,
    Color,
    Vector2,
    RaylibDrawHandle};

#[allow(dead_code)]
use crate::{state::GameState};

#[derive(Debug)]
struct Grid {
    top_right: Vector2,
    tile_size: i32,
    padding: i32,
    roundedness: f32,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(top_right: Vector2, tile_size: i32, padding: i32, roundedness: f32) -> Self { Self { top_right, tile_size, padding, roundedness} }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle, state: &mut GameState) {
        for tile in state.tiles.iter() {
            if let Some(tile) = tile {
                let x = self.top_right.x + tile.location.x * (self.tile_size + self.padding) as f32;
                let y = self.top_right.y + tile.location.y * (self.tile_size + self.padding) as f32;
                let rect = Rectangle{x, y, height: self.tile_size as f32, width: self.tile_size as f32};
                draw_handle.draw_rectangle_rounded(rect, self.roundedness, 12, Color::RED)
            }
        }
    }

    pub fn top_right(&self) -> Vector2 {
        self.top_right
    }

    pub fn tile_size(&self) -> i32 {
        self.tile_size
    }

    pub fn padding(&self) -> i32 {
        self.padding
    }
}