use super::Res;
use input::{Constraint, Input};
use std::collections::HashMap;
use std::io;
use std::mem;

mod input;

pub fn main() -> Res<()> {
    let input = Input::read(io::stdin().lock())?;
    println!("{}", input.error_rate());
    println!("{}", input.part2());
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

    fn remove_invalid_tickets(&mut self) {
        let tickets = mem::replace(&mut self.other_tickets, vec![]);
        self.other_tickets = tickets
            .into_iter()
            .filter(|t| self.invalid_fields(t).is_empty())
            .collect();
    }

    fn compute_field_order(&self) -> Vec<String> {
        // Compatibility graph mapping field names to possible field indeces.
        let mut graph = HashMap::<String, Vec<usize>>::new();

        let n = self.constraints.len();
        for (name, constr) in self.constraints.iter() {
            let edges: Vec<_> = (0..n)
                .filter(|&field_idx| self.all_tickets_satisfy(field_idx, constr))
                .collect();
            graph.insert(String::from(name), edges);
        }

        // This approach won't work in the general case, but it works on our specific input. We
        // simply sort the field names increasing by the number of field positions they are
        // compatible with. In our input these edge-counts are [1, 2, 3, ..., 20].
        let mut graph: Vec<_> = graph.into_iter().collect();
        graph.sort_by_key(|(_name, edges)| edges.len());
        debug_assert_eq!(
            graph
                .iter()
                .map(|(_name, edges)| edges.len())
                .collect::<Vec<_>>(),
            (1..=graph.len()).collect::<Vec<_>>()
        );

        let mut fields: Vec<Option<String>> = vec![None; graph.len()];
        for (name, edges) in graph {
            debug_assert_eq!(
                edges.iter().filter(|&&idx| fields[idx].is_none()).count(),
                1
            );
            for idx in edges {
                if fields[idx].is_none() {
                    fields[idx] = Some(name);
                    break;
                }
            }
        }

        let fields: Option<Vec<String>> = fields.into_iter().collect();
        fields.expect("Input didn't have expected structure")
    }

    fn all_tickets_satisfy(&self, field_idx: usize, constr: &Constraint) -> bool {
        self.other_tickets
            .iter()
            .all(|t| constr.check(t[field_idx]))
    }

    fn part2(mut self) -> u64 {
        self.remove_invalid_tickets();
        let mut product = 1u64;
        let fields = self.compute_field_order();
        for (i, name) in fields.iter().enumerate() {
            if name.starts_with("departure ") {
                product *= self.my_ticket[i] as u64;
            }
        }
        product
    }
}
