use raylib::prelude::*;

pub fn update_title(rl: &mut RaylibHandle, thread: &RaylibThread, title: &str) {
    let t: String = String::from(title) + " - FPS: " + rl.get_fps().to_string().as_str();
    rl.set_window_title(&thread, t.as_str());
}