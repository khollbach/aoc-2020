use crate::Res;
use tile::{Tile, TileId};
use std::collections::HashSet;
use std::io::{self, prelude::*};
use tile::border::Direction::{Right, Up, Down, self};
use tile::border::{Border, DIRS};
use graph::Graph;
use input::read_input;

mod tile;
mod input;
mod graph;

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

Part 2:
- flip each tile so that they're all "upright" and the borders line up.
- stitch them together to form an image
- find a sea monster (any orientation)
- use the orientation of the sea monster to re-orient the image
- count the number of non-sea monster tiles

 */

pub fn main() -> Res<()> {
    let tiles = read_input(io::stdin().lock().lines())?;
    let mut graph = Graph::new(tiles);

    println!("{}", part1(&graph));
    println!("{}", part2(&mut graph));

    Ok(())
}

/// Return the product of the ids of the four corners.
///
/// Panics if our assumptions about the puzzle don't hold.
fn part1(graph: &Graph) -> u64 {
    let mut corners = Vec::with_capacity(4);

    for (&t, neighbors) in &graph.edges {
        let num_adj = neighbors.len();
        assert!(2 <= num_adj && num_adj <= 4);

        // Each corner is adjacent to exactly 2 tiles.
        if num_adj == 2 {
            corners.push(t);
        }
    }

    assert_eq!(corners.len(), 4, "{:?}", corners);

    corners.into_iter().map(|t| t.0 as u64).product()
}

fn part2(graph: &mut Graph) -> usize {
    orient_tiles(graph);

    let image = fuse_image(graph);

    // count monsters in image

    todo!()
}

/// Flip and/or rotate each tile so that they're all "face up" (or all "face down")
/// and the borders line up.
fn orient_tiles(graph: &mut Graph) {
    let mut seen = HashSet::with_capacity(graph.len());
    let mut to_visit = Vec::with_capacity(graph.len());

    // Start anywhere.
    let first = match graph.tiles.keys().next() {
        Some(&t) => t,
        None => return,
    };
    seen.insert(first);
    to_visit.push((first, Up, graph.tiles[&first].border(Up)));

    // DFS to visit all tiles.
    while let Some((id, dir, border)) = to_visit.pop() {
        // Fix orientation.
        orient_tile(graph.tiles.get_mut(&id).unwrap(), dir, border);

        for &new_dir in &DIRS {
            if let Some(neighbor) = graph.get_neighbor(id, new_dir) {
                if !seen.contains(&neighbor) {
                    seen.insert(neighbor);
                    to_visit.push((neighbor, new_dir.flip(), graph.tiles[&id].border(new_dir)));
                }
            }
        }
    }

    // The graph should be connected.
    assert_eq!(seen.len(), graph.len(), "Graph is disconnected, or there's a bug in our code.");
}

/// Flip and/or rotate this tile.
///
/// After orienting, the tile's border in direction `dir` should equal the target `border`.
///
/// Panics if this is impossible.
fn orient_tile(tile: &mut Tile, dir: Direction, border: Border) {
    assert!(tile.has_border(border), "Tile doesn't have border.\n{:?}\n{:?}", tile, border);

    for _ in 0..4 {
        if tile.border(dir) == border { return; }
        tile.rotate_ccw();
    }

    tile.flip();

    for _ in 0..4 {
        if tile.border(dir) == border { return; }
        tile.rotate_ccw();
    }

    panic!(
        "Tile doesn't have border. Possibly a bug in one of the helper functions.\n\
        {:?}\n{:?}",
        tile, border
    );
}

/// Fuse the image into one big virtual "tile", by stripping borders and gluing tiles together.
///
/// Warning: don't treat the output like a normal tile. E.g., it doesn't have an id in the `tiles` collection,
/// it's the wrong size, etc.
fn fuse_image(graph: &Graph) -> Tile {
    let top_left = top_left_corner(graph);

    // traverse the graph row-by-row, I guess?
    // fill in a matrix of the appropriate size as you go
    todo!();
}

/// Helper for `fuse_image`.
///
/// Finds a tile that has exactly two neighbors; one below it, and one to the right of it.
fn top_left_corner(graph: &Graph) -> TileId {
    let mut ret = None;

    for (&id, neighbors) in &graph.edges {
        if neighbors.len() == 2 &&
            graph.get_neighbor(id, Down).is_some() &&
            graph.get_neighbor(id, Right).is_some()
        {
            assert!(ret.is_none(), "Two top-left corners. (Perhaps tiles weren't oriented first?) {:?} {:?}", ret, id);
            ret = Some(id);
        }
    }

    ret.expect("No top-left corner. (Perhaps tiles weren't oriented first?)")
}
