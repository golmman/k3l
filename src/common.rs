use std::cmp::max;
use std::cmp::min;

pub const PIXEL_W: u16 = 3;
pub const PIXEL_H: u16 = 1;
pub const FRAMES_PER_SECOND: u16 = 8;

#[rustfmt::skip]
pub const TEST_MAP_TILES: [u8; (TEST_MAP_WIDTH * TEST_MAP_HEIGHT) as usize] = [
    1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 2, 1,
    1, 0, 0, 2, 2, 1,
    1, 0, 0, 2, 2, 1,
    1, 0, 2, 0, 0, 1,
    1, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1,
];
pub const TEST_MAP_WIDTH: u16 = 6;
pub const TEST_MAP_HEIGHT: u16 = 7;

pub struct Point<W> {
    pub x: W,
    pub y: W,
}

impl<W> Point<W> {
    pub fn new(x: W, y: W) -> Self {
        Self { x, y }
    }
}

pub fn frame_string(s: &str, position: i16, width: u16) -> String {
    let l = s.len() as i16;
    let p = position;
    let w = width as i16;

    let skip = max(0, 1 - p);
    let take = min(min(w, l), min(w - p + 1, l + p - 1));

    if skip >= l || take <= 0 {
        return "".to_string();
    }

    let frame_string: String = s
        .chars()
        .skip(skip as usize)
        .take(take as usize)
        .collect();

    frame_string
}

// similar to the function above, picture it like this:
//
//    position (=-4)   width (=12, e.g. screen/terminal width)
//           \      .----^-----.
//            \    |            |
//             some_string
//             '---.-----'
//                len (=11)
//
// results in skip=5, take=6; so (5, 6)
pub fn calc_array_bounds(len: u16, position: i16, width: u16) -> (u16, u16) {
    let l = len as i16;
    let p = position;
    let w = width as i16;

    let skip = max(0, 1 - p);
    let take = min(min(w, l), min(w - p + 1, l + p - 1));

    if skip >= l || take <= 0 {
        return (0, 0);
    }

    (skip as u16, take as u16)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_array_bounds() {
        assert_eq!((0, 0), calc_array_bounds(10, 7, 5));
        assert_eq!((0, 4), calc_array_bounds(10, 2, 5));
        assert_eq!((0, 5), calc_array_bounds(10, 1, 5));

        assert_eq!((1, 5), calc_array_bounds(10, 0, 5));
        assert_eq!((2, 5), calc_array_bounds(10, -1, 5));

        assert_eq!((7, 3), calc_array_bounds(10, -6, 5));
        assert_eq!((0, 0), calc_array_bounds(10, -60, 5));

        assert_eq!((0, 0), calc_array_bounds(10, -60, 20));
        assert_eq!((7, 3), calc_array_bounds(10, -6, 20));
        assert_eq!((0, 10), calc_array_bounds(10, 1, 20));
        assert_eq!((0, 10), calc_array_bounds(10, 8, 20));
        assert_eq!((0, 10), calc_array_bounds(10, 11, 20));
        assert_eq!((0, 9), calc_array_bounds(10, 12, 20));
        assert_eq!((0, 6), calc_array_bounds(10, 15, 20));
        assert_eq!((0, 1), calc_array_bounds(10, 20, 20));
        assert_eq!((0, 0), calc_array_bounds(10, 21, 20));
    }

    #[test]
    fn test_frame_string() {
        let x = "1234567890";

        assert_eq!("".to_string(), frame_string(x, 7, 5));
        assert_eq!("1234".to_string(), frame_string(x, 2, 5));
        assert_eq!("12345".to_string(), frame_string(x, 1, 5));

        assert_eq!("23456".to_string(), frame_string(x, 0, 5));
        assert_eq!("34567".to_string(), frame_string(x, -1, 5));

        assert_eq!("890".to_string(), frame_string(x, -6, 5));
        assert_eq!("".to_string(), frame_string(x, -60, 5));

        assert_eq!("".to_string(), frame_string(x, -60, 20));
        assert_eq!("890".to_string(), frame_string(x, -6, 20));
        assert_eq!("1234567890".to_string(), frame_string(x, 1, 20));
        assert_eq!("1234567890".to_string(), frame_string(x, 8, 20));
        assert_eq!("1234567890".to_string(), frame_string(x, 11, 20));
        assert_eq!("123456789".to_string(), frame_string(x, 12, 20));
        assert_eq!("123456".to_string(), frame_string(x, 15, 20));
        assert_eq!("1".to_string(), frame_string(x, 20, 20));
        assert_eq!("".to_string(), frame_string(x, 21, 20));
    }
}
