use std::cmp::max;
use std::cmp::min;
use std::io::Write;

use termion::color::Bg;
use termion::color::Green;
use termion::color::Reset;

use crate::common::frame_string;
use crate::screen::DefaultScreen;
use crate::screen::ScreenChar;
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

    pub fn display(&mut self, state: &State) {
        self.screen.clear();

        self.draw_floor();
        self.draw_map(state);
        self.draw_debug_info(state);
        self.draw_cursor(state);

        self.screen.display();
    }

    fn draw_cursor(&mut self, state: &State) {
        let cursor = vec![vec![ScreenChar::new('X', 2, 0)]];

        self.screen.draw(
            &cursor,
            state.cursor_pos.x as i16,
            state.cursor_pos.y as i16,
        );
    }

    fn draw_debug_info(&mut self, state: &State) {
        let state_info_str = format!(
            "cols: {}, rows: {}, time: {}",
            self.screen.cols, self.screen.rows, state.elapsed_time,
        );
        let state_info = ScreenChar::from_str(&state_info_str);
        self.screen.draw(&state_info, 10, 3);

        let pos_info_str = format!(
            "map_x: {}, map_y: {}, cursor_x: {}, cursor_y: {}",
            state.map_pos.x, state.map_pos.y, state.cursor_pos.x, state.cursor_pos.y
        );
        let pos_info = ScreenChar::from_str(&pos_info_str);
        self.screen.draw(&pos_info, 10, 4);
    }

    fn draw_floor(&mut self) {
        let mut pixels = Vec::new();

        for y in 0..self.screen.rows {
            let mut row = Vec::new();
            for x in 0..self.screen.cols / 3 {
                row.push(ScreenChar::new('[', 0, 7));
                row.push(ScreenChar::new('-', 0, 7));
                row.push(ScreenChar::new(']', 0, 7));
            }
            pixels.push(row);
        }

        self.screen.draw(&pixels, 0, 0);
    }

    fn draw_map(&mut self, state: &State) {
        let sprite = state.get_map_sprite();

        self.screen.draw(&sprite, state.map_pos.x, state.map_pos.y);

        //let map_x = state.map_pos.x;
        //let map_y = state.map_pos.y;
        //let screen_cols = self.screen.cols;
        //let screen_rows = self.screen.rows;

        //for row in 0..state.map.height {
        //    let row = row as u16;

        //    let displayed_row = String:: new(); //state.get_map_row(row);

        //    if displayed_row.is_empty() {
        //        continue;
        //    }

        //    if map_y + (row as i16) < 1 || map_y + (row as i16) > screen_rows as i16 {
        //        continue;
        //    }

        //    let goto_x = min(screen_cols, max(1, map_x) as u16) as u16;
        //    let goto_y = min(screen_rows, max(1, map_y + row as i16) as u16) as u16;

        //    write!(
        //        self.screen_buffer,
        //        "{}{}",
        //        termion::cursor::Goto(goto_x, goto_y),
        //        displayed_row,
        //    )
        //    .unwrap();
        //}
    }
}
