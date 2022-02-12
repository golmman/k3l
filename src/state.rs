use std::io::Write;
use termion::color::Bg;
use termion::color::Color;
use termion::color::Fg;

use crate::color::reset;
use crate::common::Point;
use crate::common::FRAMES_PER_SECOND;
use crate::common::PIXEL_H;
use crate::common::PIXEL_W;
use crate::common::TEST_MAP_HEIGHT;
use crate::common::TEST_MAP_TILES;
use crate::common::TEST_MAP_WIDTH;
use crate::common::calc_array_bounds;
use crate::tile_config::BaseTile;
use crate::tile_config::TileConfig;
use crate::tile_config::TileKind;

pub struct State {
    pub cursor_pos: Point<u16>,
    pub elapsed_time: u64,
    pub map: Map,
    pub map_pos: Point<i16>,
    pub tile_config: TileConfig,

    screen_cols: u16,
    screen_rows: u16,
}

pub struct Map {
    pub tiles: Vec<Tile>,
    pub width: u16,
    pub height: u16,
}

impl Map {
    pub fn new(tile_kinds: Vec<u8>, width: u16, height: u16) -> Self {
        let tiles = tile_kinds
            .into_iter()
            .map(Tile::from)
            .collect();

        Self {
            tiles,
            width,
            height,
        }
    }
}

pub struct Tile {
    tile_kind: TileKind,
    pixel_index: usize,
}

impl From<u8> for Tile {
    fn from(s: u8) -> Self {
        Self {
            tile_kind: unsafe { std::mem::transmute(s) },
            pixel_index: 0,
        }
    }
}

impl State {
    pub fn new(screen_cols: u16, screen_rows: u16) -> Self {
        let cursor_pos = Point::new(1, 1);
        let elapsed_time = 0;
        let tile_config = TileConfig::from("tile_config.toml");
        let map = Map::new(Vec::from(TEST_MAP_TILES), TEST_MAP_WIDTH, TEST_MAP_HEIGHT);
        let map_pos = Point::new(1, 6);

        Self {
            cursor_pos,
            elapsed_time,
            map,
            map_pos,
            tile_config,

            screen_cols,
            screen_rows,
        }
    }

    pub fn get_map_row(&self, row: u16) -> String {
        let mut s = String::new();

        let (skip, take) = calc_array_bounds(self.map.width, self.map_pos.x, self.screen_cols);

        for i in skip..take {
            let index = self.map.width * row + i;

            let tile_kind = self.map.tiles[index as usize].tile_kind;
            let pixel_index = self.map.tiles[index as usize].pixel_index;
            let frame = (self.elapsed_time % FRAMES_PER_SECOND as u64) as usize;

            let tile_str = &self.tile_config.get(tile_kind).pixels[pixel_index].frames[frame];
            let color = &self.tile_config.get(tile_kind).color;
            let reset = reset();

            s.push_str(&format!("{color}{tile_str}{reset}"));
        }

        s
    }

    pub fn resize(&mut self, screen_cols: u16, screen_rows: u16) {
        self.screen_cols = screen_cols;
        self.screen_rows = screen_rows;
    }

    pub fn elapse_time(&mut self) {
        self.elapsed_time += 1;
    }

    pub fn move_map_left(&mut self) {
        self.map_pos.x -= PIXEL_W as i16;
    }

    pub fn move_map_right(&mut self) {
        self.map_pos.x += PIXEL_W as i16;
    }

    pub fn move_map_up(&mut self) {
        self.map_pos.y -= PIXEL_H as i16;
    }

    pub fn move_map_down(&mut self) {
        self.map_pos.y += PIXEL_H as i16;
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos.x < 1 + PIXEL_W {
            self.move_map_right();
            return;
        }
        self.cursor_pos.x -= PIXEL_W;

        // align cursor to pixels
        self.cursor_pos.x = ((self.cursor_pos.x - 1) / PIXEL_W) * PIXEL_W + PIXEL_W / 2 + 1;
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_pos.x + PIXEL_W > (self.screen_cols / PIXEL_W) * PIXEL_W {
            self.move_map_left();
            return;
        }
        self.cursor_pos.x += PIXEL_W;

        // align cursor to pixels
        self.cursor_pos.x = ((self.cursor_pos.x - 1) / PIXEL_W) * PIXEL_W + PIXEL_W / 2 + 1;
    }

    pub fn move_cursor_up(&mut self) {
        // TODO: align to pixel
        if self.cursor_pos.y < 1 + PIXEL_H {
            self.move_map_down();
            return;
        }
        self.cursor_pos.y -= PIXEL_H;
    }

    pub fn move_cursor_down(&mut self) {
        // TODO: align to pixel
        if self.cursor_pos.y + PIXEL_H > self.screen_rows {
            self.move_map_up();
            return;
        }
        self.cursor_pos.y += PIXEL_H;
    }
}
