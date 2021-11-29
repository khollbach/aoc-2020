use std::io::prelude::*;
use super::dir::Dir;

pub fn read_input(input: impl BufRead) -> Vec<Vec<Dir>> {
    input.lines().map(|line| {
        read_line(&line.unwrap())
    }).collect()
}

fn read_line(line: &str) -> Vec<Dir> {
    let mut dirs = Vec::with_capacity(line.len());

    let mut i = 0;
    while i < line.len() {
        let first = &line[i..i + 1];

        let s = if first == "e" || first == "w" {
            i += 1;
            first
        } else {
            let both = &line[i..i + 2];
            i += 2;
            both
        };

        dirs.push(Dir::new(s));
    }

    dirs.shrink_to_fit();
    dirs
}
