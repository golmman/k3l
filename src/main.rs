use std::fs::read_to_string;

use crate::controller::Controller;
use crate::tile_config::{TileConfig, TileKind};

use self::state::State;

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
