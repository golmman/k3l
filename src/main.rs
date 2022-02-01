use crate::controller::Controller;
use crate::screen::Screen;

use self::state::State;

mod common;
mod controller;
mod screen;
mod state;

fn main() {
    let screen = Screen::new();
    let state = State::new();

    let mut controller = Controller::new(screen, state);

    controller.run();
}
