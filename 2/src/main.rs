use regex::Regex;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() {
    let input = read_input(io::stdin().lock()).unwrap();

    println!("{}", num_correct(&input, policy1));
    println!("{}", num_correct(&input, policy2));
}

struct Row {
    a: usize,
    b: usize,
    c: char,
    p: String,
}

fn read_input<R: BufRead>(input: R) -> Result<Vec<Row>, Box<dyn Error>> {
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();

    let mut rows = vec![];

    for line in input.lines() {
        let line = line?;
        let caps = re
            .captures(&line)
            .ok_or_else(|| format!("Invalid line: {}", line))?;

        let a: usize = caps[1].parse()?;
        let b: usize = caps[2].parse()?;
        let c = caps[3].chars().next().unwrap();
        let p = String::from(&caps[4]);

        rows.push(Row { a, b, c, p });
    }

    Ok(rows)
}

fn num_correct<F>(input: &[Row], satisfies_policy: F) -> usize
where
    F: Fn(&str, char, usize, usize) -> bool,
{
    input
        .iter()
        .filter(|&r| satisfies_policy(&r.p, r.c, r.a, r.b))
        .count()
}

fn policy1(password: &str, c: char, low: usize, high: usize) -> bool {
    let count = password.chars().filter(|&a| a == c).count();
    (low..=high).contains(&count)
}

fn policy2(password: &str, c: char, first: usize, second: usize) -> bool {
    let n = password.len();
    assert!((1..=n).contains(&first) && (1..=n).contains(&second));

    // Assume ascii.
    let p = password.as_bytes();

    let a = p[first - 1] as char;
    let b = p[second - 1] as char;
    (a == c) ^ (b == c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() {
        let input = BufReader::new(File::open("input").unwrap());
        let input = read_input(input).unwrap();

        assert_eq!(num_correct(&input, policy1), 416);
    }

    #[test]
    fn part2() {
        let input = BufReader::new(File::open("input").unwrap());
        let input = read_input(input).unwrap();

        assert_eq!(num_correct(&input, policy2), 688);
    }
}
