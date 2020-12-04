use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() {
    let grid = read_input(io::stdin().lock()).unwrap();

    let slope = Slope { down: 1, right: 3 };
    println!("{}", num_collisions(&grid, slope));
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Tree,
}

fn read_input<R: BufRead>(input: R) -> Result<Grid, Box<dyn Error>> {
    let mut grid = vec![];

    for line in input.lines() {
        let mut row = vec![];

        for c in line?.chars() {
            let tile = match c {
                '.' => Tile::Empty,
                '#' => Tile::Tree,
                _ => return Err(format!("Invalid tile character: {}", c).into()),
            };

            row.push(tile);
        }

        grid.push(row);
    }

    Ok(grid)
}

/// This should be a positive rational number, in simplest terms.
#[derive(Debug, Clone, Copy)]
struct Slope {
    down: usize,
    right: usize,
}

fn num_collisions(grid: &Grid, slope: Slope) -> usize {
    assert_ne!(slope.down, 0);

    let mut collisions = 0;

    let mut y = 0;
    let mut x = 0;

    let height = grid.len();
    while y < height {
        let width = grid[y].len();
        assert_ne!(width, 0);

        if grid[y][x % width] == Tile::Tree {
            collisions += 1;
        }

        y += slope.down;
        x += slope.right;
    }

    collisions
}
