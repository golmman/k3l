use crate::controller::Controller;

mod color;
mod common;
mod controller;
mod renderer;
mod screen;
mod state;
mod tile_config;

fn main() {
    let mut controller = Controller::new();
    controller.run();
}
