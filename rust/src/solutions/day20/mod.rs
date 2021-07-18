use crate::Res;
use input::{Pixel::{Black, White, self}, Tile, TileId};
use std::collections::HashMap;
use std::io::{self, prelude::*};
use Direction::{Up, Left, Right, Down};
use std::fmt;
use std::hash::Hash;
use std::cmp::max;

mod input;

/*

12 by 12 grid of tiles
each tile is 10 by 10 pixels (the inner 8x8 is the actual image).

Part 1:
- build collection of "edges":
    each is either a pair of tiles, or a single tile
    keyed by normalized border
- loop over tiles,
    - count degrees
    - spit out the 4 tiles with degree 2

 */

pub fn main() -> Res<()> {
    let tiles = input::read_input(io::stdin().lock().lines())?;
    let graph = build_graph(tiles.values());

    part1(&graph);

    Ok(())
}

type Graph = HashMap<TileId, Vec<TileId>>;

/// An edge connects each pair of tiles that have matching borders.
///
/// Borders are normalized first, so that it doesn't matter if tiles are flipped face-up or not.
///
/// It's not stated in the problem, but each border should belong to at most 2 tiles.
/// This panics if that assumption fails.
fn build_graph<'a>(tiles: impl Iterator<Item = &'a Tile>) -> Graph {
    // Each of t's normalized borders points to t.
    let border_tile_pairs = tiles.flat_map(|t| t.borders().map(move |b| (b.normalize(), t.id)));
    let border_to_tiles = pairs_to_hashmap(border_tile_pairs);

    let mut graph: Graph = HashMap::new();
    for (border, tiles) in border_to_tiles {
        match tiles.len() {
            0 => unreachable!(),
            1 => (), // Outside edge of the puzzle; just ignore this border.
            2 => {
                let a = tiles[0];
                let b = tiles[1];
                graph.entry(a).or_default().push(b);
                graph.entry(b).or_default().push(a);
            }
            _ => panic!("More than 2 tiles share this border. {:?} {:?}", border, tiles),
        }

    }
    graph
}

fn pairs_to_hashmap<A, B>(pairs: impl Iterator<Item=(A, B)>) -> HashMap<A, Vec<B>>
where
    A: Eq + Hash
{
    let mut map: HashMap<A, Vec<B>> = HashMap::with_capacity(pairs.size_hint().0);
    for (a, b) in pairs {
        map.entry(a).or_default().push(b);
    }
    map
}

/// Return the product of the ids of the four corners.
///
/// Panics if our assumptions about the puzzle don't hold.
fn part1(graph: &HashMap<TileId, Vec<TileId>>) -> u64 {
    // todo: wtf is going on here?
    let mut freqs = HashMap::new();
    for (t, edges) in graph {
        let f = edges.len();
        *freqs.entry(f).or_insert(0) += 1;
    }
    dbg!(freqs, graph.len());
    todo!("find and fix the bug");

    // Each corners is adjacent to exactly 2 tiles.
    let mut corners = vec![];
    for (t, edges) in graph {
        if edges.len() == 2 {
            corners.push(t);
        }
    }

    assert_eq!(corners.len(), 4, "{:?}", corners);
    corners.into_iter().map(|t| t.0 as u64).product()
}

// todo: it would be nice to better understand the use of the 'move' keyword in this file.

impl Tile {
    /// This tile's 4 borders, in any order.
    fn borders<'a>(&'a self) -> impl Iterator<Item=Border> + 'a {
        DIRS.iter().map(move |&d| self.border(d))
    }

    /// Each border is always read from left-to-right, or top-to-bottom.
    ///
    /// This is important, so that we can check equality of two side-by-side borders.
    /// E.g. the right border of a tile versus the left border of the tile next to it.
    fn border(&self, dir: Direction) -> Border {
        let n = self.pixels.len();
        match dir {
            Up => Border::new((0..n).map(|i| self.pixels[0][i])),
            Down => Border::new((0..n).map(|i| self.pixels[n - 1][i])),
            Left => Border::new((0..n).map(|i| self.pixels[i][0])),
            Right => Border::new((0..n).map(|i| self.pixels[i][n - 1])),
        }
    }
}

/// Keep these in counter-clockwise order; some methods rely on it.
/// todo: is this really nec.?
#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

const DIRS: [Direction; 4] = [Up, Right, Down, Left];

/// A bitmask representing the pattern of pixels on a border.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Border(u32);

impl Border {
    const NUM_PIXELS: usize = 32;

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

    fn normalize(self) -> Border {
        max(self, self.flip())
    }
}

// todo test this
impl fmt::Debug for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.0)
    }
}

// fn part2(tiles: &mut HashMap<TileId, Tile>) -> ? {
//     let edges = compute_edges(tiles);
//     orient_tiles(tiles, &edges);
//     let image = fuse_image(&tiles); todo: implement
//     todo: count monsters in image
// }

// fn orient_tiles(tiles: &HashMap<TileId, Tile>, edges: &Edges) {
//     assert!(!tiles.is_empty());
//
//     let mut to_visit = vec![];
//     let mut seen = HashSet::new();
//
//     let first = tiles.keys().next().unwrap();
//     to_visit.push((first, Top, first.border(Top)));
//     seen.insert(first);
//
//     while let Some((tile, dir, border)) = to_visit.pop() {
//         //correct_orientation(tiles, tile, dir, border); todo!
//
//         for &dir in &DIRS {
//             let border = tile.border(dir);
//             let other = todo!(); // find_match(edges, tile, border);
//             if !seen.contains(&other) {
//                 to_visit.push((other, dir.opposite(), border));
//             }
//         }
//     }
//
//     assert_eq!(seen.len(), tiles.len());
// }

// /// Flip tile so that its `dir` border equals `border`.
// fn correct_orientation(tiles, tile_id, dir, border) {
//     let tile = tiles[tile_id];
//
//     if !tile.borders().contains(&border) {
//         tile.flip();
//     }
//     assert!(tile.borders().contains(&border));
//
//     let curr_dir = tile.borders().index_of(border).unwrap();
//     let rotation = dir.difference(curr_dir);
//     tile.rotate(rotation);
// }

// /// Counter-clockwise.
// #[derive(Clone, Copy)]
// enum Rotation {
//     R0,
//     R90,
//     R180,
//     R270,
// }
//
// impl Direction {
//     fn opposite(self) -> Self {
//         match self {
//             Top => Bottom,
//             Bottom => Top,
//             Right => Left,
//             Left => Right,
//         }
//     }
//
//     fn difference(self, other: Self) -> Rotation {
//         Rotation::turns_ccw((self as u32 - other as u32) % 4)
//     }
// }
//
// impl Rotation {
//     fn turns_ccw(num_turns: u32) -> Rotation {
//         assert!(num_turns < 4);
//         match num_turns {
//             0 => R0,
//             1 => R90,
//             2 => R180,
//             3 => R270,
//             _ => unreachable!(),
//         }
//     }
// }
//
// impl Tile {
//     fn rotate(&mut self, dir: Direction) {
//         todo!()
//     }
//
//     fn flip(&mut self) {
//         todo!()
//     }
// }
//
