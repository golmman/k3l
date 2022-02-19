use std::marker::PhantomData;
use std::ops::Deref;

use crate::controller::Controller;

mod color;
mod common;
mod controller;
mod renderer;
mod screen;
mod state;
mod tile_config;

////////////////////////
//struct ScreenCoordinate(i32);
//
//impl std::ops::Deref for ScreenCoordinate {
//    type Target = i32;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
//
//impl From<i32> for ScreenCoordinate {
//    fn from(i: i32) -> Self {
//        Self(i)
//    }
//}
//
//struct MapCoordinate(i32);
//
//impl std::ops::Deref for MapCoordinate {
//    type Target = i32;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}
//
//impl From<i32> for MapCoordinate {
//    fn from(i: i32) -> Self {
//        Self(i)
//    }
//}
//
//struct Point<W: Deref<Target = i32> + From<i32>> {
//    pub x: W,
//    pub y: W,
//}
//
//impl<W: Deref<Target = i32> + From<i32>> Point<W> {
//    fn new(x: i32, y: i32) -> Self {
//        Self {
//            x: W::from(x),
//            y: W::from(y),
//        }
//    }
//
//    pub fn width(&self) -> i32 {
//        *self.x
//    }
//}
//
//impl From<Point<ScreenCoordinate>> for Point<MapCoordinate> {
//    fn from(p: Point<ScreenCoordinate>) -> Self {
//        Point::new(*p.x / 3, *p.y)
//    }
//}

//////////////////////////////////////////////////////////


fn main() {
    let mut controller = Controller::new();
    controller.run();
}
