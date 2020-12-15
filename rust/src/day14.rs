use super::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, prelude::*};

pub fn main() -> Res<()> {
    let program = read_input(io::stdin().lock())?;
    println!("{}", run(&program, Version::V1));
    println!("{}", run(&program, Version::V2));
    Ok(())
}

fn run(program: &[Statement], version: Version) -> u64 {
    let mut mask = Mask::empty();
    let mut mem = HashMap::new();
    for stmt in program {
        match stmt {
            Statement::Mask(m) => {
                mask = m.clone();
            }
            &Statement::Assign { addr, val } => match version {
                Version::V1 => {
                    mem.insert(addr, mask.apply(val));
                }
                Version::V2 => {
                    for a in mask.decode_addr(addr) {
                        mem.insert(a, val);
                    }
                }
            },
        }
    }
    mem.values().sum()
}

enum Version {
    V1,
    V2,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Mask(Mask),
    Assign { addr: u64, val: u64 },
}

/// E.g.,   XXXX11X0X1 becomes
/// ones:  b0000110001
/// zeros: b0000000100
#[derive(Debug, Clone)]
pub struct Mask {
    mask: String,
    ones: u64,
    zeros: u64,
}

impl Mask {
    fn empty() -> Mask {
        Mask {
            mask: String::new(),
            ones: 0,
            zeros: 0,
        }
    }

    fn apply(&self, mut val: u64) -> u64 {
        val |= self.ones;
        val &= !self.zeros;
        val
    }

    fn decode_addr(&self, addr: u64) -> Vec<u64> {
        assert!(!self.mask.is_empty());
        Self::decode_helper(&self.mask, addr)
    }

    fn decode_helper(mask: &str, addr: u64) -> Vec<u64> {
        if mask.is_empty() {
            return vec![0];
        }
        let n = mask.len();

        let mut prefix = 0;
        for (offset, c) in mask.chars().enumerate() {
            let i = n - 1 - offset;
            let bit = 1 << i;
            match c {
                '0' => prefix |= addr & bit,
                '1' => prefix |= bit,
                'X' => {
                    let mut res = vec![];
                    for suffix in Self::decode_helper(&mask[offset + 1..], addr) {
                        res.push(prefix | suffix); // 0
                        res.push(prefix | bit | suffix); // 1
                    }
                    return res;
                }
                _ => panic!("Invalid char in mask: {}", c),
            }
        }
        vec![prefix]
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

        Mask {
            zeros,
            ones,
            mask: String::from(mask),
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
        let input = BufReader::new(File::open("../inputs/14")?);
        let program = read_input(input)?;
        assert_eq!(run(&program, Version::V1), 4886706177792);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/14")?);
        let program = read_input(input)?;
        assert_eq!(run(&program, Version::V2), 3348493585827);
        Ok(())
    }
}
