use std::cmp::max;
use std::cmp::min;
use std::io::Write;

use termion::color::Bg;
use termion::color::Green;
use termion::color::Reset;

use crate::common::PIXEL_W;
use crate::common::frame_string;
use crate::screen::DefaultScreen;
use crate::screen::ScreenChar;
use crate::screen::Sprite;
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
        let pixels = vec![ScreenChar::new('X', 2, 0)];
        let width = 1;
        let height = 1;

        let cursor = Sprite {
            screen_chars: pixels,
            width,
            height,
        };

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
        let state_info = Sprite::from(state_info_str);
        self.screen.draw(&state_info, 10, 3);

        let pos_info_str = format!(
            "map_x: {}, map_y: {}, cursor_x: {}, cursor_y: {}",
            state.map_pos.x, state.map_pos.y, state.cursor_pos.x, state.cursor_pos.y
        );
        let pos_info = Sprite::from(pos_info_str);
        self.screen.draw(&pos_info, 10, 4);
    }

    fn draw_floor(&mut self) {
        let mut pixels = Vec::new();
        let width = (self.screen.cols / PIXEL_W) * PIXEL_W;
        let height = self.screen.rows;

        for i in 0..((width / PIXEL_W) * height) {
            pixels.push(ScreenChar::new('[', 0, 7));
            pixels.push(ScreenChar::new('-', 0, 7));
            pixels.push(ScreenChar::new(']', 0, 7));
        }

        let sprite = Sprite {
            screen_chars: pixels,
            width,
            height,
        };

        self.screen.draw(&sprite, 0, 0);
    }

    fn draw_map(&mut self, state: &State) {
        let sprite = state.get_map_sprite();

        self.screen
            .draw(&sprite, state.map_pos.x, state.map_pos.y);

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
