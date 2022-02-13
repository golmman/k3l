use std::cmp::max;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::color::color;
use crate::color::reset;
use crate::common::Rect;

pub type Sprite = Vec<Vec<ScreenChar>>;
pub type DefaultScreen = Screen<RawTerminal<Stdout>>;

#[derive(Clone)]
struct Color {
    fg: u8,
    bg: u8,
}

impl Color {
    fn new() -> Self {
        Self { fg: 0, bg: 0 }
    }
}

#[derive(Clone, Copy)]
pub struct ScreenChar {
    ch: char,
    bg_color: u8,
    fg_color: u8,
}

impl ScreenChar {
    pub fn new(ch: char, bg_color: u8, fg_color: u8) -> Self {
        Self {
            ch,
            bg_color,
            fg_color,
        }
    }

    pub fn from_str(s: &str) -> Vec<Vec<ScreenChar>> {
        let mut rows = Vec::new();
        let mut row = Vec::new();

        for ch in s.chars() {
            row.push(Self {
                bg_color: 0,
                fg_color: 7,
                ch,
            });
        }

        rows.push(row);
        rows
    }
}

impl From<char> for ScreenChar {
    fn from(ch: char) -> Self {
        Self {
            bg_color: 0,
            fg_color: 7,
            ch,
        }
    }
}

pub struct Screen<W: Write> {
    pub main_display: W,
    prelude_buffer: String,
    pixel_buffer: Vec<ScreenChar>,

    pub cols: u16,
    pub rows: u16,
}

impl DefaultScreen {
    pub fn new() -> Self {
        Screen::from(stdout().into_raw_mode().unwrap())
    }

    pub fn resize(&mut self) {
        let (cols, rows) = termion::terminal_size().unwrap();
        self.cols = cols;
        self.rows = rows;
    }

    pub fn clear(&mut self) {
        let buffer_size = (self.cols * self.rows) as usize;
        self.prelude_buffer = String::new();
        self.pixel_buffer = vec![ScreenChar::from(' '); buffer_size];

        //self.prelude_buffer.push_str("\x1b[2J"); // clear screen
        //self.prelude_buffer.push_str("\x1b[H"); // goto to (1, 1)
    }

    pub fn draw(&mut self, sprite: &Sprite, x: i16, y: i16) {
        for (y0, sprite_row) in sprite.iter().enumerate() {
            for (x0, pixel) in sprite_row.iter().enumerate() {
                let i = (self.cols as i16 * (y + y0 as i16) + x + x0 as i16) as usize;
                self.pixel_buffer[i] = *pixel;
            }
        }
    }

    pub fn display(&mut self) {
        let mut s = String::new();
        let reset = reset();

        s.push_str(&self.prelude_buffer);

        for y in 0..self.rows {
            let row = y + 1;
            s.push_str(&format!("\x1b[{row};1H")); // goto (row, 1)

            let mut last_color = (0 as u8, 0 as u8);
            for x in 0..self.cols {
                let i = (self.cols * y + x) as usize;
                let bg_color = self.pixel_buffer[i].bg_color;
                let fg_color = self.pixel_buffer[i].fg_color;
                let ch = self.pixel_buffer[i].ch;
                let color = color(bg_color, fg_color);

                if last_color == (bg_color, fg_color) {
                    s.push(ch);
                } else {
                    s.push_str(&format!("{reset}{color}{ch}"));
                }

                last_color = (bg_color, fg_color);
            }
        }

        s.push_str(&format!("{reset}"));

        self.main_display
            .write_all(s.as_bytes())
            .unwrap();
        self.main_display.flush().unwrap();
    }

    fn clip_sprite(&mut self, sprite: &Sprite, x: i16, y: i16) -> Rect<i16> {
        Rect::new(0, 0, 0, 0)
    }
}

impl<W: Write> From<W> for Screen<W> {
    fn from(mut buffer: W) -> Self {
        write!(
            buffer,
            "{}{}{}",
            termion::cursor::Hide,
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        buffer.flush().unwrap();

        let (cols, rows) = termion::terminal_size().unwrap();
        let buffer_size = (cols * rows) as usize;

        let prelude_buffer = String::new();
        let pixel_buffer = vec![ScreenChar::from(' '); buffer_size];

        Self {
            main_display: buffer,
            prelude_buffer,
            pixel_buffer,
            cols,
            rows,
        }
    }
}

impl<W: Write> Drop for Screen<W> {
    fn drop(&mut self) {
        write!(
            self,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        )
        .unwrap();

        self.flush().unwrap();
    }
}

impl<W: Write> Write for Screen<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.main_display.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.main_display.flush()
    }
}
