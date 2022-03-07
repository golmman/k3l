use std::collections::HashSet;

use super::State;
use crate::common::MapPoint;

impl State {
    pub fn flood_fill_map(&self, start: MapPoint) -> HashSet<MapPoint> {
        self.flood_fill_map_recursive(start, HashSet::new())
    }

    #[allow(clippy::let_and_return)]
    fn flood_fill_map_recursive(
        &self,
        start: MapPoint,
        mut fill: HashSet<MapPoint>,
    ) -> HashSet<MapPoint> {
        if fill.contains(&start) || !self.is_tile_traversable(&start) {
            return fill;
        }

        fill.insert(start.clone());

        let fill = self.flood_fill_map_recursive(start.left(), fill);
        let fill = self.flood_fill_map_recursive(start.right(), fill);
        let fill = self.flood_fill_map_recursive(start.up(), fill);
        let fill = self.flood_fill_map_recursive(start.down(), fill);

        fill
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill_map_corner() {
        let state = State::new();

        let x = state.flood_fill_map(MapPoint::new(0, 0));
        assert_eq!(x.len(), 0);

        let x = state.flood_fill_map(MapPoint::new(9, 0));
        assert_eq!(x.len(), 0);
    }

    #[test]
    fn test_flood_fill_map_hall_left() {
        let state = State::new();

        let x = state.flood_fill_map(MapPoint::new(2, 2));
        assert_eq!(x.len(), 52);

        let x = state.flood_fill_map(MapPoint::new(3, 3));
        assert_eq!(x.len(), 52);
    }

    #[test]
    fn test_flood_fill_map_hall_right() {
        let state = State::new();

        let x = state.flood_fill_map(MapPoint::new(18, 7));
        assert_eq!(x.len(), 50);

        let x = state.flood_fill_map(MapPoint::new(19, 7));
        assert_eq!(x.len(), 50);
    }
}
