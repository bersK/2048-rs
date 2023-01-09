use raylib::{prelude::*, consts::KeyboardKey};

#[derive(Debug, PartialEq, Eq, Clone, Copy )]
pub enum Moves {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub fn update_title(rl: &mut RaylibHandle, thread: &RaylibThread, title: &str) {
    let t: String = String::from(title) + " - FPS: " + rl.get_fps().to_string().as_str();
    rl.set_window_title(&thread, t.as_str());
}

pub fn handle_input(rl: &mut RaylibHandle) -> Moves {
    match rl.get_key_pressed() {
        Some(KeyboardKey::KEY_UP) | Some(KeyboardKey::KEY_W) => {
            return Moves::Up;
        },
        Some(KeyboardKey::KEY_DOWN) | Some(KeyboardKey::KEY_S) => {
            return Moves::Down;
        },
        Some(KeyboardKey::KEY_RIGHT) | Some(KeyboardKey::KEY_D) => {
            return Moves::Right;
        },
        Some(KeyboardKey::KEY_LEFT) | Some(KeyboardKey::KEY_A) => {
            return Moves::Left;
        },
        _ => {
            return Moves::None;
        }
    }
}