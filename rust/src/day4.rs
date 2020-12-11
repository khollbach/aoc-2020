use super::Res;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, prelude::*};

pub fn main() -> Res<()> {
    let passports = read_input(io::stdin())?;
    println!("{}", num_valid(&passports, false));
    println!("{}", num_valid(&passports, true));
    Ok(())
}

fn num_valid(passports: &[Passport], strict: bool) -> usize {
    passports.iter().filter(|&p| p.is_valid(strict)).count()
}

struct Passport {
    fields: HashMap<String, String>,
}

fn read_input<R: Read>(mut input: R) -> Res<Vec<Passport>> {
    lazy_static! {
        static ref WORD_RE: Regex = Regex::new(r"^(.+):(.+)$").unwrap();
    }

    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let mut passports = vec![];
    for paragraph in buf.split("\n\n") {
        let mut fields = HashMap::new();
        for word in paragraph.split_whitespace() {
            let caps = match WORD_RE.captures(word) {
                Some(c) => c,
                None => return Err(format!("Invalid entry: {}", word).into()),
            };

            let key = &caps[1];
            let val = &caps[2];
            if let Some(old_val) = fields.insert(String::from(key), String::from(val)) {
                return Err(format!("Duplicate entry {}: {} / {}", key, old_val, val).into());
            }
        }
        passports.push(Passport { fields });
    }
    Ok(passports)
}

impl Passport {
    const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn is_valid(&self, strict: bool) -> bool {
        self.has_req_fields() && (!strict || self.entries_are_valid())
    }

    fn has_req_fields(&self) -> bool {
        Self::REQ_FIELDS
            .iter()
            .all(|&k| self.fields.contains_key(k))
    }

    fn entries_are_valid(&self) -> bool {
        self.fields.iter().all(|(k, v)| Self::is_valid_entry(k, v))
    }

    fn is_valid_entry(key: &str, val: &str) -> bool {
        lazy_static! {
            static ref CM_RE: Regex = Regex::new(r"^(\d+)cm$").unwrap();
            static ref IN_RE: Regex = Regex::new(r"^(\d+)in$").unwrap();
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }

        fn is_in_range(v: &str, low: u32, high: u32) -> bool {
            match v.parse() {
                Ok(n) => (low..=high).contains(&n),
                Err(_) => false,
            }
        }

        fn range_4_digits(v: &str, low: u32, high: u32) -> bool {
            v.chars().count() == 4 && is_in_range(v, low, high)
        }

        match key {
            "cid" => true,
            "byr" => range_4_digits(val, 1920, 2002),
            "iyr" => range_4_digits(val, 2010, 2020),
            "eyr" => range_4_digits(val, 2020, 2030),
            "hgt" => {
                if let Some(caps) = CM_RE.captures(val) {
                    is_in_range(&caps[1], 150, 193)
                } else if let Some(caps) = IN_RE.captures(val) {
                    is_in_range(&caps[1], 59, 76)
                } else {
                    false
                }
            }
            "hcl" => HCL_RE.is_match(val),
            "ecl" => match val {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            "pid" => PID_RE.is_match(val),
            _ => false,
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
        let input = BufReader::new(File::open("../inputs/4")?);
        let passports = read_input(input)?;
        assert_eq!(num_valid(&passports, false), 245);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/4")?);
        let passports = read_input(input)?;
        assert_eq!(num_valid(&passports, true), 133);
        Ok(())
    }
}
