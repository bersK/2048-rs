pub mod rl;
pub mod state;
pub mod tile;
use state::GameState;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;
const SCREEN_TITLE: &str = "Hello, World";

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(&SCREEN_TITLE)
        .vsync()
        .build();

    let center = Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0);

    let mut _frame_time: f32 = 0.0;
    let mut _fps: f32 = 0.0;
    let mut _elapsed_time: f32 = 0.0;
    let mut game_state: GameState = GameState::new();

    while !rl.window_should_close() {
        // UPDATE
        {
            rl::update(&mut rl, &thread, SCREEN_TITLE);
            _frame_time = rl.get_frame_time();
            _fps = 1.0 / _frame_time;
            _elapsed_time = rl.get_time() as f32;
            game_state.update();
        }

        // DRAW
        {
            let mut d = rl.begin_drawing(&thread);
            d.draw_circle_v(center, 20.0, Color::BLACK);
            d.clear_background(Color::WHITE);
            d.draw_text(format!("Hello, world! - fps - {_fps}").as_str(), 12, 12, 40, Color::BLACK);
            // d.draw_text(format!("{0}", x).as_str(), 12, SCREEN_HEIGHT-80, 40, Color::BLACK);
            d.draw_text(format!("{0:?}", game_state.grid).as_str(), 12, SCREEN_HEIGHT-40, 40, Color::BLACK);
        }
    }
}
