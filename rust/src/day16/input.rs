use super::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::ops::Range;

#[derive(Debug)]
pub struct Input {
    pub constraints: HashMap<String, Constraint>,
    pub my_ticket: Vec<u32>,
    pub other_tickets: Vec<Vec<u32>>,
}

#[derive(Debug)]
pub struct Constraint {
    pub include: Range<u32>,
    pub exclude: Range<u32>,
}

impl Constraint {
    pub fn check(&self, val: u32) -> bool {
        self.include.contains(&val) && !self.exclude.contains(&val)
    }
}

impl Input {
    pub fn read(mut input: impl BufRead) -> Res<Input> {
        let constraints = Self::read_constraints(&mut input)?;

        Self::check_line(&mut input, "your ticket:")?;
        let my_ticket = Self::read_ticket(&mut input)?;
        Self::check_line(&mut input, "")?;

        Self::check_line(&mut input, "nearby tickets:")?;
        let mut other_tickets = vec![];
        // While there are more lines:
        while !input.fill_buf()?.is_empty() {
            other_tickets.push(Self::read_ticket(&mut input)?);
        }

        Ok(Input {
            constraints,
            my_ticket,
            other_tickets,
        })
    }

    /// Consumes a line and checks it against `expected`.
    fn check_line(input: &mut impl BufRead, expected: &str) -> Res<()> {
        match input.lines().next() {
            Some(line) => {
                let line = line?;
                if line == expected {
                    Ok(())
                } else {
                    Err(format!("Bad input: line did not match '{}': {}", expected, line).into())
                }
            }
            None => Err(format!("Unexpected EOF when expecting '{}'", expected).into()),
        }
    }

    /// Read a line and parse it as a comma-separated list of one or more numbers.
    fn read_ticket(input: &mut impl BufRead) -> Res<Vec<u32>> {
        let line = match input.lines().next() {
            Some(line) => line?,
            None => return Err("Unexpected EOF when reading ticket".into()),
        };
        let mut vals = vec![];
        for word in line.split(',') {
            vals.push(word.parse()?);
        }
        Ok(vals)
    }

    /// Consumes up to and including the first empty line.
    fn read_constraints(input: &mut impl BufRead) -> Res<HashMap<String, Constraint>> {
        let mut contraints = HashMap::new();
        for line in input.lines() {
            let line = line?;
            if line == "" {
                return Ok(contraints);
            }

            let (name, constr) = Self::parse_constraint(&line)?;
            let ret = contraints.insert(String::from(name), constr);
            if ret.is_some() {
                return Err(format!("Field defined twice: {}", name).into());
            }
        }

        Err("Bad input: expected empty line but didn't find one".into())
    }

    fn parse_constraint(line: &str) -> Res<(&str, Constraint)> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }

        let caps = match RE.captures(&line) {
            Some(caps) => caps,
            None => return Err(format!("Line doesn't match constraint regex: {}", line).into()),
        };

        let ll: u32 = caps[2].parse()?;
        let l: u32 = caps[3].parse()?;
        let r: u32 = caps[4].parse()?;
        let rr: u32 = caps[5].parse()?;
        if !(ll <= l && l <= r && r <= rr) {
            return Err(format!("Bounds not increasing: {} {} {} {}", ll, l, r, rr).into());
        }

        let constr = Constraint {
            include: Range {
                start: ll,
                end: rr + 1,
            },
            exclude: Range {
                start: l + 1,
                end: r,
            },
        };

        Ok((caps.get(1).unwrap().as_str(), constr))
    }
}
