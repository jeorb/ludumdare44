use quicksilver::{
    input::Key,
    lifecycle::Window,
};

pub struct Input {
    pub x: f32,
    pub y: f32,
    pub shoot: bool,
    pub quit: bool,
}

pub fn get_input(window: &Window) -> Input {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut shoot = false;
    let mut quit = false;

    if window.keyboard()[Key::Right].is_down() || window.keyboard()[Key::D].is_down() {
        x += 1.0;
    }
    if window.keyboard()[Key::Left].is_down() || window.keyboard()[Key::A].is_down() {
        x -= 1.0;
    }
    if window.keyboard()[Key::Up].is_down() || window.keyboard()[Key::W].is_down() {
        y -= 1.0;
    }
    if window.keyboard()[Key::Down].is_down() || window.keyboard()[Key::S].is_down() {
        y += 1.0;
    }
    if window.keyboard()[Key::LShift].is_down() || window.keyboard()[Key::RShift].is_down() {
        x *= 0.5;
        y *= 0.5;
    }
    if window.keyboard()[Key::Space].is_down() {
        shoot = true;
    }
    if window.keyboard()[Key::Q].is_down() || window.keyboard()[Key::Escape].is_down() {
        quit = true;
    }
    Input {
        x: x,
        y: y,
        shoot: shoot,
        quit: quit,
    }

}
