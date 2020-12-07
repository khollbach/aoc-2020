use std::error::Error;
use std::io::{self, prelude::*};

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    let nums = read_input(io::stdin().lock())?;

    let max = nums.iter().max().unwrap();
    println!("{}", max);

    let missing = find_missing(nums).unwrap();
    println!("{}", missing);

    Ok(())
}

fn read_input(input: impl BufRead) -> Res<Vec<u32>> {
    let mut nums = vec![];
    for line in input.lines() {
        let line = line?;
        nums.push(parse_num(&line)?);
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
        acc |= bit;
        acc <<= 1;
    }
    acc >>= 1;
    Ok(acc)
}

fn find_missing(mut nums: Vec<u32>) -> Option<u32> {
    nums.sort_unstable();
    for i in 1..nums.len() {
        let prev = nums[i - 1];
        let curr = nums[i];
        if curr > prev + 1 {
            return Some(prev + 1);
        }
    }
    None
}
