use std::cmp::max;
use std::cmp::min;

pub const TILE_SIZE: Size2d = Size2d { width: 3, height: 1 };
pub const FRAMES_PER_SECOND: u16 = 8;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Size2d {
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PixelPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TilePoint {
    pub x: i32,
    pub y: i32,
}

impl TilePoint {
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

pub fn intersect(r1: &RectAbsolute<i32>, r2: &RectAbsolute<i32>) -> RectAbsolute<i32> {
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
