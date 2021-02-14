use crate::Res;
use std::io;
use types::Action::{self, Forward, Rotate, Translate};
use types::{Direction, Point};

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
        ship.take_action_1(a);
    }
    ship.pos.manhattan_norm()
}

fn part2(actions: &[Action]) -> u32 {
    let mut ship = Ship::new();
    for &a in actions {
        ship.take_action_2(a);
    }
    ship.pos.manhattan_norm()
}

struct Ship {
    pos: Point,
    dir: Direction,
    waypoint: Point,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            pos: Point::ORIGIN,
            dir: Direction::E,
            waypoint: Point { x: 10, y: 1 },
        }
    }

    /// Part 1 "actions"; ignores waypoint.
    fn take_action_1(&mut self, a: Action) {
        match a {
            Forward(n) => self.pos += Point::cardinal(self.dir) * n,
            Translate(d, n) => self.pos += Point::cardinal(d) * n,
            Rotate(r) => self.dir = self.dir.rotate(r),
        }
    }

    /// Part 2 "actions"; mostly moves the waypoint.
    fn take_action_2(&mut self, a: Action) {
        match a {
            Forward(n) => self.pos += self.waypoint * n,
            Translate(d, n) => self.waypoint += Point::cardinal(d) * n,
            Rotate(r) => self.waypoint = self.waypoint.rotate_about_origin(r),
        }
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
