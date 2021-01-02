use super::Res;
use input::Input;
use std::io;

mod input;

pub fn main() -> Res<()> {
    let input = Input::read(io::stdin().lock())?;
    println!("{}", input.error_rate());
    Ok(())
}

impl Input {
    fn error_rate(&self) -> u32 {
        self.other_tickets
            .iter()
            .map(|t| self.invalid_fields(t).iter().sum::<u32>())
            .sum()
    }

    /// Returns the value of each field that is clearly invalid.
    fn invalid_fields(&self, ticket: &[u32]) -> Vec<u32> {
        ticket
            .iter()
            .filter(|&&val| !self.potentially_valid_field(val))
            .copied()
            .collect()
    }

    /// Satisfies any constraint?
    fn potentially_valid_field(&self, val: u32) -> bool {
        self.constraints.values().any(|c| c.check(val))
    }
}
