use std::io;
use lazy_static::lazy_static;
use crate::Res;
use input::read_input;

/*
inputs are:
    7^a
    7^b
and we want:
    7^ab

find a and b by trial-and-error, then compute 7^ab
note that we'll want a fast way to compute 7^n

this is all mod 2020_12_27
 */

mod input;

pub fn main() -> Res<()> {
    let (key1, key2) = read_input(io::stdin().lock());

    let ans = part_1(key1, key2);
    println!("{}", ans);

    Ok(())
}

fn part_1(key1: u64, key2: u64) -> u64 {
    let a = log_7(key1).unwrap();
    let b = log_7(key2).unwrap();

    // Note: do _not_ wrap this (% MOD).
    let ab = a * b;

    power_of_7(ab)
}

const MOD: u64 = 2020_12_27; // ~20 M

/// Find a value e such that n == 7^e % MOD.
fn log_7(n: u64) -> Option<u64> {
    assert!(n < MOD);

    for e in 0..MOD {
        if power_of_7(e) == n {
            return Some(e);
        }
    }

    None
}

/// Compute 7^n % MOD.
fn power_of_7(n: u64) -> u64 {
    lazy_static! {
        static ref POWERS_OF_7: [u64; 64] = precompute_powers_of_7();
    }

    let mut ans = 1;

    for i in 0..64 {
        let ith_bit = n & (1 << i);

        if ith_bit != 0 {
            ans = ans * POWERS_OF_7[i] % MOD;
        }
    }

    ans
}

/// For each exponent e in 2^0, ..., 2^63, compute 7^e % MOD.
fn precompute_powers_of_7() -> [u64; 64] {
    let mut table = [0u64; 64];

    table[0] = 7;
    for i in 1..64 {
        table[i] = table[i-1].pow(2) % MOD;
    }

    table
}
