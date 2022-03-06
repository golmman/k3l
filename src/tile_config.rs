use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::color::Color;

pub type TileId = [char; 3];

#[derive(Clone, Debug, PartialEq)]
pub enum TileState {
    Solid,
    Liquid,
    Gas,
}

impl From<&str> for TileState {
    fn from(key: &str) -> Self {
        match key {
            "solid" => TileState::Solid,
            "liquid" => TileState::Liquid,
            "gas" => TileState::Gas,
            _ => panic!("TileState '{key}' unkown."),
        }
    }
}

impl From<String> for TileState {
    fn from(key: String) -> Self {
        TileState::from(key.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct TileString {
    pub frames: Vec<String>,
}

impl From<Vec<&str>> for TileString {
    fn from(f: Vec<&str>) -> Self {
        let frames = f
            .into_iter()
            .map(String::from)
            .collect();

        Self { frames }
    }
}

impl From<&toml::Value> for TileString {
    fn from(value: &toml::Value) -> Self {
        let str_vec: Vec<&str> = value
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap())
            .collect();

        TileString::from(str_vec)
    }
}

#[derive(Clone, Debug)]
pub struct BaseTile {
    pub block_state: TileState,
    pub color: Color,
    pub floor_state: TileState,
    pub id: TileId,
    pub key: String,
    pub minable: bool,
    pub name: String,
    pub tile_strings: Vec<TileString>,
}

impl BaseTile {
    pub fn is_traversable(&self) -> bool {
        self.block_state == TileState::Gas && self.floor_state == TileState::Solid
    }
}

#[derive(Clone, Debug)]
pub struct TileConfig {
    tiles: HashMap<TileId, BaseTile>,
}

impl TileConfig {
    pub fn get(&self, tile_id: TileId) -> &BaseTile {
        self.tiles.get(&tile_id).unwrap()
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut tiles = HashMap::new();

        let tile_config_string = read_to_string(path).unwrap();
        let tile_config: toml::value::Value = toml::from_str(&tile_config_string).unwrap();
        let tile_confg_table = tile_config.as_table().unwrap();

        for (key, t) in tile_confg_table {
            let bg_color = t["bg_color"].as_integer().unwrap() as u8;
            let fg_color = t["fg_color"].as_integer().unwrap() as u8;

            let key = key.to_string();
            let color = Color::new(bg_color, fg_color);
            let minable = t
                .get("minable")
                .map(|v| v.as_bool())
                .flatten()
                .unwrap_or(false);
            let name = t["name"].as_str().unwrap().to_string();
            let floor_state = TileState::from(t["floor_state"].as_str().unwrap());
            let block_state = TileState::from(t["block_state"].as_str().unwrap());
            let mut id_chars = t["id"].as_str().unwrap().chars();
            let id = [
                id_chars.next().unwrap(),
                id_chars.next().unwrap(),
                id_chars.next().unwrap(),
            ];
            let tile_strings = t["tile_strings"]
                .as_array()
                .unwrap()
                .iter()
                .map(TileString::from)
                .collect();

            tiles.insert(
                id,
                BaseTile {
                    block_state,
                    color,
                    floor_state,
                    id,
                    key,
                    minable,
                    name,
                    tile_strings,
                },
            );
        }

        Self { tiles }
    }
}
