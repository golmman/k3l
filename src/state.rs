use crate::common::Point;

pub struct State {
    pub cursor_pos: Point<u16>,
    pub elapsed_time: u64,
    pub map: Map,
    pub map_pos: Point<i16>,
}

pub struct Map {
    pub tiles: Vec<Vec<MapTile>>,
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
        let mut tiles = Vec::<Vec<MapTile>>::new();

        for chars_row in chars.split_terminator("ENDL") {
            let mut tiles_row = Vec::<MapTile>::new();

            for chars_col in chars_row.chars() {
                tiles_row.push(MapTile::from(chars_col));
            }

            tiles.push(tiles_row);
        }

        Self {
            tiles,
        }
    }
}

pub struct MapTile {
    pub displayed_char: char,
}

impl MapTile {
    pub fn new(displayed_char: char) -> Self {
        Self { displayed_char }
    }
}

impl From<char> for MapTile {
    fn from(displayed_char: char) -> Self {
        MapTile::new(displayed_char)
    }
}

impl State {
    pub fn new() -> Self {
        let cursor_pos = Point::new(1, 1);
        let elapsed_time = 0;
        let map = Map::new();
        let map_pos = Point::new(15, 6);

        Self {
            cursor_pos,
            elapsed_time,
            map,
            map_pos,
        }
    }

    pub fn move_map_left(&mut self) {
        self.map_pos.x -= 1;
    }

    pub fn move_map_right(&mut self) {
        self.map_pos.x += 1;
    }

    pub fn move_map_up(&mut self) {
        self.map_pos.y -= 1;
    }

    pub fn move_map_down(&mut self) {
        self.map_pos.y += 1;
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor_pos.x -= 1;
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor_pos.x += 1;
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor_pos.y -= 1;
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor_pos.y += 1;
    }
}
