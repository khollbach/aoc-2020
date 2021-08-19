use std::fmt;
use Direction::{Down, Up, Right, Left};
use super::Pixel::{Black, White, self};
use std::cmp::min;
use super::Tile;

/// A bitmask representing the pattern of pixels on a border.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Border(u32);

impl Border {
    const NUM_PIXELS: usize = 10;

    fn new(pixels: impl Iterator<Item=Pixel>) -> Border {
        let mut acc = 0;
        for p in pixels {
            acc <<= 1;
            acc |= match p {
                Black => 0,
                White => 1,
            };
        }
        Border(acc)
    }

    fn flip(self) -> Border {
        let original = self.0;

        let mut flipped = 0;
        for i in 0..Border::NUM_PIXELS {
            let j = Border::NUM_PIXELS - 1 - i;

            if original & (1 << i) != 0 {
                flipped |= 1 << j;
            }
        }

        Border(flipped)
    }

    pub fn normalize(self) -> Border {
        min(self, self.flip())
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    pub fn flip(self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

pub const DIRS: [Direction; 4] = [Up, Right, Down, Left];

impl Tile {
    /// This tile's 4 borders, in any order.
    pub fn borders(&self) -> impl Iterator<Item=Border> + '_ {
        DIRS.iter().map(move |&d| self.border(d))
    }

    /// Each border is always read from left-to-right, or top-to-bottom.
    ///
    /// This is important, so that we can check equality of two side-by-side borders.
    /// E.g. the right border of a tile versus the left border of the tile next to it.
    pub fn border(&self, dir: Direction) -> Border {
        let n = self.pixels.len();
        match dir {
            Up => Border::new((0..n).map(|i| self.pixels[0][i])),
            Down => Border::new((0..n).map(|i| self.pixels[n - 1][i])),
            Left => Border::new((0..n).map(|i| self.pixels[i][0])),
            Right => Border::new((0..n).map(|i| self.pixels[i][n - 1])),
        }
    }

    /// Does this tile have the given border?
    ///
    /// We normalize so that it doesn't matter if the tile is "face up" or "face down".
    pub fn has_border(&self, border: Border) -> bool {
        let normalized = border.normalize();
        self.borders().any(|b| b.normalize() == normalized)
    }
}

impl fmt::Debug for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0width$b}", self.0, width=Border::NUM_PIXELS)
    }
}
