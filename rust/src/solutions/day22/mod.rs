use std::collections::HashSet;
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

    let final_deck = part1(d1.clone(), d2.clone());
    println!("{}", score(&final_deck));

    let (_player, deck) = part2(d1, d2);
    println!("{}", score(&deck));

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

/*
loop:
    if we're in a repeated state, break out, p1 wins
    if either deck is empty, other player wins

    take top card of each deck
    if there's enough in each deck to recurse,
        do so
    else
        play normally
    winner gets both cards
 */

use Player::{Player1, Player2};

enum Player {
    Player1,
    Player2,
}

/// Simulate playing a game of Recursive Combat.
///
/// Return the state of the winning deck at the end.
fn part2(mut d1: Deck, mut d2: Deck) -> (Player, Deck) {
    let mut states_seen = HashSet::new();

    loop {
        // Game over?
        if d1.is_empty() {
            return (Player2, d2);
        }
        if d2.is_empty() {
            return (Player1, d1);
        }

        // Repeated state?
        let state = (d1.clone(), d2.clone());
        if states_seen.contains(&state) {
            return (Player1, d1);
        }
        states_seen.insert(state);

        // Top two cards.
        let a = d1.pop_front().unwrap();
        let b = d2.pop_front().unwrap();

        // Who won?
        let winner = if d1.len() >= a as usize && d2.len() >= b as usize {
            let deck1 = d1.iter().copied().take(a as usize).collect();
            let deck2 = d2.iter().copied().take(b as usize).collect();
            let (player, _deck) = part2(deck1, deck2);
            player
        } else if a > b {
            Player1
        } else {
            Player2
        };

        // Winner gets both cards; winning card on top.
        match winner {
            Player1 => {
                d1.push_back(a);
                d1.push_back(b);
            }
            Player2 => {
                d2.push_back(b);
                d2.push_back(a);
            }
        }
    }
}
