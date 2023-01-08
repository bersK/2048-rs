use raylib::prelude::Color;

use crate::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub location: Vector2,
    pub score: u32,
}

impl Tile {
    pub fn new(location: Vector2, score: u32) -> Self { Self { location, score } }

    pub fn get_tile_color(score: u32) -> Color {
        match score {
            0 =>      Color::new(238, 228, 218, 255),
            2 =>      Color::new(238, 228, 218, 255),
            4 =>      Color::new(237, 224, 200, 255),
            8 =>      Color::new(242, 177, 121, 255),
            16 =>     Color::new(245, 149, 99, 255),
            32 =>     Color::new(246, 124, 95, 255),
            64 =>     Color::new(246, 94, 59, 255),
            128 =>    Color::new(237, 207, 114, 255),
            256 =>    Color::new(237, 204, 97, 255),
            512 =>    Color::new(237, 200, 80, 255),
            1024 =>   Color::new(237, 197, 63, 255),
            2048 =>   Color::new(237, 194, 46, 255),
            4096.. => Color::new(60, 58, 50, 255),
            _ =>      Color::new(238, 228, 218, 255),
        }
    }

    pub fn generate_tile_idx(x: usize, y: usize) -> usize {
        (x * 3) + y
    }

}