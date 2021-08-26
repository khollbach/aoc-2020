use crate::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::iter;
use super::tile::{Pixel, TileId, Tile};

/// Read the input into a collection of Tiles keyed by TileId.
///
/// All tiles will be square and have the same size.
pub fn read_input(mut lines: impl Iterator<Item = io::Result<String>>) -> Res<HashMap<TileId, Tile>> {
    let mut tiles = HashMap::new();
    let mut tile_size = None;

    while let Some(tile) = read_tile(&mut lines)? {
        // Check the tiles are all the same size.
        let curr_size = tile.pixels.len();
        match tile_size {
            Some(size) => if curr_size != size {
                return Err(format!("Different tile sizes: {} {} {:?}", size, curr_size, tile).into());
            }
            None => if curr_size < 3 {
                // There should at least be a border and a single inner pixel.
                return Err(format!("Tile size empty or unreasonably small: {}", curr_size).into());
            } else {
                tile_size = Some(curr_size);
            }
        }

        let ret = tiles.insert(tile.id, tile);
        if let Some(other) = ret {
            return Err(format!("Repeated tile id: {:?}", other.id).into());
        }
    }

    Ok(tiles)
}

/// Consume a single tile (and the following blank line "separator") from the input stream.
///
/// Returns Ok(None) on end-of-stream.
fn read_tile(lines: &mut impl Iterator<Item = io::Result<String>>) -> Res<Option<Tile>> {
    lazy_static! {
        static ref ID_REGEX: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    }

    let first_line = match lines.next() {
        Some(line) => line?,
        None => return Ok(None),
    };

    let id: u32 = match ID_REGEX.captures(&first_line) {
        Some(caps) => caps[1].parse()?,
        None => return Err(format!("Line didn't match tile id regex: {:?}", first_line).into()),
    };

    let mut pixels = vec![];

    for line in lines.chain(iter::once(Ok(String::new()))) {
        let line = line?;

        // A blank line signals the end of the current tile.
        if line == "" {
            let tile = Tile::new(id, pixels);

            let n = tile.pixels.len();
            if n > 0 && tile.pixels.iter().all(|row| row.len() == n) {
                return Ok(Some(tile));
            } else {
                return Err(format!("Jagged or non-square tile: {:?}", tile).into());
            }
        }

        let row: Res<Vec<Pixel>> = line.chars().map(Pixel::new).collect();
        pixels.push(row?);
    }

    unreachable!()
}
