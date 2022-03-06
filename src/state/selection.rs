use crate::common::MapPoint;

#[derive(Debug)]
pub struct Selection {
    pub pos: Option<MapPoint>,
    pub size: Option<MapPoint>,
}

impl Selection {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            pos: None,
            size: None,
        }
    }

    pub fn normalize(&mut self) {
        if let Selection {
            pos: Some(pos),
            size: Some(size),
        } = self
        {
            if size.x < 0 {
                pos.x += size.x;
                size.x = -size.x;
            }
            if size.y < 0 {
                pos.y += size.y;
                size.y = -size.y;
            }
        }
    }
}
