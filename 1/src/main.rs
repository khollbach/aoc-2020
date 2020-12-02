use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::BufRead;

fn main() {
    let nums = read_nums(io::stdin().lock()).unwrap();

    let (x, y) = two_sum(&nums, 2020).unwrap();
    println!("{}", x * y);

    let (x, y, z) = three_sum(&nums, 2020).unwrap();
    println!("{}", x * y * z);
}

fn read_nums<R: BufRead>(input: R) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut nums = vec![];

    for line in input.lines() {
        let line = line?;

        let n: i32 = line.parse()?;
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
    fn part1() {
        let input = BufReader::new(File::open("input").unwrap());
        let nums = read_nums(input).unwrap();

        let (x, y) = two_sum(&nums, 2020).unwrap();
        assert_eq!(x * y, 731731);
    }

    #[test]
    fn part2() {
        let input = BufReader::new(File::open("input").unwrap());
        let nums = read_nums(input).unwrap();

        let (x, y, z) = three_sum(&nums, 2020).unwrap();
        assert_eq!(x * y * z, 116115990);
    }
}
