use crate::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io::prelude::*;
use std::iter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileId(u32);

pub struct Tile {
    pub id: TileId,
    pub pixels: Vec<Vec<Pixel>>,
}

#[derive(Clone, Copy)]
pub enum Pixel {
    Black,
    White,
}

pub fn read_input(mut input: impl BufRead) -> Res<HashMap<TileId, Tile>> {
    let mut tiles = HashMap::new();
    while !input.fill_buf()?.is_empty() {
        let tile = read_tile(&mut input)?;
        let ret = tiles.insert(tile.id, tile);
        if let Some(other) = ret {
            return Err(format!("Repeated tile id: {}", other.id.0).into());
        }
    }
    Ok(tiles)
}

/// Consume a single tile from the input stream.
fn read_tile(input: &mut impl BufRead) -> Res<Tile> {
    lazy_static! {
        static ref ID_RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    }

    let mut lines = input.lines();

    let line = match lines.next() {
        Some(line) => line?,
        None => return Err("EOF when expecting tile".into()),
    };

    let id: u32 = match ID_RE.captures(&line) {
        Some(caps) => caps[1].parse()?,
        None => return Err(format!("Line didn't match tile id regex: {}", line).into()),
    };

    let mut pixels = vec![];
    for line in lines.chain(iter::once(Ok(String::new()))) {
        let line = line?;
        if line == "" {
            return Ok(Tile::new(id, pixels));
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
        for (i, row) in self.pixels.iter().enumerate() {
            let s: String = row.iter().map(|pixel| pixel.to_char()).collect();
            write!(f, "\n{}", s)?;
        }
        Ok(())
    }
}
