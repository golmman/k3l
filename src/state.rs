use termion::color::Color;

use crate::common::Point;
use crate::common::PIXEL_H;
use crate::common::PIXEL_W;

pub struct State {
    pub cursor_pos: Point<u16>,
    pub elapsed_time: u64,
    pub map: Map,
    pub map_pos: Point<i16>,

    screen_cols: u16,
    screen_rows: u16,
}

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new() -> Self {
        let mut map = String::new();
        map.push_str(r#" ___________________________ ENDL"#);
        map.push_str(r#"|                           |ENDL"#);
        map.push_str(r#"|        _    _____ _       |ENDL"#);
        map.push_str(r#"|       | | _|___ /| |      |ENDL"#);
        map.push_str(r#"|       | |/ / |_ \| |      |ENDL"#);
        map.push_str(r#"|       |   < ___) | |      |ENDL"#);
        map.push_str(r#"|       |_|\_\____/|_|      |ENDL"#);
        map.push_str(r#"|                           |ENDL"#);
        map.push_str(r#"|___________________________|ENDL"#);
        Map::from(map)
    }

    pub fn get_row(&self, row: u16) -> String {
        let mut s = String::new();

        for map_tile in &self.tiles[row as usize] {
            s.push(map_tile.displayed_char);
        }

        s
    }
}

impl From<String> for Map {
    fn from(chars: String) -> Self {
        let mut tiles = Vec::<Vec<Tile>>::new();

        for chars_row in chars.split_terminator("ENDL") {
            let mut tiles_row = Vec::<Tile>::new();

            for chars_col in chars_row.chars() {
                tiles_row.push(Tile::from(chars_col));
            }

            tiles.push(tiles_row);
        }

        Self { tiles }
    }
}

pub struct Tile {
    pub displayed_char: char,

    pub bg_color: Box<dyn Color>,
}

impl Tile {
    pub fn new(displayed_char: char) -> Self {
        Self {
            displayed_char,
            bg_color: Box::new(termion::color::Black),
        }
    }
}

impl From<char> for Tile {
    fn from(displayed_char: char) -> Self {
        Tile::new(displayed_char)
    }
}

impl State {
    pub fn new(screen_cols: u16, screen_rows: u16) -> Self {
        let cursor_pos = Point::new(1, 1);
        let elapsed_time = 0;
        let map = Map::new();
        let map_pos = Point::new(15, 6);

        Self {
            cursor_pos,
            elapsed_time,
            map,
            map_pos,

            screen_cols,
            screen_rows,
        }
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
