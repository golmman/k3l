use std::cmp::max;
use std::cmp::min;

pub const PIXEL_W: u16 = 3;
pub const PIXEL_H: u16 = 1;
pub const FRAMES_PER_SECOND: u16 = 8;

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
    let take = min(min(w, l), w - p + 1);

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frame_string() {
        let x = "1234567890";

        assert_eq!("".to_string(), frame_string(x, 7, 5));
        assert_eq!("1234".to_string(), frame_string(x, 2, 5));
        assert_eq!("12345".to_string(), frame_string(x, 1, 5));

        assert_eq!("23456".to_string(), frame_string(x, 0, 5));
        assert_eq!("34567".to_string(), frame_string(x, -1, 5));

        // TOOD: function internally take is wrongly calculated as 5, but this does not seem to affect the desired functionality
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
