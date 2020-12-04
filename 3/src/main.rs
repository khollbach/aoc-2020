use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() {
    let grid = read_input(io::stdin().lock()).unwrap();

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}

fn part1(grid: &Grid) -> usize {
    let slope = Slope { right: 3, down: 1 };
    num_collisions(&grid, slope)
}

fn part2(grid: &Grid) -> usize {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    let ans = slopes.into_iter().map(|s| num_collisions(&grid, s));
    ans.product::<usize>()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() {
        let input = BufReader::new(File::open("input").unwrap());
        let grid = read_input(input).unwrap();

        assert_eq!(super::part1(&grid), 234);
    }

    #[test]
    fn part2() {
        let input = BufReader::new(File::open("input").unwrap());
        let grid = read_input(input).unwrap();

        assert_eq!(super::part2(&grid), 5813773056);
    }
}
