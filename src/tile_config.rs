use std::fs::read_to_string;
use std::path::Path;

use crate::color::Color;

// TODO: make this unecessary
const TILE_KIND_KEYS: [&str; 3] = ["dirt_floor", "dirt_wall", "lava_floor"];

#[derive(Clone, Copy, Debug)]
pub enum TileKind {
    DirtFloor,
    DirtWall,
    LavaFloor,
}

impl From<&str> for TileKind {
    fn from(key: &str) -> Self {
        match key {
            "dirt_floor" => TileKind::DirtFloor,
            "dirt_wall" => TileKind::DirtWall,
            "lava_floor" => TileKind::LavaFloor,
            _ => panic!(),
        }
    }
}

impl From<String> for TileKind {
    fn from(key: String) -> Self {
        TileKind::from(key.as_str())
    }
}

#[derive(Clone, Debug)]
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
            _ => panic!(),
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
    pub color: Color,
    pub name: String,
    pub tile_strings: Vec<TileString>,
    pub floor_state: TileState,
    pub block_state: TileState,
    pub kind: TileKind,
}

impl BaseTile {
    pub fn new() -> Self {
        Self {
            color: Color {
                bg_color: 0,
                fg_color: 7,
            },
            name: String::from(""),
            tile_strings: Vec::new(),
            floor_state: TileState::Solid,
            block_state: TileState::Solid,
            kind: TileKind::DirtFloor,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TileConfig {
    tiles: Vec<BaseTile>,
}

impl TileConfig {
    pub fn get(&self, tile_kind: TileKind) -> &BaseTile {
        &self.tiles[tile_kind as usize]
    }
}

impl<P: AsRef<Path>> From<P> for TileConfig {
    fn from(path: P) -> Self {
        let mut tiles = vec![BaseTile::new(); TILE_KIND_KEYS.len()];

        let tile_config_string = read_to_string(path).unwrap();
        let tile_config: toml::value::Value = toml::from_str(&tile_config_string).unwrap();

        for key in TILE_KIND_KEYS {
            let t = &tile_config[key];
            let bg_color = t["bg_color"].as_integer().unwrap() as u8;
            let fg_color = t["fg_color"].as_integer().unwrap() as u8;

            let color = Color { bg_color, fg_color };
            let name = t["name"].as_str().unwrap().to_string();
            let floor_state = TileState::from(t["floor_state"].as_str().unwrap());
            let block_state = TileState::from(t["block_state"].as_str().unwrap());
            let kind = TileKind::from(key);
            let tile_strings = t["tile_strings"]
                .as_array()
                .unwrap()
                .iter()
                .map(TileString::from)
                .collect();

            tiles[kind as usize] = BaseTile {
                color,
                name,
                tile_strings,
                floor_state,
                block_state,
                kind,
            }
        }

        Self { tiles }
    }
}
