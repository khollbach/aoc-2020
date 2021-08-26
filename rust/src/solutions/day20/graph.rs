use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use super::tile::{TileId, Tile};
use super::tile::border::Direction::{self, Up};
use super::tile::border::{DIRS, Border};

/// Representation of the puzzle-graph.
///
/// An edge connects each pair of tiles that have matching borders.
pub struct Graph {
    pub tiles: HashMap<TileId, Tile>,
    pub edges: HashMap<TileId, Vec<TileId>>,
}

impl Graph {
    /// Borders are normalized first, so that it doesn't matter if tiles are flipped face-up or not.
    ///
    /// It's not stated in the problem, but each border should belong to at most 2 tiles.
    /// This panics if that assumption fails.
    pub fn new(tiles: HashMap<TileId, Tile>) -> Graph {
        // Each of t's normalized borders points to t.
        let border_tile_pairs = tiles.values().flat_map(|t| t.borders().map(move |b| (b.normalize(), t.id)));
        let border_to_tiles = pairs_to_hashmap(border_tile_pairs);

        // For each border, insert an edge between the two tiles that share that border.
        let mut edges = HashMap::<_, Vec<_>>::with_capacity(tiles.len());
        for (border, tiles) in border_to_tiles {
            match tiles.len() {
                0 => unreachable!(),
                1 => (), // The "outer edge" of the puzzle.
                2 => {
                    let a = tiles[0];
                    let b = tiles[1];
                    edges.entry(a).or_default().push(b);
                    edges.entry(b).or_default().push(a);
                }
                _ => panic!("More than 2 tiles share this border. {:?} {:?}", border, tiles),
            }
        }

        Graph { tiles, edges }
    }

    /// Flip and/or rotate each tile so that they're all "face up" (or all "face down")
    /// and the borders line up.
    pub fn orient_tiles(&mut self) {
        let n = self.len();

        let mut seen = HashSet::with_capacity(n);
        let mut to_visit = Vec::with_capacity(n);

        // Start anywhere.
        let first = match self.tiles.keys().next() {
            Some(&t) => t,
            None => return,
        };
        seen.insert(first);
        to_visit.push((first, Up, self.tiles[&first].border(Up)));

        // DFS to visit all tiles.
        while let Some((id, dir, border)) = to_visit.pop() {
            // Fix orientation.
            orient_tile_border(self.tiles.get_mut(&id).unwrap(), dir, border);

            for &new_dir in &DIRS {
                if let Some(neighbor) = self.get_neighbor(id, new_dir) {
                    if !seen.contains(&neighbor) {
                        seen.insert(neighbor);
                        to_visit.push((neighbor, new_dir.flip(), self.tiles[&id].border(new_dir)));
                    }
                }
            }
        }

        // The graph should be connected.
        assert_eq!(seen.len(), n, "Graph is disconnected, or there's a bug in our code.");
    }

    /// Get the neighbor of this tile in this direction.
    ///
    /// If the tile is an edge-piece of corner-piece, there may be no such neighbor.
    pub fn get_neighbor(&self, id: TileId, dir: Direction) -> Option<TileId> {
        let border = self.tiles[&id].border(dir);
        let neighbors = &self.edges[&id];
        neighbors.iter().find(|t| self.tiles[t].has_border(border)).copied()
    }

    /// How many tiles are in the graph?
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// The graph should be a square grid, so the dimensions are sqrt(n) by sqrt(n).
    pub fn dimension_hint(&self) -> usize {
        let n = self.len();
        (n as f64).sqrt().ceil() as usize
    }

    /// Tile dimension in pixels, including border.
    pub fn tile_dimension(&self) -> usize {
        self.tiles.values().next().expect("Empty graph.").pixels.len()
    }
}

/// Helper for `Graph::new`.
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

/// Helper for `Graph::orient_tiles`. Flip and/or rotate this tile.
///
/// After orienting, tile's specified direction should equal the specified border.
///
/// Panics if this isn't possible.
fn orient_tile_border(tile: &mut Tile, dir: Direction, border: Border) {
    assert!(tile.has_border(border), "Tile doesn't have border.\n{:?}\n{:?}", tile, border);

    orient_tile(tile, |t| t.border(dir) == border);
}

/// Flip and/or rotate this tile.
///
/// After orienting, the predicate should be satisfied.
///
/// Panics if this is impossible.
pub fn orient_tile(tile: &mut Tile, predicate: impl Fn(&Tile) -> bool) {
    for _ in 0..4 {
        if predicate(tile) { return; }
        tile.rotate_ccw();
    }

    tile.flip();

    for _ in 0..4 {
        if predicate(tile) { return; }
        tile.rotate_ccw();
    }

    // Restore the original orientation before panicking.
    tile.flip();

    panic!("Couldn't satisfy predicate in any orientation. {:?}", tile);
}
