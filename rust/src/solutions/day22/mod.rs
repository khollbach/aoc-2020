use crate::Res;
use std::io;
use input::{read_input, Deck};

mod input;

/*
- read input: two lists of numbers (stripping the two headers)
- simulate the game, until someone wins
- win condition: a deck is empty
- decks are queues
- round iteration:
    - compare top cards of each deck
    - push both to the bottom of the winner's deck
 */

pub fn main() -> Res<()> {
    let (d1, d2) = read_input(io::stdin().lock());

    let final_deck = part1(d1, d2);
    println!("{}", score(&final_deck));

    Ok(())
}

/// Simulate playing the game, until someone wins.
///
/// Return the state of the winning deck at the time the game ends.
fn part1(mut d1: Deck, mut d2: Deck) -> Deck {
    while !d1.is_empty() && !d2.is_empty() {
        let a = d1.pop_front().unwrap();
        let b = d2.pop_front().unwrap();

        if a > b {
            d1.push_back(a);
            d1.push_back(b);
        } else {
            d2.push_back(b);
            d2.push_back(a);
        }
    }

    if !d1.is_empty() {
        d1
    } else {
        d2
    }
}

fn score(deck: &Deck) -> u32 {
    let n = deck.len() as u32;
    deck.iter().rev().zip(1..=n).map(|(a, i)| a * i).sum()
}
