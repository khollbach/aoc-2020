use crate::Res;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::prelude::*;
use std::ops::{AddAssign, Mul};
use Action::{Forward, Rotate, Translate};
use Direction::{E, N, S, W};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };

    pub fn cardinal(d: Direction) -> Point {
        let (x, y) = match d {
            N => (0, 1),
            S => (0, -1),
            E => (1, 0),
            W => (-1, 0),
        };
        Point { x, y }
    }

    pub fn rotate_about_origin(self, r: Rotation) -> Point {
        let mut p = self;
        for _ in 0..r.num_times_ccw() {
            p = Point { x: -p.y, y: p.x };
        }
        p
    }

    pub fn manhattan_norm(self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, Point { x, y }: Point) {
        self.x += x;
        self.y += y;
    }
}

impl Mul<u32> for Point {
    type Output = Point;

    fn mul(self, c: u32) -> Point {
        let mut p = self;
        p.x *= c as i32;
        p.y *= c as i32;
        p
    }
}

#[derive(Clone, Copy, FromPrimitive)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn new(c: char) -> Option<Direction> {
        match c {
            'N' => Some(N),
            'E' => Some(E),
            'S' => Some(S),
            'W' => Some(W),
            _ => None,
        }
    }

    pub fn rotate(self, r: Rotation) -> Direction {
        let num_times_cw = 4 - r.num_times_ccw();
        let new_dir = (self as u32 + num_times_cw) % 4;
        Direction::from_u32(new_dir).unwrap()
    }
}

/// Counter-clockwise rotation by a multiple of 90 degrees.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, FromPrimitive)]
pub enum Rotation {
    CCW_0,
    CCW_90,
    CCW_180,
    CCW_270,
}

impl Rotation {
    fn new(c: char, n: u32) -> Res<Rotation> {
        if !(c == 'L' || c == 'R') {
            return Err(format!("Invalid character: {}", c).into());
        }
        if n % 90 != 0 {
            return Err(format!("Invalid number of degrees for rotation: {}", n).into());
        }

        let mut num_times_ccw = n / 90 % 4;
        if c == 'R' {
            // Flip by 180 degrees.
            num_times_ccw = 4 - num_times_ccw;
        }

        Ok(Rotation::from_u32(num_times_ccw).unwrap())
    }

    fn num_times_ccw(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Forward(u32),
    Translate(Direction, u32),
    Rotate(Rotation),
}

impl Action {
    fn new(c: char, n: u32) -> Res<Action> {
        if c == 'F' {
            return Ok(Forward(n));
        } else if let Some(d) = Direction::new(c) {
            return Ok(Translate(d, n));
        } else {
            Ok(Rotate(Rotation::new(c, n)?))
        }
    }

    pub fn read_actions(input: impl BufRead) -> Res<Vec<Action>> {
        let mut actions = vec![];
        for line in input.lines() {
            let line = line?;
            let mut chars = line.chars();
            let c = match chars.next() {
                Some(c) => c,
                None => return Err("Empty line in input".into()),
            };
            let n: u32 = chars.collect::<String>().parse()?;
            if n != 0 {
                actions.push(Action::new(c, n)?);
            }
        }
        Ok(actions)
    }
}
