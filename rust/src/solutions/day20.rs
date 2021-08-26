use crate::Res;
use std::io::{self, prelude::*};
use input::read_input;
use tile::{Tile, Pixel};
use graph::{Graph, orient_tile};
use image::{fuse_image, delete_sea_monsters};

mod input;
mod tile;
mod graph;
mod image;

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
    // Fuse the solved puzzle into a single image.
    graph.orient_tiles();
    let mut image = fuse_image(graph);

    // Find the correct orientation of the entire image.
    fn is_correct_orientation(image: &Tile) -> bool {
        delete_sea_monsters(image).pixels != image.pixels
    }
    orient_tile(&mut image, is_correct_orientation);

    // Delete sea monsters and count water tiles.
    let image = delete_sea_monsters(&image);
    let num_water_tiles = image.pixels.iter().map(|row| {
        row.iter().filter(|&&p| p == Pixel::White).count()
    }).sum();
    num_water_tiles
}
