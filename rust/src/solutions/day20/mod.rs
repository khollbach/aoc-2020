use crate::Res;
use input::{Pixel, Tile, TileId};
use std::collections::{HashMap as Map, HashSet as Set};
use std::io;

mod input;

pub fn main() -> Res<()> {
    let tiles = input::read_input(io::stdin().lock())?;
    //build_graph(input);
    Ok(())
}

fn part1(tiles: &Map<TileId, Tile>) -> u64 {
    let edges = compute_edges(&mut tiles);
    let corners = find_corners(&edges);
    assert_eq!(corners.len(), 4);
    let mut product = 1;
    for TileId(id) in corners {
        corners *= id as u64;
    }
    product
}

fn part2(tiles: &mut Map<TileId, Tile>) -> ? {
    let edges = compute_edges(tiles);
    orient_tiles(tiles, &edges);
    let image = fuse_image(&tiles); todo: implement
    todo: count monsters in image
}

type Edges = Map<Border, Vec<TileId>>;

fn compute_edges(tiles: &mut Map<TileId, Tile>) -> Edges {
    let mut edges = Map::new();

    for (&tile_id, tile) in tiles {
        for b in tile.borders() {
            edges.entry(b).or_default().push(tile_id);
        }
        tile.flip();
        for b in tile.borders() {
            edges.entry(b).or_default().push(tile_id);
        }
    }

    let borders: Vec<_> = edges.keys().copied().collect();
    for b in borders {
        let tiles = edges[b];

        // We assume borders are uniquely matched.
        assert!(tiles.len() <= 2);

        // Remove corners and edges of the picture.
        if tiles.len() == 1 {
            edges.remove(b);
        }
    }

    edges
}

fn orient_tiles(tiles: &Map<TileId, Tile>, edges: &Edges) {
    assert!(!tiles.is_empty());

    let mut to_visit = vec![];
    let mut seen = Set::new();

    let first = tiles.keys().next().unwrap();
    to_visit.push((first, Top, first.border(Top)));
    seen.insert(first);

    while let Some((tile, dir, border)) = to_visit.pop() {
        correct_orientation(tiles, tile, dir, border);

        for &dir in &DIRS {
            let border = tile.border(dir);
            let other = find_match(edges, tile, border);
            if !seen.contains(&other) {
                to_visit.push((other, dir.opposite(), border));
            }
        }
    }

    assert_eq!(seen.len(), tiles.len());
}

/// Flip tile so that its `dir` border equals `border`.
fn correct_orientation(tiles, tile_id, dir, border) {
    let tile = tiles[tile_id];

    if !tile.borders().contains(&border) {
        tile.flip();
    }
    assert!(tile.borders().contains(&border));

    let curr_dir = tile.borders().index_of(border).unwrap();
    let rotation = dir.difference(curr_dir);
    tile.rotate(rotation);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Border(u32);

/// Keep these in counter-clockwise order; some methods rely on it.
#[derive(Clone, Copy)]
enum Direction {
    Right,
    Top,
    Left,
    Bottom,
}

const DIRS: [Direction; 4] = [Top, Right, Bottom, Left];

/// Counter-clockwise.
#[derive(Clone, Copy)]
enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Top => Bottom,
            Bottom => Top,
            Right => Left,
            Left => Right,
        }
    }

    fn difference(self, other: Self) -> Rotation {
        Rotation::turns_ccw((self as u32 - other as u32) % 4)
    }
}

impl Rotation {
    fn turns_ccw(num_turns: u32) -> Rotation {
        assert!(num_turns < 4);
        match num_turns {
            0 => R0,
            1 => R90,
            2 => R180,
            3 => R270,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn rotate(&mut self, dir: Direction) {
        todo!()
    }

    fn flip(&mut self) {
        todo!()
    }

    /// 4 borders, in any order.
    fn borders(&self) -> Vec<Border> {
        DIRS.iter().map(|&d| self.border(d)).collect()
    }

    /// Always read right-to-left, or top-to-bottom.
    /// This is important, so that we can check equality of two side-by-side borders.
    fn border(&self, dir: Direction) -> Border {
        let n = self.pixels.len();
        match dir {
            Top => Border::new((0..n).map(|i| self.pixels[0][i])),
            Bottom => Border::new((0..n).map(|i| self.pixels[n-1][i])),
            Left => Border::new((0..n).map(|i| self.pixels[i][0])),
            Right => Border::new((0..n).map(|i| self.pixels[i][n-1])),
        }
    }
}

impl Border {
    fn new(pixels: Iterator<Item=Pixel>) -> Border {
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
}
