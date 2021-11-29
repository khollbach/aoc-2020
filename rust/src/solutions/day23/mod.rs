use crate::Res;
use std::io;
use std::iter;
use input::read_input;
use std::collections::HashMap;

mod input;

pub fn main() -> Res<()> {
    let mut cups = read_input(io::stdin().lock());

    let mut ring = Ring::new(&cups);
    ring.simulate_game(100);
    let ans = ring.to_vec();
    let ans: String = ans[1..].iter().map(u32::to_string).collect();
    println!("{}", ans);

    extend_cups(&mut cups, 1_000_000);
    let mut ring = Ring::new(&cups);
    ring.simulate_game(10_000_000);
    let a = ring.next[&1];
    let b = ring.next[&a];
    println!("{}", a as u64 * b as u64);

    Ok(())
}

fn extend_cups(cups: &mut Vec<u32>, largest_cup: u32) {
    let curr_max = *cups.iter().max().unwrap();
    cups.extend(curr_max + 1..=largest_cup);
}

struct Ring {
    next: HashMap<u32, u32>,
    curr: u32,
    largest_cup: u32,
}

impl Ring {
    fn new(cups: &[u32]) -> Self {
        assert!(!cups.is_empty());
        debug_assert!(!cups.contains(&0));

        let mut next = HashMap::new();

        let n = cups.len();
        for i in 0..n {
            let a = cups[i];
            let b = cups[(i + 1) % n];
            next.insert(a, b);
        }

        let largest_cup = *cups.iter().max().unwrap();

        Self { next, curr: cups[0], largest_cup }
    }

    fn simulate_game(&mut self, num_rounds: u32) {
        for _ in 0..num_rounds {
            self.simulate_round();
        }
    }

    fn simulate_round(&mut self) {
        let removed: Vec<_> = iter::repeat_with(|| self.pop_after(self.curr)).take(3).collect();

        let dest = self.get_dest();

        for cup in removed.into_iter().rev() {
            self.push_after(dest, cup)
        }

        self.curr = self.next[&self.curr];
    }

    fn pop_after(&mut self, target: u32) -> u32 {
        let ret = self.next[&target];
        self.next.insert(target, self.next[&ret]);
        self.next.remove(&ret);
        ret
    }

    fn push_after(&mut self, target: u32, new: u32) {
        let rest = self.next[&target];
        self.next.insert(target, new);
        self.next.insert(new, rest);
    }

    fn get_dest(&self) -> u32 {
        let mut cup = self.curr - 1;

        while !self.next.contains_key(&cup) {
            if cup == 0 {
                cup = self.largest_cup;
            } else {
                cup -= 1;
            }
        }

        cup
    }

    /// Starting from 1.
    fn to_vec(&self) -> Vec<u32> {
        assert!(self.next.contains_key(&1));

        let mut cups = Vec::with_capacity(self.next.len());

        let mut curr = 1;
        loop {
            cups.push(curr);

            curr = self.next[&curr];
            if curr == 1 {
                break;
            }
        }

        cups
    }
}
