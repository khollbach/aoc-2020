use super::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, prelude::*};

// todo: this works on the example, but I get "wrong answer; too high" on the actual input...
pub fn main() -> Res<()> {
    let program = read_input(io::stdin().lock())?;
    println!("{}", run(&program));
    Ok(())
}

/// ```
/// use aoc_2020::day14::{read_input, run};
///
/// let input = "\
/// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
/// mem[8] = 11
/// mem[7] = 101
/// mem[8] = 0
/// ";
/// let program = read_input(input.as_bytes()).unwrap();
/// assert_eq!(run(&program), 165);
/// ```
pub fn run(program: &[Statement]) -> u64 {
    let mut mask = Mask::empty();
    let mut mem = HashMap::new();
    for &stmt in program {
        match stmt {
            Statement::Mask(m) => {
                mask = m;
            }
            Statement::Assign { addr, val } => {
                mem.insert(addr, mask.apply(val));
            }
        }
    }
    mem.values().sum()
}

#[derive(Clone, Copy)]
pub enum Statement {
    Mask(Mask),
    Assign { addr: u64, val: u64 },
}

/// E.g.,   XXXX11X0X1 becomes
/// ones:  b0000110001
/// zeros: b0000000100
#[derive(Clone, Copy)]
pub struct Mask {
    ones: u64,
    zeros: u64,
}

impl Mask {
    fn empty() -> Mask {
        Mask { ones: 0, zeros: 0 }
    }

    fn apply(self, val: u64) -> u64 {
        val | self.ones & !self.zeros
    }
}

pub fn read_input(input: impl BufRead) -> Res<Vec<Statement>> {
    let mut program = vec![];
    for line in input.lines() {
        program.push(Statement::new(&line?)?);
    }
    Ok(program)
}

impl Statement {
    fn new(line: &str) -> Res<Statement> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask = ([X01]{36})$").unwrap();
            static ref ASSN_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }

        if let Some(caps) = MASK_RE.captures(&line) {
            Ok(Statement::Mask(Mask::new(&caps[1])))
        } else if let Some(caps) = ASSN_RE.captures(&line) {
            Ok(Statement::Assign {
                addr: caps[1].parse()?,
                val: caps[2].parse()?,
            })
        } else {
            Err(format!("Line did not match any statement: {}", line).into())
        }
    }
}

impl Mask {
    fn new(mask: &str) -> Mask {
        assert_eq!(mask.len(), 36);

        let mut ones = 0;
        let mut zeros = 0;

        // Reading from right to left.
        for (i, c) in mask.chars().rev().enumerate() {
            match c {
                'X' => (),
                '0' => zeros |= 1 << i,
                '1' => ones |= 1 << i,
                _ => panic!("Invalid char in mask: {}", c),
            }
        }
        debug_assert_eq!(ones & zeros, 0);

        Mask { zeros, ones }
    }
}
