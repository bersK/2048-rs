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
                let (mut x_offset, mut y_offset, mut font_size) = (10, 15, 40);
                Grid::modify_font_offsets(tile.score, &mut x_offset, &mut y_offset, &mut font_size);
                draw_handle.draw_rectangle_rounded(rect, self.roundedness, 12, tile::Tile::get_tile_color(tile.score));

                // Draw the score text if the score is above 0
                if tile.score <= 0 {
                    continue;
                }
                draw_handle.draw_text(
                    &tile.score.to_string(),
                    x as i32 + self.tile_size / 2 - x_offset,
                    y as i32 + self.tile_size / 2 - y_offset,
                    font_size,
                    Color::BLACK);
            }
        }
    }

    fn modify_font_offsets(score: u32, x_offset: &mut i32, y_offset: &mut i32, font_size: &mut i32) {
        match score {
            0..=8 => { *x_offset = 18; *y_offset = 33; *font_size = 80;},
            16..=64 => { *x_offset = 22; *y_offset = 25; *font_size = 60; },
            128..=512 => { *x_offset = 25; *y_offset = 15; *font_size = 40;},
            1024..=4096 => { *x_offset = 45; *y_offset = 15; *font_size = 40; },
            _ => {},
        }
    }

}