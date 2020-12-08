use std::error::Error;
use std::io::{self, prelude::*};

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    let grid = read_input(io::stdin().lock())?;
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
    Ok(())
}

struct Grid {
    rows: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Tree,
}

fn read_input<R: BufRead>(input: R) -> Res<Grid> {
    let mut rows = vec![];
    for line in input.lines() {
        let row: Res<_> = line?.chars().map(Tile::new).collect();
        rows.push(row?);
    }
    Ok(Grid { rows })
}

impl Tile {
    fn new(c: char) -> Res<Tile> {
        match c {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Tree),
            _ => Err(format!("Invalid tile character: {}", c).into()),
        }
    }
}

fn part1(grid: &Grid) -> usize {
    let slope = Slope { right: 3, down: 1 };
    num_collisions(grid, slope)
}

fn part2(grid: &Grid) -> usize {
    let slopes = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    let ans = slopes.iter().map(|&s| num_collisions(grid, s));
    ans.product()
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

    let height = grid.rows.len();
    while y < height {
        let width = grid.rows[y].len();
        assert_ne!(width, 0);

        if grid.rows[y][x % width] == Tile::Tree {
            collisions += 1;
        }

        y += slope.down;
        x += slope.right;
    }

    collisions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() -> Res<()> {
        let input = BufReader::new(File::open("../../inputs/3")?);
        let grid = read_input(input)?;
        assert_eq!(super::part1(&grid), 234);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("../../inputs/3")?);
        let grid = read_input(input)?;
        assert_eq!(super::part2(&grid), 5813773056);
        Ok(())
    }
}
