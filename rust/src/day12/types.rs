use crate::Res;
use std::io::prelude::*;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };

    pub fn l1_norm(self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32
    }
}

#[derive(Clone, Copy)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn new(c: char) -> Option<Dir> {
        use Dir::*;
        match c {
            'N' => Some(N),
            'E' => Some(E),
            'S' => Some(S),
            'W' => Some(W),
            _ => None,
        }
    }

    pub fn rotate(self, r: Rotation) -> Dir {
        use Dir::*;
        let mut idx = match self {
            E => 0,
            N => 1,
            W => 2,
            S => 3,
        };
        idx = (idx + r.num_times_ccw()) % 4;
        match idx {
            0 => E,
            1 => N,
            2 => W,
            3 => S,
            _ => unreachable!(),
        }
    }
}

/// Counter-clockwise rotation by a multiple of 90 degrees.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
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
        let mut num_rots = n / 90 % 4;
        if c == 'R' {
            // Flip by 180 degrees.
            num_rots = 4 - num_rots;
        }
        use Rotation::*;
        let r = match num_rots {
            0 => CCW_0,
            1 => CCW_90,
            2 => CCW_180,
            3 => CCW_270,
            _ => unreachable!(),
        };
        Ok(r)
    }

    fn num_times_ccw(self) -> u8 {
        use Rotation::*;
        match self {
            CCW_0 => 0,
            CCW_90 => 1,
            CCW_180 => 2,
            CCW_270 => 3,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Forward(u32),
    Direct(Dir, u32),
    Rotate(Rotation),
}

impl Action {
    fn new(c: char, n: u32) -> Res<Action> {
        use Action::*;
        if c == 'F' {
            return Ok(Forward(n));
        } else if let Some(d) = Dir::new(c) {
            return Ok(Direct(d, n));
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
