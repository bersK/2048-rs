pub mod rl;
pub mod state;
pub mod tile;
pub mod grid;
pub mod utils;

use state::GameState;
use grid::Grid;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;
const SCREEN_TITLE: &str = "Hello, World";
const TILE_SIZE : i32 = 100;
const PADDING : i32 = 10;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(&SCREEN_TITLE)
        .vsync()
        .build();

    let mut _frame_time: f32 = 0.0;
    let mut _fps: f32 = 0.0;
    let mut _elapsed_time: f32 = 0.0;
    let mut game_state: GameState = GameState::new();
    let offset = TILE_SIZE * (state::GRID_SIZE as i32) + PADDING * (state::GRID_SIZE - 1) as i32;
    let grid_origin: Vector2 = Vector2{x: (SCREEN_WIDTH / 2 - offset / 2) as f32, y: (SCREEN_HEIGHT / 2 - offset / 2) as f32};
    let grid: Grid = Grid::new(grid_origin, 100, 10, 0.2);

    game_state.grid[0][0] = 2;
    game_state.grid[0][1] = 8;
    game_state.grid[0][2] = 2;
    game_state.grid[2][0] = 128;
    game_state.grid[2][1] = 2048;
    game_state.grid[2][2] = 16;
    game_state.grid[3][3] = 4096;
    game_state.grid[2][3] = 64;

    while !rl.window_should_close() {
        // UPDATE
        {
            rl::update_title(&mut rl, &thread, SCREEN_TITLE);
            game_state.move_grid(rl::handle_input(&mut rl));

            _frame_time = rl.get_frame_time();
            _fps = 1.0 / _frame_time;
            _elapsed_time = rl.get_time() as f32;
        }

        // DRAW
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            grid.draw(&mut d, &mut game_state);
            d.draw_text(format!("SCORE:\n{0}", game_state.score).as_str(), 12, 0, 40, Color::BLACK);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{tile::Tile, rl::Moves};

    use super::*;

    #[test]
    fn update_game_state() {
        let mut game_state: GameState = GameState::new();
        game_state.update();
        assert_eq!(game_state.grid[0][0], 0);
    }

    #[test]
    fn get_compressed_array() {
        let mut game_state: GameState = GameState::new();
        println!("{:?}", game_state.grid[0][0]);
        let val = GameState::get_value(&mut game_state.grid);
        *val = 2;
        println!("{:?}", game_state.grid[0][0]);

        assert_eq!(game_state.grid[0][0], 2);
    }


    #[test]
    fn get_tile_color() {
        let tile = Tile::new(Vector2{x: 0.0, y: 0.0}, 0);
        let color = Tile::get_tile_color(0);
        println!("{:?}", color);
        assert_eq!(color, Tile::get_tile_color(tile.score));
        assert_ne!(color, Tile::get_tile_color(128));
        assert_eq!(Tile::get_tile_color(12800), Tile::get_tile_color(12800));
        assert_eq!(Tile::get_tile_color(4096), Tile::get_tile_color(12800));
    }

    #[test]
    fn working_tile_update() {
        let mut game_state: GameState = GameState::new();
        game_state.grid[2][2] = 2;
        game_state.grid[0][0] = 2;
        game_state.update();

        assert_eq!(game_state.tiles[0].unwrap().score, 2);
        print!("{:?}", game_state.tiles[0].unwrap());
        assert_eq!(game_state.tiles[8].unwrap().score, 2);
        print!("{:?}", game_state.tiles[8].unwrap());

        game_state.grid[2][2] = 0;
        game_state.grid[0][0] = 0;
        game_state.update();

        let unwrap_none = std::panic::catch_unwind(|| game_state.tiles[0].unwrap());
        assert!(unwrap_none.is_err());
        let unwrap_none = std::panic::catch_unwind(|| game_state.tiles[8].unwrap());
        assert!(unwrap_none.is_err());
    }

    #[test]
    fn move_grid_down() {
        let mut game_state: GameState = GameState::new();
        game_state.grid[0][2] = 2;
        game_state.grid[0][0] = 2;
        game_state.move_grid(Moves::Right);

        assert_eq!(game_state.grid[0][2], 4);

        game_state.grid[0][1] = 2;
        println!("Before move right\n{:?}", game_state.grid);
        game_state.move_grid(Moves::Right);
        println!("After move right\n{:?}", game_state.grid);
        assert_eq!(game_state.grid[0][2], 4);

        game_state.grid[0][0] = 2;
        println!("After move left\n{:?}", game_state.grid);
        game_state.move_grid(Moves::Left);
        println!("After move left\n{:?}", game_state.grid);
        assert_eq!(game_state.grid[0][0], 4);
        assert_eq!(game_state.grid[0][1], 4);
    }

    #[test]
    fn test_clamp() {
        let val = 10;
        assert_eq!(5, utils::clamp(val, 5, 0));
        assert_eq!(5, utils::clamp(val, 0, 5));
    }
}
