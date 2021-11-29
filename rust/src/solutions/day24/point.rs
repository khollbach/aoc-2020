use std::ops::{Add, AddAssign};
use super::dir::DIRS;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn neighbors(self) -> impl Iterator<Item=Point> {
        DIRS.iter().map(move |d| self + d.to_point())
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = *self + other
    }
}
