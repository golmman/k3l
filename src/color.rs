// usage of termions 'Color' module is awkward
// see https://gitlab.redox-os.org/redox-os/termion/-/issues/123

// see
// https://stackoverflow.com/questions/4842424/list-of-ansi-color-escape-sequences
// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub bg_color: u8,
    pub fg_color: u8,
}

impl Color {
    pub const RESET: &'static str = "\x1b[0m";

    pub fn null() -> Self {
        Self {
            bg_color: 0,
            fg_color: 0,
        }
    }

    pub fn text() -> Self {
        Self {
            bg_color: 0,
            fg_color: 7,
        }
    }
}

impl From<&Color> for String {
    fn from(color: &Color) -> Self {
        let bg_color = color.bg_color;
        let fg_color = color.fg_color;
        format!("\x1b[38;5;{fg_color};48;5;{bg_color}m")
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
