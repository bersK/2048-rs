use raylib::prelude::{
    Rectangle,
    RaylibDraw,
    Color,
    Vector2,
    RaylibDrawHandle};

use crate::{state::GameState, tile};

#[derive(Debug)]
pub struct Grid {
    top_right: Vector2,
    tile_size: i32,
    padding: i32,
    roundedness: f32,
}

impl Grid {
    pub fn new(top_right: Vector2, tile_size: i32, padding: i32, roundedness: f32) -> Self { Self { top_right, tile_size, padding, roundedness} }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle, state: &mut GameState) {
        for tile in state.tiles.iter() {
            if let Some(tile) = tile {
                let x = self.top_right.x + tile.location.x * (self.tile_size + self.padding) as f32;
                let y = self.top_right.y + tile.location.y * (self.tile_size + self.padding) as f32;
                let rect = Rectangle{x, y, height: self.tile_size as f32, width: self.tile_size as f32};
                // TODO(stefan): Will be adding a method for custom aligned based on the size of the integer
                let (x_offset, y_offset) = (self.tile_size / 2, self.tile_size / 2);
                draw_handle.draw_rectangle_rounded(rect, self.roundedness, 12, tile::Tile::get_tile_color(tile.score));
                draw_handle.draw_text(format!("{0}", tile.score).as_str(), x as i32 + self.tile_size / 2 - x_offset, y as i32 + self.tile_size / 2 - y_offset, 40, Color::BLACK);
            }
        }
    }
}