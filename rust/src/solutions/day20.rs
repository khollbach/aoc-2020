use crate::Res;
use tile::{Tile, TileId};
use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};
use std::hash::Hash;

mod tile;
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
    let mut tiles = input::read_input(io::stdin().lock().lines())?;
    let graph = build_graph(tiles.values());

    println!("{}", part1(&graph));
    println!("{}", part2(&mut tiles, &graph));

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
    let num_tiles_hint = tiles.size_hint().0;

    // Each of t's normalized borders points to t.
    let border_tile_pairs = tiles.flat_map(|t| t.borders().map(move |b| (b.normalize(), t.id)));
    let border_to_tiles = pairs_to_hashmap(border_tile_pairs);

    // In `graph`, each tile sees all other tiles that it shares a border with.
    let mut graph: Graph = HashMap::with_capacity(num_tiles_hint);
    for (border, tiles) in border_to_tiles {
        match tiles.len() {
            0 => unreachable!(),
            1 => (), // The "outer edge" of the puzzle.
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
    // Each corners is adjacent to exactly 2 tiles.
    let mut corners = vec![];
    for (t, edges) in graph {
        let num_adj = edges.len();
        assert!(2 <= num_adj && num_adj <= 4);
        if num_adj == 2 {
            corners.push(t);
        }
    }

    assert_eq!(corners.len(), 4, "{:?}", corners);
    corners.into_iter().map(|t| t.0 as u64).product()
}

fn part2(tiles: &mut HashMap<TileId, Tile>, graph: &Graph) -> usize {
    // orient_tiles(tiles, graph);

    // let image = fuse_image(&tiles);

    // todo: count monsters in image
    todo!()
}

// fn orient_tiles(tiles: &mut HashMap<TileId, Tile>, graph: &Graph) {
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
