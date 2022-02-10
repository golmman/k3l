use std::fs::read_to_string;

use crate::controller::Controller;
use crate::tile_config::{TileKind, load_tile_config};

mod common;
mod controller;
mod renderer;
mod screen;
mod state;
mod tile_config;

fn main() {
    let tiles_config_string = read_to_string("tile_config.toml").unwrap();
    let tiles_config: toml::value::Value = toml::from_str(&tiles_config_string).unwrap();

    println!("{}", TileKind::DirtFloor as i32);
    println!("{}", TileKind::DirtWall as i32);
    println!("{}", tiles_config["dirt_wall"]["bgcolor"].as_str().unwrap());
    println!("{}", tiles_config["multiline_test"]["name"]);
    println!("{}", tiles_config["multiline_test"]["pp"].get(1).unwrap().get(1).unwrap());
    println!("{:?}", tiles_config.get("nonsense"));

    let tc = load_tile_config("tile_config.toml");
    println!("{tc:#?}");
    return;

    let mut controller = Controller::new();
    controller.run();
}
