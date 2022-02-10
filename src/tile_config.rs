use std::fs::read_to_string;
use std::path::Path;

use termion::color::Color;

use crate::common::{FRAMES_PER_SECOND, PIXEL_W};

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
pub enum TileColor {
    Black,
    Blue,
    Cyan,
    Green,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
    Magenta,
    Red,
    White,
    Yellow,
}

impl TileColor {
    pub fn get_color(&self) -> Box<dyn Color> {
        match self {
            TileColor::Black => Box::new(termion::color::Black),
            TileColor::Blue => Box::new(termion::color::Blue),
            TileColor::Cyan => Box::new(termion::color::Cyan),
            TileColor::Green => Box::new(termion::color::Green),
            TileColor::LightBlack => Box::new(termion::color::LightBlack),
            TileColor::LightBlue => Box::new(termion::color::LightBlue),
            TileColor::LightCyan => Box::new(termion::color::LightCyan),
            TileColor::LightGreen => Box::new(termion::color::LightGreen),
            TileColor::LightMagenta => Box::new(termion::color::LightMagenta),
            TileColor::LightRed => Box::new(termion::color::LightRed),
            TileColor::LightWhite => Box::new(termion::color::LightWhite),
            TileColor::LightYellow => Box::new(termion::color::LightYellow),
            TileColor::Magenta => Box::new(termion::color::Magenta),
            TileColor::Red => Box::new(termion::color::Red),
            TileColor::White => Box::new(termion::color::White),
            TileColor::Yellow => Box::new(termion::color::Yellow),
            _ => panic!(),
        }
    }
}

impl From<&str> for TileColor {
    fn from(key: &str) -> Self {
        match key {
            "black" => TileColor::Black,
            "blue" => TileColor::Blue,
            "cyan" => TileColor::Cyan,
            "green" => TileColor::Green,
            "light_black" => TileColor::LightBlack,
            "light_blue" => TileColor::LightBlue,
            "light_cyan" => TileColor::LightCyan,
            "light_green" => TileColor::LightGreen,
            "light_magenta" => TileColor::LightMagenta,
            "light_red" => TileColor::LightRed,
            "light_white" => TileColor::LightWhite,
            "light_yellow" => TileColor::LightYellow,
            "magenta" => TileColor::Magenta,
            "red" => TileColor::Red,
            "white" => TileColor::White,
            "yellow" => TileColor::Yellow,
            _ => panic!("{key}"),
        }
    }
}

impl From<String> for TileColor {
    fn from(key: String) -> Self {
        TileColor::from(key.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct Pixel {
    frames: Vec<String>,
}

impl From<Vec<&str>> for Pixel {
    fn from(f: Vec<&str>) -> Self {
        let frames_str = match f.len() {
            8 => f,
            4 => vec![f[0], f[0], f[1], f[1], f[2], f[2], f[3], f[3]],
            2 => vec![f[0], f[0], f[0], f[0], f[1], f[1], f[1], f[1]],
            1 => vec![f[0], f[0], f[0], f[0], f[0], f[0], f[0], f[0]],
            _ => panic!("A pixel must have 1, 2, 4 or 8 frames defined."),
        };

        let frames = frames_str
            .into_iter()
            .map(String::from)
            .collect();

        Self {
            frames,
        }
    }
}

impl From<&toml::Value> for Pixel {
    fn from(value: &toml::Value) -> Self {
        let str_vec: Vec<&str> = value
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| x.as_str().unwrap())
            .collect();

        Pixel::from(str_vec)
    }
}

#[derive(Clone, Debug)]
pub struct BaseTile {
    bgcolor: TileColor,
    fgcolor: TileColor,
    name: String,
    pixels: Vec<Pixel>,
    floor_state: TileState,
    block_state: TileState,
    kind: TileKind,
}

impl BaseTile {
    pub fn new() -> Self {
        Self {
            bgcolor: TileColor::Black,
            fgcolor: TileColor::Black,
            name: String::from(""),
            pixels: Vec::new(),
            floor_state: TileState::Solid,
            block_state: TileState::Solid,
            kind: TileKind::DirtFloor,
        }
    }
}

#[derive(Debug)]
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

        let tile_config_string = read_to_string("tile_config.toml").unwrap();
        let tile_config: toml::value::Value = toml::from_str(&tile_config_string).unwrap();

        for key in TILE_KIND_KEYS {
            let t = &tile_config[key];

            let bgcolor = TileColor::from(t["bgcolor"].as_str().unwrap());
            let fgcolor = TileColor::from(t["fgcolor"].as_str().unwrap());
            let name = t["name"].as_str().unwrap().to_string();
            let floor_state = TileState::from(t["floor_state"].as_str().unwrap());
            let block_state = TileState::from(t["block_state"].as_str().unwrap());
            let kind = TileKind::from(key);
            let pixels = t["pixels"]
                .as_array()
                .unwrap()
                .into_iter()
                .map(Pixel::from)
                .collect();

            tiles[kind as usize] = BaseTile {
                bgcolor,
                fgcolor,
                name,
                pixels,
                floor_state,
                block_state,
                kind,
            }
        }

        Self { tiles }
    }
}
