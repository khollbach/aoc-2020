use super::Res;
use point::{Point, Point3, Point4};
use std::collections::HashSet;
use std::io::{self, prelude::*};

mod point;

pub fn main() -> Res<()> {
    let (state3, state4) = read_input(io::stdin().lock())?;
    println!("{}", state3.simulate(6).num_active());
    println!("{}", state4.simulate(6).num_active());
    Ok(())
}

#[derive(Debug, Clone)]
struct State<P: Point> {
    active: HashSet<P>,
}

impl<P: Point> State<P> {
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
            Some(p).into_iter().chain(adj)
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

fn read_input(input: impl BufRead) -> Res<(State<Point3>, State<Point4>)> {
    let mut active3 = HashSet::new();
    let mut active4 = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line?.chars().enumerate() {
            let x = x as i32;
            match c {
                '#' => {
                    active3.insert(Point3::new(x, y));
                    active4.insert(Point4::new(x, y));
                }
                '.' => (),
                _ => return Err(format!("Invalid char: {}", c).into()),
            }
        }
    }
    Ok((State { active: active3 }, State { active: active4 }))
}
