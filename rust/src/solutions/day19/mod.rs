use crate::Res;
use std::io::{self, prelude::*};
use input::read_input;
use nfa::Nfa;

mod input;
mod nfa;

pub fn main() -> Res<()> {
    let lines: Vec<_> = io::stdin().lock().lines().collect::<Result<_, _>>()?;
    let input = read_input(&lines);

    let nfa = Nfa::new(input.rules.iter().map(String::as_str));
    let accepted = nfa.compute_accepted_set();

    let num_valid = input.queries.iter().filter(|&q| accepted.contains(q)).count();
    println!("{}", num_valid);

    Ok(())
}
