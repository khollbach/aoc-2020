use super::Res;
use std::io::{self, prelude::*};

pub fn main() -> Res<()> {
    let Input {
        start_time,
        bus_ids,
    } = read_input(io::stdin().lock())?;

    let Output { bus_id, wait_time } = part1(start_time, &bus_ids).expect("bus_ids is empty");
    println!("{}", bus_id * wait_time);
    Ok(())
}

struct Input {
    start_time: u32,
    bus_ids: Vec<u32>,
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

    let mut bus_ids = vec![];
    for word in line.split(',') {
        if word != "x" {
            bus_ids.push(word.parse()?);
        }
    }
    Ok(Input {
        start_time,
        bus_ids,
    })
}

struct Output {
    bus_id: u32,
    wait_time: u32,
}

fn part1(start_time: u32, bus_ids: &[u32]) -> Option<Output> {
    let (wait_time, bus_id) = bus_ids
        .iter()
        .map(|&id| (wait_time(start_time, id), id))
        .min()?;

    Some(Output { bus_id, wait_time })
}

fn wait_time(start_time: u32, bus_id: u32) -> u32 {
    assert_ne!(bus_id, 0);
    let time_since_last_bus = start_time % bus_id;
    bus_id - time_since_last_bus
}
