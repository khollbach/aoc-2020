use regex::Regex;
use std::error::Error;
use std::io::{self, prelude::*};

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    let input = read_input(io::stdin().lock())?;
    println!("{}", count_valid(&input, Row::policy1));
    println!("{}", count_valid(&input, Row::policy2));
    Ok(())
}

struct Row {
    low: usize,
    high: usize,
    c: char,
    password: String,
}

fn read_input<R: BufRead>(input: R) -> Res<Vec<Row>> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();

    let mut rows = vec![];
    for line in input.lines() {
        let line = line?;
        let caps = re
            .captures(&line)
            .ok_or_else(|| format!("Invalid line: {}", line))?;

        let low: usize = caps[1].parse()?;
        let high: usize = caps[2].parse()?;
        let c = caps[3].chars().next().unwrap();
        let password = String::from(&caps[4]);
        rows.push(Row {
            low,
            high,
            c,
            password,
        });
    }
    Ok(rows)
}

fn count_valid<P>(input: &[Row], is_valid: P) -> usize
where
    P: Fn(&Row) -> bool,
{
    input.iter().filter(|r| is_valid(&r)).count()
}

impl Row {
    fn policy1(&self) -> bool {
        let count = self.password.chars().filter(|&a| a == self.c).count();
        (self.low..=self.high).contains(&count)
    }

    fn policy2(&self) -> bool {
        // Check indeces.
        let n = self.password.len();
        if !(1..=n).contains(&self.low) || !(1..=n).contains(&self.high) {
            return false;
        }

        // Assume ascii.
        let p = self.password.as_bytes();
        let a = p[self.low - 1] as char;
        let b = p[self.high - 1] as char;

        (a == self.c) ^ (b == self.c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() -> Res<()> {
        let input = BufReader::new(File::open("input")?);
        let input = read_input(input)?;
        assert_eq!(count_valid(&input, Row::policy1), 416);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("input")?);
        let input = read_input(input)?;
        assert_eq!(count_valid(&input, Row::policy2), 688);
        Ok(())
    }
}
