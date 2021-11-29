use std::io::prelude::*;

/// Read input into a list of numbers.
///
/// `input` should just be a single line of decimal digits; panics otherwise.
pub fn read_input(input: impl BufRead) -> Vec<u32> {
    let mut lines = input.lines();
    let line = lines.next().unwrap().unwrap();
    assert!(lines.next().is_none());

    let mut nums = Vec::with_capacity(line.len());
    for c in line.chars() {
        let digit = c.to_digit(10).unwrap();
        nums.push(digit);
    }
    nums
}
