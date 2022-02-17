use std::fs::read_to_string;
use std::path::Path;

use rand::random;

use crate::common::Point;
use crate::common::TILE_H;
use crate::common::TILE_W;
use crate::screen::Pixel;
use crate::screen::Sprite;
use crate::tile_config::TileConfig;
use crate::tile_config::TileId;

#[derive(Debug)]
pub struct TilePos {
    pos: Point<i16>,
    tile_id: TileId,
}

#[derive(Debug)]
pub struct Neighborhood4 {
    left: Option<TilePos>,
    right: Option<TilePos>,
    up: Option<TilePos>,
    down: Option<TilePos>,
}

impl Neighborhood4 {
    #[rustfmt::skip]
    pub fn filter_traversable(self, tile_config: &TileConfig) -> Self {
        Neighborhood4 {
            left:  self. left.filter(|t| tile_config.get(t.tile_id).is_traversable()),
            right: self.right.filter(|t| tile_config.get(t.tile_id).is_traversable()),
            up:    self.   up.filter(|t| tile_config.get(t.tile_id).is_traversable()),
            down:  self. down.filter(|t| tile_config.get(t.tile_id).is_traversable()),
        }
    }
}

impl From<Vec<TilePos>> for Neighborhood4 {
    fn from(mut v: Vec<TilePos>) -> Self {
        Self {
            left: v.pop(),
            right: v.pop(),
            up: v.pop(),
            down: v.pop(),
        }
    }
}

impl From<Neighborhood4> for Vec<TilePos> {
    fn from(n: Neighborhood4) -> Self {
        vec![n.left, n.right, n.up, n.down]
            .into_iter()
            .filter_map(|t| t)
            .collect()
    }
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: u16,
    pub height: u16,
}

impl Map {
    pub fn get_tile_pos(&self, point: Point<i16>) -> Option<TilePos> {
        if point.x < 0 || point.x >= self.width as i16 {
            return None;
        }

        if point.y < 0 || point.y >= self.height as i16 {
            return None;
        }

        let tile = self.tiles[(self.width as i16 * point.y + point.x) as usize].clone();
        Some(TilePos {
            tile_id: tile.tile_id,
            pos: point,
        })
    }

    pub fn get_neighborhood4(&self, point: &Point<i16>) -> Neighborhood4 {
        let left = self.get_tile_pos(point.left());
        let right = self.get_tile_pos(point.right());
        let up = self.get_tile_pos(point.up());
        let down = self.get_tile_pos(point.down());

        Neighborhood4 {
            left,
            right,
            up,
            down,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P, tile_config: &TileConfig) -> Self {
        let map_toml_string = read_to_string(path).unwrap();
        let map_toml_value: toml::value::Value = toml::from_str(&map_toml_string).unwrap();
        let map_data = &map_toml_value["data"];
        let map_data_tile_ids = &map_data["tile_ids"].as_str().unwrap();

        let lines = map_data_tile_ids.split_ascii_whitespace();

        let mut tiles = Vec::new();
        let mut width: u16 = 0;
        let mut height: u16 = 0;

        for line in lines {
            if height == 0 {
                width = line.len() as u16 / TILE_W;
            }
            height += 1;

            let mut chars = line.chars();
            loop {
                let ch1 = chars.next();
                let ch2 = chars.next();
                let ch3 = chars.next();

                if ch1.is_none() || ch2.is_none() || ch3.is_none() {
                    break;
                }

                let tile_id = [ch1.unwrap(), ch2.unwrap(), ch3.unwrap()];

                let max_id = tile_config
                    .get(tile_id)
                    .tile_strings
                    .len() as u8;

                let tile_string_alternative_id = (random::<u8>() % max_id) as usize;

                tiles.push(Tile {
                    tile_id,
                    tile_string_alternative_id,
                });
            }
        }

        Map {
            tiles,
            width,
            height,
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    tile_id: TileId,
    tile_string_alternative_id: usize,
}

pub struct State {
    pub astar_start: Point<i16>,
    pub astar_goal: Point<i16>,
    pub astar_path: Option<Vec<Point<i16>>>,

    pub cursor_pos: Point<u16>,
    pub elapsed_time: u64,
    pub map: Map,
    pub map_pos: Point<i16>,
    pub tile_config: TileConfig,

    pub screen_width: u16,
    pub screen_height: u16,
}

impl State {
    pub fn new() -> Self {
        let cursor_pos = Point::new(1, 1);
        let elapsed_time = 0;
        let tile_config = TileConfig::from_file("tile_config.toml");
        let map = Map::from_file("example_map.toml", &tile_config);
        let map_pos = Point::new(72, 1);

        Self {
            astar_start: Point::new(0, 0),
            astar_goal: Point::new(0, 0),
            astar_path: None,

            cursor_pos,
            elapsed_time,
            map,
            map_pos,
            tile_config,

            screen_width: 0,
            screen_height: 0,
        }
    }

    pub fn get_map_sprite(&self) -> Sprite {
        let mut pixels = Vec::new();
        let width = TILE_W * self.map.width;
        let height = self.map.height;

        for tile in &self.map.tiles {
            let tile_id = tile.tile_id;
            let tile_string_alternative_id = tile.tile_string_alternative_id;

            let tile_frames = &self
                .tile_config
                .get(tile_id)
                .tile_strings[tile_string_alternative_id]
                .frames;
            let frame = (self.elapsed_time % tile_frames.len() as u64) as usize;

            let tile_str = &tile_frames[frame];
            let color = self.tile_config.get(tile_id).color;

            for ch in tile_str.chars() {
                pixels.push(Pixel { ch, color });
            }
        }

        Sprite {
            pixels,
            width,
            height,
        }
    }

    pub fn resize(&mut self, screen_cols: u16, screen_rows: u16) {
        self.screen_width = screen_cols;
        self.screen_height = screen_rows;
    }

    pub fn elapse_time(&mut self) {
        self.elapsed_time += 1;
    }

    pub fn set_astar_start(&mut self) {
        self.astar_start.x = self.cursor_pos.x as i16 - self.map_pos.x - 1;
        self.astar_start.y = self.cursor_pos.y as i16 - self.map_pos.y;
    }

    pub fn set_astar_goal(&mut self) {
        self.astar_goal.x = self.cursor_pos.x as i16 - self.map_pos.x - 1;
        self.astar_goal.y = self.cursor_pos.y as i16 - self.map_pos.y;
    }

    pub fn move_map_left(&mut self) {
        self.map_pos.x -= TILE_W as i16;
    }

    pub fn move_map_right(&mut self) {
        self.map_pos.x += TILE_W as i16;
    }

    pub fn move_map_up(&mut self) {
        self.map_pos.y -= TILE_H as i16;
    }

    pub fn move_map_down(&mut self) {
        self.map_pos.y += TILE_H as i16;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos.x < TILE_W {
            self.move_map_right();
            return;
        }
        self.cursor_pos.x -= TILE_W;

        // align cursor to pixels
        self.cursor_pos.x = ((self.cursor_pos.x - 1) / TILE_W) * TILE_W + TILE_W / 2;
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos.x + TILE_W > (self.screen_width / TILE_W) * TILE_W - 1 {
            self.move_map_left();
            return;
        }
        self.cursor_pos.x += TILE_W;

        // align cursor to pixels
        self.cursor_pos.x = ((self.cursor_pos.x - 1) / TILE_W) * TILE_W + TILE_W / 2;
    }

    pub fn move_cursor_up(&mut self) {
        // TODO: align to pixel
        if self.cursor_pos.y < TILE_H {
            self.move_map_down();
            return;
        }
        self.cursor_pos.y -= TILE_H;
    }

    pub fn move_cursor_down(&mut self) {
        // TODO: align to pixel
        if self.cursor_pos.y + TILE_H > self.screen_height - 1 {
            self.move_map_up();
            return;
        }
        self.cursor_pos.y += TILE_H;
    }
}

pub fn get_shortest_path(
    start: &Point<i16>,
    goal: &Point<i16>,
    map: &Map,
    tile_config: &TileConfig,
) -> Option<Vec<Point<i16>>> {
    let path = pathfinding::prelude::astar(
        start,
        |p| successors(p, map, tile_config),
        |p| heuristic(p, goal),
        |p| p == goal,
    );

    path.map(|p| p.0)
}

fn successors(point: &Point<i16>, map: &Map, tile_config: &TileConfig) -> Vec<(Point<i16>, u32)> {
    let neigh_tiles: Vec<TilePos> = map
        .get_neighborhood4(point)
        .filter_traversable(tile_config)
        .into();

    neigh_tiles
        .iter()
        .map(|t| (t.pos.clone(), 1))
        .collect()
}

fn heuristic(point: &Point<i16>, goal: &Point<i16>) -> u32 {
    (pathfinding::prelude::absdiff(point.x, goal.x)
        + pathfinding::prelude::absdiff(point.y, goal.y)) as u32
}
