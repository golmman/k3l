use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::color::Color;
use crate::common::intersect;
use crate::common::RectAbsolute;
use crate::common::ScreenPoint;

pub type DefaultScreen = Screen<RawTerminal<Stdout>>;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub ch: char,
    pub color: Color,
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        Self {
            color: Color::text(),
            ch,
        }
    }
}

pub struct Sprite {
    pub pixels: Vec<Pixel>,
    pub size: ScreenPoint,
}

impl Sprite {
    pub fn from_color_text(text: &str, color: Color) -> Self {
        let width = text.chars().count() as i32;
        let height = 1;
        let mut pixels = Vec::new();

        for ch in text.chars() {
            pixels.push(Pixel { ch, color });
        }

        Self {
            pixels,
            size: ScreenPoint::new(width, height),
        }
    }
}

impl From<&str> for Sprite {
    fn from(s: &str) -> Self {
        Self::from_color_text(
            s,
            Color {
                bg_color: 0,
                fg_color: 7,
            },
        )
    }
}

impl From<String> for Sprite {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

pub struct Screen<W: Write> {
    main_display: W,
    prelude_buffer: String,

    // TODO: make sprite?
    pixel_buffer: Vec<Pixel>,

    pub size: ScreenPoint,
}

impl DefaultScreen {
    pub fn new() -> Self {
        Screen::from(stdout().into_raw_mode().unwrap())
    }

    pub fn resize(&mut self) -> ScreenPoint {
        let (cols, rows) = termion::terminal_size().unwrap();

        self.size = ScreenPoint::new(cols as i32, rows as i32);

        self.size.clone()
    }

    pub fn clear(&mut self) {
        let buffer_size = (self.size.width() * self.size.height()) as usize;
        self.prelude_buffer = String::new();
        self.pixel_buffer = vec![Pixel::from(' '); buffer_size];

        //self.prelude_buffer.push_str("\x1b[2J"); // clear screen
        //self.prelude_buffer.push_str("\x1b[H"); // goto to (1, 1)
    }

    pub fn draw(&mut self, sprite: &Sprite, p: ScreenPoint) {
        let screen_rect = RectAbsolute {
            x1: 0,
            y1: 0,
            x2: self.size.width(),
            y2: self.size.height(),
        };

        let sprite_rect = RectAbsolute {
            x1: p.x,
            y1: p.y,
            x2: p.x + sprite.size.width(),
            y2: p.y + sprite.size.height(),
        };

        let intersection = intersect(&screen_rect, &sprite_rect);

        for sprite_y in intersection.y1..intersection.y2 {
            for sprite_x in intersection.x1..intersection.x2 {
                let screen_i = (self.size.width() * sprite_y + sprite_x) as usize;
                let sprite_i = (sprite.size.width() * (sprite_y - p.y) + sprite_x - p.x) as usize;

                self.pixel_buffer[screen_i] = sprite.pixels[sprite_i];
            }
        }
    }

    pub fn display(&mut self) {
        let mut s = String::new();
        let reset = Color::RESET;

        s.push_str(&self.prelude_buffer);

        for y in 0..self.size.height() {
            let row = y + 1;
            s.push_str(&format!("\x1b[{row};1H")); // goto (row, 1)

            // TODO: further optimization is possible, like recycling bg/fg color only
            let mut last_color = Color::null();
            for x in 0..self.size.width() {
                let i = (self.size.width() * y + x) as usize;
                let ch = self.pixel_buffer[i].ch;
                let color = self.pixel_buffer[i].color;

                if last_color == color {
                    s.push(ch);
                } else {
                    s.push_str(&format!("{reset}{color}{ch}"));
                }

                last_color = color;
            }
        }

        s.push_str(reset);

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
        let pixel_buffer = vec![Pixel::from(' '); buffer_size];

        Self {
            main_display: buffer,
            prelude_buffer,
            pixel_buffer,
            size: ScreenPoint::new(cols as i32, rows as i32),
        }
    }
}

impl<W: Write> Drop for Screen<W> {
    fn drop(&mut self) {
        write!(
            self.main_display,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show,
        )
        .unwrap();

        self.main_display.flush().unwrap();
    }
}
