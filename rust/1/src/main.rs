use std::collections::HashSet;
use std::error::Error;
use std::io::{self, prelude::*};

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    let nums = read_nums(io::stdin().lock())?;

    let (x, y) = two_sum(&nums, 2020).expect("No solution");
    println!("{}", x * y);

    let (x, y, z) = three_sum(&nums, 2020).expect("No solution");
    println!("{}", x * y * z);

    Ok(())
}

fn read_nums<R: BufRead>(input: R) -> Res<Vec<i32>> {
    let mut nums = vec![];
    for line in input.lines() {
        let n: i32 = line?.parse()?;
        nums.push(n);
    }
    Ok(nums)
}

fn two_sum(nums: &[i32], target: i32) -> Option<(i32, i32)> {
    let mut seen = HashSet::new();
    for &x in nums {
        let y = target - x;
        if seen.contains(&y) {
            return Some((y, x));
        }
        seen.insert(x);
    }
    None
}

fn three_sum(nums: &[i32], target: i32) -> Option<(i32, i32, i32)> {
    let mut seen = HashSet::new();
    for (i, &x) in nums.iter().enumerate() {
        for &y in nums.iter().skip(i + 1) {
            let z = target - x - y;
            if seen.contains(&z) {
                return Some((z, x, y));
            }
        }
        seen.insert(x);
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
        let input = BufReader::new(File::open("../../inputs/1")?);
        let nums = read_nums(input)?;

        let (x, y) = two_sum(&nums, 2020).unwrap();
        assert_eq!(x * y, 731731);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("../../inputs/1")?);
        let nums = read_nums(input)?;

        let (x, y, z) = three_sum(&nums, 2020).unwrap();
        assert_eq!(x * y * z, 116115990);
        Ok(())
    }
}
