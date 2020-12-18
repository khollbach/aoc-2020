use super::Res;
use point::{Point, Point3, Point4};
use std::collections::HashSet;
use std::io::{self, prelude::*};
use std::iter;

mod point;

pub fn main() -> Res<()> {
    let coords = read_input(io::stdin().lock())?;
    println!("{}", State::<Point3>::new(&coords).simulate(6).num_active());
    println!("{}", State::<Point4>::new(&coords).simulate(6).num_active());
    Ok(())
}

fn read_input(input: impl BufRead) -> Res<Vec<(i32, i32)>> {
    let mut coords = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            match c {
                '#' => coords.push((x as i32, y as i32)),
                '.' => (),
                _ => return Err(format!("Invalid char: {}", c).into()),
            }
        }
    }
    Ok(coords)
}

#[derive(Debug)]
struct State<P: Point> {
    active: HashSet<P>,
}

impl<P: Point> State<P> {
    fn new(coords: &[(i32, i32)]) -> Self {
        State {
            active: coords.iter().map(|&(x, y)| P::new(x, y)).collect(),
        }
    }

    fn simulate(self, num_rounds: usize) -> Self {
        let mut state = self;
        for _ in 0..num_rounds {
            state = state.evolve();
        }
        state
    }

    fn evolve(&self) -> Self {
        // Anything currently active, or next to something active.
        let relevant_points = self.active.iter().flat_map(|&p| {
            let adj = p.adj_points().into_iter();
            iter::once(p).chain(adj)
        });

        let active: HashSet<_> = relevant_points
            .filter(|&p| self.is_active_next_round(p))
            .collect();
        State { active }
    }

    fn is_active_next_round(&self, p: P) -> bool {
        match (self.active.contains(&p), self.num_active_neighbours(p)) {
            (true, 2..=3) => true,
            (false, 3) => true,
            _ => false,
        }
    }

    fn num_active_neighbours(&self, p: P) -> usize {
        p.adj_points()
            .into_iter()
            .filter(|neighbour| self.active.contains(neighbour))
            .count()
    }

    fn num_active(&self) -> usize {
        self.active.len()
    }
}
