use crate::controller::Controller;
use crate::screen::Screen;

mod controller;
mod screen;

fn main() {
    println!("Hello, world!");

    let screen = Screen::new();

    let mut controller = Controller::new(screen);

    controller.run();
}
