use crate::common::Point;

pub struct State {
    pub cursor_pos: Point,
}

impl State {
    pub fn new() -> Self {
        let cursor_pos = Point::new(1, 1);
        Self { cursor_pos }
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
