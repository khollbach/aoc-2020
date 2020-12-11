use super::Res;
use std::io::{self, prelude::*};
use std::mem;

pub fn main() -> Res<()> {
    let initial_state = State::from_input(io::stdin().lock())?;

    println!("{}", part1(initial_state));

    Ok(())
}

fn part1(initial_state: State) -> usize {
    let mut state = initial_state;
    let mut buf = state.clone();
    loop {
        // Compute next state.
        state.evolve(&mut buf);

        if buf == state {
            return state.num_occupied();
        }

        // Update state.
        mem::swap(&mut state, &mut buf);
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
                    "Non-rectangular grid: row lengths {} and {}",
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

    fn evolve(&self, output_buf: &mut State) {
        let (n, m) = self.dimensions();
        assert_eq!(output_buf.dimensions(), (n, m));

        for i in 0..n {
            for j in 0..m {
                let tile = self.grid[i][j];
                let num_adj = self.adj_people(i, j);
                let new_tile = match (tile, num_adj) {
                    (Tile::Chair, 0) => Tile::Person,
                    (Tile::Person, n) if n >= 4 => Tile::Chair,
                    _ => tile,
                };
                output_buf.grid[i][j] = new_tile;
            }
        }
    }

    fn adj_people(&self, i: usize, j: usize) -> u8 {
        let (n, m) = self.dimensions();
        assert!(i < n && j < m);

        // So we can do signed arithmetic.
        let i = i as isize;
        let j = j as isize;
        let n = n as isize;
        let m = m as isize;

        let mut count = 0;
        for &di in &[-1, 0, 1] {
            for &dj in &[-1, 0, 1] {
                let r = i + di;
                let c = j + dj;
                if (r, c) != (i, j)
                    && (0 <= r && r < n)
                    && (0 <= c && c < m)
                    && self.grid[r as usize][c as usize] == Tile::Person
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn num_occupied(&self) -> usize {
        fn row_count(row: &[Tile]) -> usize {
            row.iter().filter(|&&t| t == Tile::Person).count()
        }
        self.grid.iter().map(|r| row_count(r)).sum()
    }
}
