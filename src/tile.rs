use colors_transform::{Rgb};
use colors_transform::Color as ColorTrait;
use raylib::prelude::Color;

use crate::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub location: Vector2,
    pub score: u32,
}

impl Tile {
    pub fn new(location: Vector2, score: u32) -> Self { Self { location, score } }

    fn get_tile_color_rgb(score: u32) -> Rgb {
        match score {
            0 =>    Rgb::from_hex_str("#eee4da").unwrap(),
            2 =>    Rgb::from_hex_str("#eee4da").unwrap(),
            4 =>    Rgb::from_hex_str("#ede0c8").unwrap(),
            8 =>    Rgb::from_hex_str("#f2b179").unwrap(),
            16 =>   Rgb::from_hex_str("#f59563").unwrap(),
            32 =>   Rgb::from_hex_str("#f67c5f").unwrap(),
            64 =>   Rgb::from_hex_str("#f65e3b").unwrap(),
            128 =>  Rgb::from_hex_str("#edcf72").unwrap(),
            256 =>  Rgb::from_hex_str("#edcc61").unwrap(),
            512 =>  Rgb::from_hex_str("#edc850").unwrap(),
            1024 => Rgb::from_hex_str("#edc53f").unwrap(),
            2048 => Rgb::from_hex_str("#edc22e").unwrap(),
            4096.. => Rgb::from_hex_str("#3c3a32").unwrap(),
            _ =>    Rgb::from_hex_str("#eee4da").unwrap(),
        }
    }

    pub fn get_tile_color(score: u32) -> Color {
        let rgb = Self::get_tile_color_rgb(score);
        Color::new(rgb.get_red() as u8, rgb.get_green() as u8, rgb.get_blue() as u8, 255)
    }

    pub fn generate_tile_idx(x: usize, y: usize) -> usize {
        (x * 3) + y
    }

}