use std::collections::HashSet;
use std::iter;
use super::point::Point;

pub fn part_2(black_tiles: &mut HashSet<Point>) {
    for _ in 0..100 {
        *black_tiles = simulate_day(&black_tiles);
    }
}

fn simulate_day(before: &HashSet<Point>) -> HashSet<Point> {
    let relevant: HashSet<_> = before.iter().flat_map(|&p| {
        iter::once(p).chain(p.neighbors())
    }).collect();

    relevant.into_iter().filter(|&p| should_live(before, p)).collect()
}

fn should_live(before: &HashSet<Point>, p: Point) -> bool {
    let num_adj = p.neighbors().filter(|other| before.contains(other)).count();
    if before.contains(&p) {
        num_adj == 1 || num_adj == 2
    } else {
        num_adj == 2
    }
}
