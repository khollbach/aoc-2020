use crate::Res;
use tile::{Tile, TileId};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, prelude::*};
use std::hash::Hash;
use tile::border::Direction::{Left, Right, Up, Down, self};
use crate::solutions::day20::tile::border::{Border, DIRS};
use crate::solutions::day20::tile::Pixel;

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

Part 2:
- flip each tile so that they're all "upright" and the borders line up.
- stitch them together to form an image
- find a sea monster (any orientation)
- use the orientation of the sea monster to re-orient the image
- count the number of non-sea monster tiles

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
    orient_tiles(tiles, graph);

    let image = fuse_image(tiles, graph);

    // count monsters in image

    todo!()
}

/// Flip and/or rotate each tile so that they're all "face up" (or all "face down")
/// and the borders line up.
fn orient_tiles(tiles: &mut HashMap<TileId, Tile>, graph: &Graph) {
    let n = tiles.len();
    assert_eq!(graph.len(), n);

    let mut seen = HashSet::with_capacity(n);
    let mut to_visit = Vec::with_capacity(n);

    let first = match tiles.keys().next() {
        Some(&t) => t,
        None => return,
    };
    seen.insert(first);
    to_visit.push((first, Up, tiles[&first].border(Up)));

    while let Some((id, dir, border)) = to_visit.pop() {
        orient_tile(tiles, id, dir, border);

        let curr_tile = &tiles[&id];
        let neighbors = &graph[&id];

        for &dir in &DIRS {
            let border = curr_tile.border(dir);
            if let Some(&neighbor) = neighbors.iter().find(|&t| tiles[t].has_border(border)) {
                if !seen.contains(&neighbor) {
                    seen.insert(neighbor);
                    to_visit.push((neighbor, dir.flip(), border));
                }
            }
        }
    }

    // The graph must be connected.
    assert_eq!(seen.len(), n, "Graph is disconnected, or there's a bug in our code.");
}

/// Flip and/or rotate this tile.
///
/// After orienting, the tile's border in direction `dir` should equal the target `border`.
///
/// Panics if this is impossible.
fn orient_tile(tiles: &mut HashMap<TileId, Tile>, id: TileId, dir: Direction, border: Border) {
    let tile = tiles.get_mut(&id).unwrap();
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
fn fuse_image(tiles: &HashMap<TileId, Tile>, graph: &Graph) -> Tile {
    let top_left = top_left_corner(tiles, graph);

    // traverse the graph row-by-row, I guess?
    // fill in a matrix of the appropriate size as you go
    /// I'm thinking of moving graph into a class,
    /// with both the tiles and the edges.
    ///
    /// This class can have methods for getting the tile below a tile, etc.
    ///
    /// This let us refactor the common code from orienting, and fusing the image.
    todo!();

    // return it :)
    todo!()
}

/// Helper for `fuse_image`.
///
/// Finds a tile that has exactly two neighbors; one below it, and one to the right of it.
fn top_left_corner(tiles: &HashMap<TileId, Tile>, graph: &Graph) -> TileId {
    for (&id, tile) in tiles {
        let neighbors = &graph[&id];

        if neighbors.len() == 2 {
            let bottom = tile.border(Down);
            let right = tile.border(Right);

            if neighbors.iter().any(|t| tiles[t].has_border(bottom)) &&
                neighbors.iter().any(|t| tiles[t].has_border(right))
            {
                return id;
            }
        }
    }

    panic!("No top-left corner. Perhaps tiles weren't oriented first?");
}
