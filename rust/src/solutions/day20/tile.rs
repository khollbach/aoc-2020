use std::fmt;
use crate::Res;

mod border;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileId(pub u32);

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
