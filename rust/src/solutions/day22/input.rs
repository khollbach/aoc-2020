use std::collections::VecDeque;
use std::io::{self, BufRead};

pub type Deck = VecDeque<u32>;

pub fn read_input(input: impl BufRead) -> (Deck, Deck) {
    let mut lines = input.lines();

    let line = lines.next().unwrap().unwrap();
    assert_eq!(line, "Player 1:");
    let d1 = read_deck(&mut lines);

    let line = lines.next().unwrap().unwrap();
    assert_eq!(line, "Player 2:");
    let d2 = read_deck(&mut lines);

    assert!(lines.next().is_none());

    (d1, d2)
}

/// Reads until the first blank line or EOF.
fn read_deck(lines: &mut impl Iterator<Item=Result<String, io::Error>>) -> Deck {
    let mut deck = VecDeque::new();

    for line in lines {
        let line = line.unwrap();
        if line == "" { break; }

        deck.push_back(line.parse().unwrap());
    }

    deck
}
