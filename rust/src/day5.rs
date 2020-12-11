use super::Res;
use std::io::{self, prelude::*};

pub fn main() -> Res<()> {
    let nums = read_input(io::stdin().lock())?;

    let max = nums.iter().max().expect("Empty input");
    println!("{}", max);

    let missing = find_missing(nums).expect("Empty input");
    println!("{}", missing);

    Ok(())
}

fn read_input(input: impl BufRead) -> Res<Vec<u32>> {
    let mut nums = vec![];
    for line in input.lines() {
        nums.push(parse_num(&line?)?);
    }
    Ok(nums)
}

fn parse_num(bsp_code: &str) -> Res<u32> {
    let mut acc = 0;
    for c in bsp_code.chars() {
        let bit = match c {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => return Err(format!("Invalid char: {}", c).into()),
        };
        acc <<= 1;
        acc |= bit;
    }
    Ok(acc)
}

fn find_missing(mut nums: Vec<u32>) -> Option<u32> {
    nums.sort();
    for i in 1..nums.len() {
        let prev = nums[i - 1];
        let curr = nums[i];
        if curr > prev + 1 {
            return Some(prev + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/5")?);
        let nums = read_input(input)?;
        let max = nums.into_iter().max().unwrap();
        assert_eq!(max, 816);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("../inputs/5")?);
        let nums = read_input(input)?;
        let missing = find_missing(nums).unwrap();
        assert_eq!(missing, 539);
        Ok(())
    }
}
