use super::graph::Graph;
use super::tile::{Tile, Pixel, TileId};
use super::tile::border::Direction::{Down, Right};

/// Fuse the image into one big virtual "tile", by stripping borders and gluing tiles together.
///
/// WARNING: the tiles in the graph must have been oriented first.
///
/// Warning: don't treat the output like a normal tile. E.g., it doesn't have an id in the `tiles` collection,
/// it's the wrong size, etc.
pub fn fuse_image(graph: &Graph) -> Tile {
    let grid = grid_of_tiles(graph);

    let grid_dim = grid.len();
    assert_eq!(grid_dim, 12);
    let tile_dim_incl_border = graph.tile_dimension();
    assert_eq!(tile_dim_incl_border, 10);

    let tile_dim = tile_dim_incl_border - 2;
    let image_dim = grid_dim * tile_dim;

    let mut image = vec![vec![Pixel::Black; image_dim]; image_dim];

    for i in 0..grid_dim {
        for j in 0..grid_dim {
            let tile = &graph.tiles[&grid[i][j]];

            for k in 0..tile_dim {
                for l in 0..tile_dim {
                    let row = i * tile_dim + k;
                    let col = j * tile_dim + l;
                    image[row][col] = tile.pixels[k + 1][l + 1]; // Ignore tile's border.
                }
            }
        }
    }

    Tile { id: TileId(0xDEADBEEF), pixels: image }
}

/// Helper for `fuse_image`.
///
/// Fill in a matrix with the grid structure of the graph.
///
/// WARNING: the tiles in the graph must have been oriented first.
fn grid_of_tiles(graph: &Graph) -> Vec<Vec<TileId>> {
    let dim = graph.dimension_hint();
    assert_ne!(dim, 0, "Empty graph.");

    // The current tile.
    let mut curr = top_left_corner(graph);

    let mut grid = Vec::with_capacity(dim);
    for i in 0.. {
        let mut row = Vec::with_capacity(dim);
        loop {
            row.push(curr);

            // Next column: go right.
            curr = match graph.get_neighbor(curr, Right) {
                Some(next) => next,
                None => break,
            };
        }
        assert_eq!(row.len(), dim);
        grid.push(row);

        // Next row: go down from the left-most tile in the current row.
        curr = match graph.get_neighbor(grid[i][0], Down) {
            Some(next) => next,
            None => break,
        };
    }
    assert_eq!(grid.len(), dim);

    grid
}

/// Helper for `grid_of_tiles`.
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

/// Return a clone of `image`, with all sea monsters deleted.
///
/// Note that if `image` isn't oriented correctly, this is likely to be a no-op.
///
/// Based on how the instructions for Day 20 are phrased, we should be able to detect
/// the correct orientation simply by re-trying this function for every possible orientation
/// of the image. Once we find an orientation where this function actually has an effect, we
/// can assume it's the correct orientation.
pub fn delete_sea_monsters(image: &Tile) -> Tile {
    let mut output = image.clone();

    for i in 0..image.pixels.len() - SEA_MONSTER_HEIGHT {
        for j in 0..image.pixels[i].len() - SEA_MONSTER_WIDTH {
            if sea_monster_at(image, i, j) {
                delete_sea_monster(&mut output, i, j);
            }
        }
    }

    output
}

/// Helper for `delete_sea_monsters`.
fn sea_monster_at(image: &Tile, i: usize, j: usize) -> bool {
    SEA_MONSTER_POINTS.iter().all(|&(di, dj)| {
        image.pixels[i + di][j + dj] == Pixel::White
    })
}

/// Helper for `delete_sea_monsters`.
fn delete_sea_monster(image: &mut Tile, i: usize, j: usize) {
    for &(di, dj) in &SEA_MONSTER_POINTS {
        image.pixels[i + di][j + dj] = Pixel::Black;
    }
}

/*
This is what a sea monster looks like:
                  #
#    ##    ##    ###
 #  #  #  #  #  #
01234567890123456789
 */
const SEA_MONSTER_POINTS: [(usize, usize); 15] = [
    // Tail
    (1, 0),
    (2, 1),
    // Loop 1
    (2, 4),
    (1, 5),
    (1, 6),
    (2, 7),
    // Loop 2
    (2, 10),
    (1, 11),
    (1, 12),
    (2, 13),
    // Head
    (2, 16),
    (1, 17),
    (1, 18),
    (0, 18),
    (1, 19),
];
const SEA_MONSTER_HEIGHT: usize = 3;
const SEA_MONSTER_WIDTH: usize = 20;
