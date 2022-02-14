use std::cmp::max;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::color::color;
use crate::color::reset;
use crate::common::intersect;
use crate::common::Rect;

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

pub struct Sprite {
    // TODO: Rename "Pixel"?
    pub screen_chars: Vec<ScreenChar>,
    pub width: u16,
    pub height: u16,
}

impl From<&str> for Sprite {
    fn from(s: &str) -> Self {
        let width = s.chars().count() as u16;
        let height = 1;
        let mut screen_chars = Vec::new();

        for ch in s.chars() {
            screen_chars.push(ScreenChar {
                bg_color: 0,
                fg_color: 7,
                ch,
            });
        }

        Self {
            screen_chars,
            width,
            height,
        }
    }
}

impl From<String> for Sprite {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

pub struct Screen<W: Write> {
    pub main_display: W,
    prelude_buffer: String,

    // TODO: make sprite?
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
        let screen_rect = Rect {
            x: 0,
            y: 0,
            w: self.cols as i16,
            h: self.rows as i16,
        };

        let sprite_rect = Rect {
            x,
            y,
            w: sprite.width as i16,
            h: sprite.height as i16,
        };

        let intersection_sprite = intersect(&screen_rect, &sprite_rect);

        for sprite_y in intersection_sprite.y..(intersection_sprite.y + intersection_sprite.h) {
            for sprite_x in intersection_sprite.x..(intersection_sprite.x + intersection_sprite.w) {
                let screen_i = (self.cols as i16 * (sprite_y as i16) + sprite_x as i16) as usize;
                let sprite_i =
                    (sprite.width as i16 * (sprite_y as i16 - y) + sprite_x as i16 - x) as usize;

                self.pixel_buffer[screen_i] = sprite.screen_chars[sprite_i];
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
