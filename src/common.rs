use std::cmp::max;
use std::cmp::min;

pub const TILE_W: u16 = 3;
pub const TILE_H: u16 = 1;
pub const FRAMES_PER_SECOND: u16 = 8;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point<W> {
    pub x: W,
    pub y: W,
}

impl<W> Point<W> {
    pub fn new(x: W, y: W) -> Self {
        Self { x, y }
    }
}

impl Point<i16> {
    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}

#[derive(Debug)]
pub struct RectAbsolute<W> {
    pub x1: W,
    pub y1: W,
    pub x2: W,
    pub y2: W,
}
pub fn intersect(r1: &RectAbsolute<i16>, r2: &RectAbsolute<i16>) -> RectAbsolute<i16> {
    let x1 = max(r1.x1, r2.x1);
    let y1 = max(r1.y1, r2.y1);
    let x2 = min(r1.x2, r2.x2);
    let y2 = min(r1.y2, r2.y2);

    RectAbsolute { x1, y1, x2, y2 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_array_bounds() {}
}
