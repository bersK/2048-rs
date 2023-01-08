use crate::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub idx: u32,
    pub rounding_corner: u32,
}

impl Tile {
    pub fn new(location: Vector2) -> Self {
        Self {
            idx: Self::gen_idx(location),
            rounding_corner: 0,
        }
    }

    // Converts a 2D location to a 1D index 
    pub fn gen_idx(location: Vector2) -> u32 {
        location.x as u32 * 3 + location.y as u32 * 3
    }
    // Converts a 1D index to a 2D location
    pub fn gen_location(idx: u32) -> Vector2 {
        Vector2::new((idx / 3u32) as f32, (idx % 3u32) as f32)
    }
}