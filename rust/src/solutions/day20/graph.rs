use std::collections::HashMap;
use std::hash::Hash;
use super::tile::{TileId, Tile};
use super::tile::border::Direction;

pub struct Graph {
    pub tiles: HashMap<TileId, Tile>,
    pub edges: HashMap<TileId, Vec<TileId>>,
}

impl Graph {
    /// An edge connects each pair of tiles that have matching borders.
    ///
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

    /// How many tiles are in the graph?
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Get the neighbor of this tile in this direction.
    ///
    /// If the tile is an edge-piece of corner-piece, there may be no such neighbor.
    pub fn get_neighbor(&self, id: TileId, dir: Direction) -> Option<TileId> {
        let border = self.tiles[&id].border(dir);
        let neighbors = &self.edges[&id];
        neighbors.iter().find(|t| self.tiles[t].has_border(border)).copied()
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
