use crate::common::TILE_SIZE;
use std::fs::read_to_string;
use std::path::Path;

use crate::common::MapPoint;
use crate::tile_config::{TileConfig, TileId};
use rand::random;

#[derive(Debug)]
pub struct TilePos {
    pub pos: MapPoint,
    pub tile_id: TileId,
}

#[derive(Clone)]
pub struct Tile {
    pub tile_id: TileId,
    pub tile_string_alternative_id: usize,
}

#[derive(Debug)]
pub struct Neighborhood4 {
    left: Option<TilePos>,
    right: Option<TilePos>,
    up: Option<TilePos>,
    down: Option<TilePos>,
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub size: MapPoint,
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
            .flatten()
            .collect()
    }
}

impl Map {
    pub fn get_tile(&self, point: &MapPoint) -> Option<&Tile> {
        if point.x < 0 || point.x >= self.size.width() {
            return None;
        }

        if point.y < 0 || point.y >= self.size.height() {
            return None;
        }

        Some(&self.tiles[(self.size.width() * point.y + point.x) as usize])
    }

    pub fn get_tile_pos(&self, point: &MapPoint) -> Option<TilePos> {
        if let Some(tile) = self.get_tile(point) {
            return Some(TilePos {
                tile_id: tile.tile_id,
                pos: point.clone(),
            });
        }

        None
    }

    pub fn get_neighborhood4(&self, point: &MapPoint) -> Neighborhood4 {
        let left = self.get_tile_pos(&point.left());
        let right = self.get_tile_pos(&point.right());
        let up = self.get_tile_pos(&point.up());
        let down = self.get_tile_pos(&point.down());

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
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        for line in lines {
            if height == 0 {
                width = line.len() as i32 / TILE_SIZE.width();
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
            size: MapPoint::new(width, height),
        }
    }
}
