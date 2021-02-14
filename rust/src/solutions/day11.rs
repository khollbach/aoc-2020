use crate::Res;
use std::io::{self, prelude::*};
use std::mem;

pub fn main() -> Res<()> {
    let initial_state = State::from_input(io::stdin().lock())?;
    println!("{}", final_num_ppl(initial_state.clone(), false));
    println!("{}", final_num_ppl(initial_state, true));
    Ok(())
}

fn final_num_ppl(initial_state: State, part2: bool) -> usize {
    let mut state = initial_state;
    let mut next_state = state.clone();
    loop {
        // Compute next state.
        state.evolve(&mut next_state, part2);

        if next_state == state {
            return state.total_num_ppl();
        }

        // Update state.
        mem::swap(&mut state, &mut next_state);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Chair,
    Person,
    Floor,
}

impl Tile {
    fn new(c: char) -> Res<Tile> {
        match c {
            '#' => Ok(Tile::Person),
            'L' => Ok(Tile::Chair),
            '.' => Ok(Tile::Floor),
            _ => Err(format!("Invalid tile character: {}", c).into()),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    grid: Vec<Vec<Tile>>,
}

impl State {
    fn from_input(input: impl BufRead) -> Res<State> {
        let mut grid: Vec<Vec<_>> = vec![];
        for (i, line) in input.lines().enumerate() {
            let row: Res<_> = line?.chars().map(Tile::new).collect();
            let row: Vec<_> = row?;
            if i != 0 && row.len() != grid[0].len() {
                return Err(format!(
                    "Jagged grid: row lengths {} and {}",
                    grid[0].len(),
                    row.len()
                )
                .into());
            }
            grid.push(row);
        }
        Ok(State { grid })
    }

    fn dimensions(&self) -> (usize, usize) {
        let n = self.grid.len();
        let m = if n == 0 { 0 } else { self.grid[0].len() };
        (n, m)
    }

    fn evolve(&self, output_buf: &mut State, part2: bool) {
        let (n, m) = self.dimensions();
        assert_eq!(output_buf.dimensions(), (n, m));

        for i in 0..n {
            for j in 0..m {
                let tile = self.grid[i][j];
                let nearby_ppl = if part2 {
                    self.visible_people(i, j)
                } else {
                    self.adj_people(i, j)
                };
                let thresh = if part2 { 5 } else { 4 };

                let new_tile = match (tile, nearby_ppl) {
                    (Tile::Chair, 0) => Tile::Person,
                    (Tile::Person, n) if n >= thresh => Tile::Chair,
                    _ => tile,
                };
                output_buf.grid[i][j] = new_tile;
            }
        }
    }

    fn adj_people(&self, i: usize, j: usize) -> u8 {
        self.vis_ppl_helper(i, j, 1)
    }

    fn visible_people(&self, i: usize, j: usize) -> u8 {
        self.vis_ppl_helper(i, j, usize::MAX)
    }

    fn vis_ppl_helper(&self, i: usize, j: usize, dist_limit: usize) -> u8 {
        let (n, m) = self.dimensions();
        assert!(i < n && j < m);

        // So we can do signed arithmetic.
        let i = i as isize;
        let j = j as isize;
        let n = n as isize;
        let m = m as isize;

        // Returns the first visible tile in a direction.
        // Visibility is limited by `dist_limit`.
        let line_of_sight = |i, j, di, dj| -> Option<Tile> {
            let mut r = i + di;
            let mut c = j + dj;
            let mut dist = 1;
            while dist <= dist_limit && (0 <= r && r < n) && (0 <= c && c < m) {
                let tile = self.grid[r as usize][c as usize];
                if tile != Tile::Floor {
                    return Some(tile);
                }
                r += di;
                c += dj;
                dist += 1;
            }
            None
        };

        let mut count = 0;
        for &di in &[-1, 0, 1] {
            for &dj in &[-1, 0, 1] {
                if (di, dj) != (0, 0) && line_of_sight(i, j, di, dj) == Some(Tile::Person) {
                    count += 1;
                }
            }
        }
        count
    }

    fn total_num_ppl(&self) -> usize {
        fn row_count(row: &[Tile]) -> usize {
            row.iter().filter(|&&t| t == Tile::Person).count()
        }
        self.grid.iter().map(|r| row_count(r)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/11")?);
        let initial_state = State::from_input(input)?;
        assert_eq!(final_num_ppl(initial_state, false), 2319);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/11")?);
        let initial_state = State::from_input(input)?;
        assert_eq!(final_num_ppl(initial_state, true), 2117);
        Ok(())
    }
}
