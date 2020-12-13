use super::Res;
use std::io;
use types::{Action, Dir, Point, Rotation};

mod types;

pub fn main() -> Res<()> {
    let actions = Action::read_actions(io::stdin().lock())?;
    println!("{}", part1(&actions));
    Ok(())
}

fn part1(actions: &[Action]) -> u32 {
    let mut ship = Ship::new();
    for &a in actions {
        ship.take_action(a);
    }
    ship.pos.l1_norm()
}

struct Ship {
    pos: Point,
    dir: Dir,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            pos: Point::ORIGIN,
            dir: Dir::E,
        }
    }

    fn take_action(&mut self, a: Action) {
        use Action::*;
        match a {
            Forward(n) => self.move_(self.dir, n),
            Direct(d, n) => self.move_(d, n),
            Rotate(r) => self.rotate(r),
        }
    }

    fn move_(&mut self, d: Dir, n: u32) {
        use Dir::*;
        let n = n as i32;
        match d {
            N => self.pos.y += n,
            S => self.pos.y -= n,
            E => self.pos.x += n,
            W => self.pos.x -= n,
        }
    }

    fn rotate(&mut self, r: Rotation) {
        self.dir = self.dir.rotate(r);
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
}
