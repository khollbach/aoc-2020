use super::Res;
use std::io;
use types::{Action, Dir, Point, Rotation};

mod types;

pub fn main() -> Res<()> {
    let actions = Action::read_actions(io::stdin().lock())?;
    println!("{}", part1(&actions));
    println!("{}", part2(&actions));
    Ok(())
}

fn part1(actions: &[Action]) -> u32 {
    let mut ship = Ship::new();
    for &a in actions {
        ship.take_action(a);
    }
    ship.pos.l1_norm()
}

fn part2(actions: &[Action]) -> u32 {
    let mut ship = Ship::new();
    for &a in actions {
        ship.move_waypoint(a);
    }
    ship.pos.l1_norm()
}

struct Ship {
    pos: Point,
    dir: Dir,
    waypoint: Point,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            pos: Point::ORIGIN,
            dir: Dir::E,
            waypoint: Point { x: 10, y: 1 },
        }
    }

    /// Part1 "actions"; ignores waypoint.
    fn take_action(&mut self, a: Action) {
        use Action::*;
        match a {
            Forward(n) => self.pos.move_dir(self.dir, n),
            Translate(d, n) => self.pos.move_dir(d, n),
            Rotate(r) => self.dir = self.dir.rotate(r),
        }
    }

    /// Part2 "actions"; mostly moves the waypoint.
    fn move_waypoint(&mut self, a: Action) {
        use Action::*;
        match a {
            Forward(n) => self.pos.move_toward(self.waypoint, n),
            Translate(d, n) => self.waypoint.move_dir(d, n),
            Rotate(r) => self.waypoint.rotate_about_origin(r),
        }
    }
}

impl Point {
    fn move_dir(&mut self, d: Dir, n: u32) {
        use Dir::*;
        let n = n as i32;
        match d {
            N => self.y += n,
            S => self.y -= n,
            E => self.x += n,
            W => self.x -= n,
        }
    }

    fn move_toward(&mut self, Point { x, y }: Point, n: u32) {
        let n = n as i32;
        self.x += x * n;
        self.y += y * n;
    }

    fn rotate_about_origin(&mut self, r: Rotation) {
        use Rotation::*;
        let (x, y) = match r {
            CCW_0 => (self.x, self.y),
            CCW_90 => (-self.y, self.x),
            CCW_180 => (-self.x, -self.y),
            CCW_270 => (self.y, -self.x),
        };
        self.x = x;
        self.y = y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/12")?);
        let actions = Action::read_actions(input)?;
        assert_eq!(super::part1(&actions), 415);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/12")?);
        let actions = Action::read_actions(input)?;
        assert_eq!(super::part2(&actions), 29401);
        Ok(())
    }
}
