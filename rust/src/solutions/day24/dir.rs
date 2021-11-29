use super::point::Point;

#[derive(Debug, Copy, Clone)]
pub enum Dir {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

use Dir::*;
pub const DIRS: [Dir; 6] = [E, W, NE, NW, SE, SW];

impl Dir {
    pub fn new(s: &str) -> Dir {
        use Dir::*;
        match s {
            "e" => E,
            "w" => W,
            "ne" => NE,
            "nw" => NW,
            "se" => SE,
            "sw" => SW,
            _ => panic!("Invalid dir string: {}", s),
        }
    }

    pub fn to_point(self) -> Point {
        use Dir::*;
        let (x, y) = match self {
            E => (1, 0),
            W => (-1, 0),

            NW => (0, 1),
            SE => (0, -1),

            NE => (1, 1),
            SW => (-1, -1),
        };

        Point { x, y }
    }
}
