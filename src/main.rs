use crate::controller::Controller;

use self::common::Point;
use self::state::Map;
use self::tile_config::TileConfig;

mod color;
mod common;
mod controller;
mod renderer;
mod screen;
mod state;
mod tile_config;

fn main() {
    //let tile_config = TileConfig::from_file("tile_config.toml");
    //let map = Map::from_file("example_map.toml", &tile_config);

    //let start = Point::new(2, 2);
    //let goal = Point::new(9, 10);
    //let x = state::get_shortest_path(&start, &goal, &map, &tile_config);

    //println!("{x:?}");

    //return;

    let mut controller = Controller::new();
    controller.run();
}
