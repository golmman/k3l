use rand::random;

use crate::common::Point;
use crate::common::TEST_MAP_HEIGHT;
use crate::common::TEST_MAP_TILES;
use crate::common::TEST_MAP_WIDTH;
use crate::common::TILE_H;
use crate::common::TILE_W;
use crate::screen::Pixel;
use crate::screen::Sprite;
use crate::tile_config::TileConfig;
use crate::tile_config::TileId;

pub struct State {
    pub cursor_pos: Point<u16>,
    pub elapsed_time: u64,
    pub map: Map,
    pub map_pos: Point<i16>,
    pub tile_config: TileConfig,

    pub screen_width: u16,
    pub screen_height: u16,
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: u16,
    pub height: u16,
}

impl Map {
    pub fn new(tile_config: &TileConfig, tile_ids: Vec<&str>, width: u16, height: u16) -> Self {
        let mut tiles = Vec::new();
        for tile_id in tile_ids {
            let mut id_chars = tile_id.chars();
            let tile_id = [id_chars.next().unwrap(), id_chars.next().unwrap(), id_chars.next().unwrap()];

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

        Self {
            tiles,
            width,
            height,
        }
    }
}

pub struct Tile {
    tile_id: TileId,
    tile_string_alternative_id: usize,
}

impl State {
    pub fn new() -> Self {
        let cursor_pos = Point::new(1, 1);
        let elapsed_time = 0;
        let tile_config = TileConfig::from("tile_config.toml");
        let map = Map::new(
            &tile_config,
            Vec::from(TEST_MAP_TILES),
            TEST_MAP_WIDTH,
            TEST_MAP_HEIGHT,
        );
        let map_pos = Point::new(72, 1);

        Self {
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
