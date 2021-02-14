use crate::Res;
use std::collections::HashMap;

pub fn main() -> Res<()> {
    println!("{}", memory_game(&[8, 11, 0, 19, 1, 2], 2020));
    println!("{}", memory_game(&[8, 11, 0, 19, 1, 2], 30_000_000));
    Ok(())
}

fn memory_game(prefix: &[u32], n: usize) -> u32 {
    assert!(!prefix.is_empty());

    let mut occurances = HashMap::<u32, Vec<usize>>::new();
    for (i, &val) in prefix.iter().enumerate() {
        occurances.entry(val).or_default().push(i);
    }

    let mut prev = prefix[prefix.len() - 1];
    for i in prefix.len()..n {
        let curr = match occurances[&prev].len() {
            0 => unreachable!(),
            1 => 0,
            len => {
                let gap_size = occurances[&prev][len - 1] - occurances[&prev][len - 2];
                gap_size as u32
            }
        };
        occurances.entry(curr).or_default().push(i);
        prev = curr;
    }

    prev
}
