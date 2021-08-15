use crate::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::iter;
use super::tile::{Pixel, TileId, Tile};

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
