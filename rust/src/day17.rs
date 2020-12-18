use super::Res;
use std::collections::HashSet;
use std::io::{self, prelude::*};
use std::ops::Add;

pub fn main() -> Res<()> {
    let initial_state = State::from_input(io::stdin().lock())?;
    println!("{}", initial_state.simulate(6).num_active());
    Ok(())
}

#[derive(Debug, Clone)]
struct State {
    active: HashSet<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl State {
    fn from_input(input: impl BufRead) -> Res<State> {
        let mut active = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            let y = y as i32;
            for (x, c) in line?.chars().enumerate() {
                let x = x as i32;
                match c {
                    '#' => {
                        active.insert(Point { x, y, z: 0 });
                    }
                    '.' => (),
                    _ => return Err(format!("Invalid char: {}", c).into()),
                }
            }
        }
        Ok(State { active })
    }

    fn simulate(self, num_rounds: usize) -> State {
        let mut state = self;
        for _ in 0..num_rounds {
            state = state.evolve();
        }
        state
    }

    fn evolve(&self) -> State {
        // Anything currently active, or near something active.
        let relevant_points = self.active.iter().flat_map(|&p| {
            let adj = p.adj_points().into_iter();
            Some(p).into_iter().chain(adj)
        });

        let active: HashSet<_> = relevant_points
            .filter(|&p| self.is_active_next_round(p))
            .collect();
        State { active }
    }

    fn is_active_next_round(&self, p: Point) -> bool {
        match (self.active.contains(&p), self.active_neighbours(p)) {
            (true, 2..=3) => true,
            (false, 3) => true,
            _ => false,
        }
    }

    fn active_neighbours(&self, p: Point) -> usize {
        p.adj_points()
            .into_iter()
            .filter(|neighbour| self.active.contains(neighbour))
            .count()
    }

    fn num_active(&self) -> usize {
        self.active.len()
    }
}

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0, z: 0 };

    fn adj_points(self) -> Vec<Point> {
        let mut points = vec![];
        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                for &z in &[-1, 0, 1] {
                    let p = Point { x, y, z };
                    if p != Point::ORIGIN {
                        points.push(self + p);
                    }
                }
            }
        }
        points
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, Point { x, y, z }: Point) -> Point {
        let mut p = self;
        p.x += x;
        p.y += y;
        p.z += z;
        p
    }
}
