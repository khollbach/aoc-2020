use crate::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::{fmt, io};
use std::iter;

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

/// Read the input into a collection of Tiles keyed by TileId.
pub fn read_input(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<HashMap<TileId, Tile>> {
    let mut tiles = HashMap::new();
    while let Some(tile) = read_tile(&mut lines)? {
        let ret = tiles.insert(tile.id, tile);
        if let Some(other) = ret {
            return Err(format!("Repeated tile id: {}", other.id.0).into());
        }
    }
    Ok(tiles)
}

/// Consume a single tile (and the following blank line "separator") from the input stream.
///
/// Returns Ok(None) on end-of-stream.
fn read_tile(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Option<Tile>> {
    lazy_static! {
        static ref ID_RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    }

    let first_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(None),
    };

    let id: u32 = match ID_RE.captures(&first_line) {
        Some(caps) => caps[1].parse()?,
        None => return Err(format!("Line didn't match tile id regex: {}", first_line).into()),
    };

    let mut pixels = vec![];
    for line in lines.chain(iter::once(Ok(String::new()))) {
        let line = line?;
        if line == "" {
            return Ok(Some(Tile::new(id, pixels)));
        }

        let row: Res<Vec<Pixel>> = line.chars().map(Pixel::new).collect();
        pixels.push(row?);
    }
    unreachable!()
}

impl Tile {
    fn new(id: u32, pixels: Vec<Vec<Pixel>>) -> Tile {
        Tile {
            id: TileId(id),
            pixels,
        }
    }
}

impl Pixel {
    fn new(c: char) -> Res<Pixel> {
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
        for row in self.pixels.iter() {
            let s: String = row.iter().map(|pixel| pixel.to_char()).collect();
            write!(f, "\n{}", s)?;
        }
        Ok(())
    }
}
