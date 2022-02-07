use std::cmp::max;
use std::cmp::min;
use std::io::Write;

use termion::color::Fg;
use termion::color::Green;
use termion::color::Reset;

use crate::common::frame_string;
use crate::screen::DefaultScreen;
use crate::state::State;

pub struct Renderer {
    pub screen: DefaultScreen,
    screen_buffer: Vec<u8>,
}

impl Renderer {
    pub fn new() -> Self {
        let screen = DefaultScreen::new();
        let screen_buffer = Vec::<u8>::new();

        Self {
            screen,
            screen_buffer,
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen_buffer.clear();
        write!(self.screen_buffer, "{}", termion::clear::All,).unwrap();
    }

    pub fn draw(&mut self, state: &State) {
        self.clear_screen();

        self.draw_floor();
        self.draw_map(state);
        self.draw_debug_info(state);
        self.draw_cursor(state);

        self.screen_buffer.flush().unwrap();

        // double buffering
        self.screen
            .write_all(&self.screen_buffer)
            .unwrap();
        self.screen.flush().unwrap();
    }

    fn draw_cursor(&mut self, state: &State) {
        write!(
            self.screen_buffer,
            "{}{}X{}",
            termion::cursor::Goto(state.cursor_pos.x, state.cursor_pos.y),
            Fg(Green),
            Fg(Reset),
        )
        .unwrap();
    }

    fn draw_debug_info(&mut self, state: &State) {
        write!(self.screen_buffer, "{}", termion::cursor::Goto(10, 3),).unwrap();
        write!(
            self.screen_buffer,
            "cols: {}, rows: {}, time: {}",
            self.screen.cols, self.screen.rows, state.elapsed_time,
        )
        .unwrap();

        write!(self.screen_buffer, "{}", termion::cursor::Goto(10, 4),).unwrap();
        write!(
            self.screen_buffer,
            "map_x: {}, map_y: {}",
            state.map_pos.x, state.map_pos.y,
        )
        .unwrap();
    }

    fn draw_floor(&mut self) {
        for y in 0..self.screen.rows {
            write!(
                self.screen_buffer,
                "{}{}",
                termion::cursor::Goto(1, y + 1),
                ".".repeat(self.screen.cols.into())
            )
            .unwrap();
        }
    }

    fn draw_map(&mut self, state: &State) {
        let map_x = state.map_pos.x;
        let map_y = state.map_pos.y;
        let screen_cols = self.screen.cols;
        let screen_rows = self.screen.rows;

        for row in 0..state.map.tiles.len() {
            let row = row as u16;
            let full_row = state.map.get_row(row);

            let displayed_row = frame_string(&full_row, map_x, screen_cols);

            if displayed_row.is_empty() {
                continue;
            }

            if map_y + (row as i16) < 1 || map_y + (row as i16) > screen_rows as i16 {
                continue;
            }

            let goto_x = min(screen_cols, max(1, map_x) as u16) as u16;
            let goto_y = min(screen_rows, max(1, map_y + row as i16) as u16) as u16;

            write!(
                self.screen_buffer,
                "{}{}",
                termion::cursor::Goto(goto_x, goto_y),
                displayed_row,
            )
            .unwrap();
        }
    }
}
