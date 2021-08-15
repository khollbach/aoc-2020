use crate::Res;
use input::{Pixel, Tile, TileId};
use std::collections::HashMap;
use std::io;

mod input;

pub fn main() -> Res<()> {
    let input = input::read_input(io::stdin().lock())?;
    build_graph(input);
    Ok(())
}

/*
 * n x n tiles
 * T tiles
 *
 * k=8 borders /tile
 *
 * Suppose: O(k*n) edges in the graph
 *
 * algo:
 * - O(kn) scan tiles, build map: [border -> tiles w. that border]
 * - O(kn*k)? scan map, add clique between members of a bucket.
 */

fn solve_puzzle(tiles: &mut HashMap<TileId, Tile>) -> Vec<Vec<TileId>> {
    // Maps from border to tiles with that border.
    let mut buckets = HashMap::<u32, Vec<TileId>>::new();
    for tile in input {
        for border in tile.possible_borders() {
            buckets.entry(border).or_default().push(tile.id);
        }
    }

    // Our solution relies on borders having a unique match (unless they're on the very edge of the
    // puzzle.)
    for bucket in buckets.values() {
        assert!(bucket.len() <= 2);
    }

    let mut solved = Vec::with_capacity(n);
    for r in 0..n {
        let mut row = Vec::with_capacity(n);
        if r == 0 {
            // Top-left corner: pick an arbitrary corner piece.
            // Also must rotate it s.t. it's edges face up and left.
            place_top_left(tiles);
        } else {
            // Left edge piece.
            // Match with the bottom border of the previous row.
            let above = solved[r-1][0];
            let curr = find_match(buckets[border], above).expect("No matching tile");

            // Rotate to fit: curr's top must match `above`s bot.
            rotate_to_match(?);

            row.push(curr);
        }
        for c in 1..n {
            let prev = row[c-1];

            let border = tiles[prev].right_border();

            // Rotate curr to fit: it's left must match `prev`s right.
            rotate_to_match(?);

            row.push(curr);
        }
        solved.push(row);
    }
    solved
}

fn place_top_left(tiles: &mut [Tile]) -> TileId {
    ?
}

fn rotate_to_match(?) ? {
    ?
}

/// Find a tile in `tiles` other than `exclude`.
fn find_match(tiles: &[TileId], exclude: TileId) -> Option<TileId> {
    debug_assert_eq!(tiles.iter().filter(|t| t != exclude).count(), 1);
    for t in tiles {
        if t != exclude {
            return Some(t);
        }
    }
    None
}

/*

(grid graph:)
map[tile -> tiles]

!! borders map: !!
map[border -> tiles]

ABCDEFGH
IJKLMNOP
QRSTUVWX
YZ...

*/

impl Tile {
    /// May contain duplicates.
    fn possible_borders(&self) -> Vec<u32> {
        let n = self.pixels.len();
        assert_ne!(n, 0);
        assert_eq!(self.pixels[0].len(), n);

        let mut borders = Vec::with_capacity(8);

        let add = |b| borders.push(border_to_u32(b));

        // top
        add((0..n).map(|i| self.pixels[0][i]));
        add((0..n).rev().map(|i| self.pixels[0][i]));

        // bottom
        add((0..n).map(|i| self.pixels[n - 1][i]));
        add((0..n).rev().map(|i| self.pixels[n - 1][i]));

        // left
        add((0..n).map(|i| self.pixels[i][0]));
        add((0..n).rev().map(|i| self.pixels[i][0]));

        // right
        add((0..n).map(|i| self.pixels[i][n - 1]));
        add((0..n).rev().map(|i| self.pixels[i][n - 1]));

        borders
    }
}

fn border_to_u32(pixels: impl Iterator<Item = Pixel>) -> u32 {
    let mut acc = 0;
    for (i, p) in pixels.enumerate() {
        assert!(i < 32);
        acc <<= 1;
        acc |= match p {
            Pixel::Black => 0,
            Pixel::White => 1,
        };
    }
    acc
}
