use std::fmt;
use crate::Res;

pub mod border;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileId(pub u32);

#[derive(Clone)]
pub struct Tile {
    pub id: TileId,
    pub pixels: Vec<Vec<Pixel>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Black,
    White,
}

impl Tile {
    pub fn new(id: u32, pixels: Vec<Vec<Pixel>>) -> Tile {
        Tile {
            id: TileId(id),
            pixels,
        }
    }

    /// Flip this tile horizontally, i.e. "mirror" it left-to-right.
    pub fn flip(&mut self) {
        for row in &mut self.pixels {
            row.reverse();
        }
    }

    /// Rotate this tile counter-clockwise, in place.
    pub fn rotate_ccw(&mut self) {
        let n = self.pixels.len();

        let height = n / 2;
        let width = n / 2 + n % 2;

        for i in 0..height {
            for j in 0..width {
                let tl = Point(i, j);
                let tr = Point(j, n - 1 - i);
                let br = Point(n - 1 - i, n - 1 - j);
                let bl = Point(n - 1 - j, i);

                four_way_swap(&mut self.pixels, tl, tr, br, bl);
            }
        }
    }
}

/// Helper for `Tile::rotate_ccw`.
#[derive(Copy, Clone)]
struct Point(usize, usize);

/// Helper for `Tile::rotate_ccw`.
///
/// Rotate a point counter-clockwise in all four quadrants at once.
fn four_way_swap(m: &mut Vec<Vec<Pixel>>, tl: Point, tr: Point, br: Point, bl: Point) {
    fn get(matrix: &mut Vec<Vec<Pixel>>, p: Point) -> &mut Pixel {
        &mut matrix[p.0][p.1]
    }

    let tmp = *get(m, tl);
    *get(m, tl) = *get(m, tr);
    *get(m, tr) = *get(m, br);
    *get(m, br) = *get(m, bl);
    *get(m, bl) = tmp;
}

impl Pixel {
    pub fn new(c: char) -> Res<Pixel> {
        match c {
            '.' => Ok(Pixel::Black),
            '#' => Ok(Pixel::White),
            _ => Err(format!("Invalid pixel: {}", c).into()),
        }
    }

    fn to_char(self) -> char {
        match self {
            Pixel::Black => '.',
            Pixel::White => '#',
        }
    }
}

impl fmt::Debug for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile {}:", self.id.0)?;
        for row in &self.pixels {
            let s: String = row.iter().map(|pixel| pixel.to_char()).collect();
            write!(f, "\n{}", s)?;
        }
        Ok(())
    }
}
