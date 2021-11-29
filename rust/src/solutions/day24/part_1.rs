use std::collections::HashSet;
use super::dir::Dir;
use super::point::Point;

pub fn part_1(input: &[Vec<Dir>]) -> HashSet<Point> {
    let mut black_tiles = HashSet::new();

    for dirs in input {
        let p = follow_dirs(dirs);

        if black_tiles.contains(&p) {
            black_tiles.remove(&p);
        } else {
            black_tiles.insert(p);
        }
    }

    black_tiles
}

fn follow_dirs(dirs: &[Dir]) -> Point {
    let mut p = Point::default();

    for d in dirs {
        p = p + d.to_point();
    }

    p
}
