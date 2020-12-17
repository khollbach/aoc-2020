use super::Res;
use std::io::{self, prelude::*};

pub fn main() -> Res<()> {
    let input = read_input(io::stdin().lock())?;

    let Part1Ret { bus_id, wait_time } =
        part1(input.start_time, &input.bus_ids()).expect("bus_ids is empty");
    println!("{}", bus_id * wait_time);
    println!("{}", part2(&input.constraints));
    Ok(())
}

#[derive(Debug)]
struct Input {
    start_time: u32,
    constraints: Vec<Constraint>,
}

#[derive(Debug)]
struct Constraint {
    bus_id: u32,
    offset: u32,
}

impl Input {
    fn bus_ids(&self) -> Vec<u32> {
        self.constraints.iter().map(|c| c.bus_id).collect()
    }
}

fn read_input(input: impl BufRead) -> Res<Input> {
    let mut lines = input.lines();
    let start_time = match lines.next() {
        Some(line) => line?.parse()?,
        None => return Err("Empty input".into()),
    };

    let line = match lines.next() {
        Some(line) => line?,
        None => return Err("Expected 2 lines, got only 1".into()),
    };
    if lines.next().is_some() {
        return Err("Too many lines in input; expected only 2".into());
    }

    let mut constraints = vec![];
    for (i, word) in line.split(',').enumerate() {
        if word != "x" {
            let c = Constraint {
                bus_id: word.parse()?,
                offset: i as u32,
            };
            constraints.push(c);
        }
    }
    Ok(Input {
        start_time,
        constraints,
    })
}

struct Part1Ret {
    bus_id: u32,
    wait_time: u32,
}

fn part1(start_time: u32, bus_ids: &[u32]) -> Option<Part1Ret> {
    let (wait_time, bus_id) = bus_ids
        .iter()
        .map(|&id| (wait_time(start_time, id), id))
        .min()?;

    Some(Part1Ret { bus_id, wait_time })
}

fn wait_time(start_time: u32, bus_id: u32) -> u32 {
    assert_ne!(bus_id, 0);
    let time_since_last_bus = start_time % bus_id;
    bus_id - time_since_last_bus
}

fn part2(constraints: &[Constraint]) -> u64 {
    let mut guess = 0;
    let mut increment = 1;
    for c in constraints {
        while !c.check(guess) {
            guess += increment;
        }
        // This assumes all bus_ids are mutually prime.
        // Otherwise we should do lcm here.
        increment *= c.bus_id as u64;
    }
    guess
}

impl Constraint {
    fn check(&self, guess: u64) -> bool {
        (guess + self.offset as u64) % self.bus_id as u64 == 0
    }
}
