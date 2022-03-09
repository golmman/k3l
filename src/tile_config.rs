use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use crate::color::Color;
use crate::screen::Animation;

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
pub struct BaseTile {
    pub block_state: TileState,
    pub color: Color,
    pub floor_state: TileState,
    pub id: TileId,
    pub key: String,
    pub minable: bool,
    pub name: String,
    pub animations: Vec<Animation>,
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
                .and_then(|v| v.as_bool())
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
            let animations = t["animations"]
                .as_array()
                .unwrap()
                .iter()
                .map(Animation::from)
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
                    animations,
                },
            );
        }

        Self { tiles }
    }
}
