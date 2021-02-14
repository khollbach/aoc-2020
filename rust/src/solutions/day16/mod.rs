use crate::Res;
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

    fn part2(mut self) -> u64 {
        self.remove_invalid_tickets();

        let fields = self.compute_field_order();

        fields
            .iter()
            .enumerate()
            .filter_map(|(i, name)| {
                if name.starts_with("departure ") {
                    Some(self.my_ticket[i] as u64)
                } else {
                    None
                }
            })
            .product()
    }

    fn remove_invalid_tickets(&mut self) {
        let tickets = mem::replace(&mut self.other_tickets, vec![]);

        self.other_tickets = tickets
            .into_iter()
            .filter(|t| self.invalid_fields(t).is_empty())
            .collect();
    }

    fn compute_field_order(&self) -> Vec<String> {
        let graph = self.compat_graph();

        // This approach won't work in the general case, but it works on our specific input. We
        // simply sort the field names increasing by the number of field positions they are
        // compatible with. In our input these edge-counts are [1, 2, 3, ..., 20].
        let mut graph: Vec<_> = graph.into_iter().collect();
        graph.sort_by_key(|(_name, edges)| edges.len());

        assert_eq!(
            graph
                .iter()
                .map(|(_name, edges)| edges.len())
                .collect::<Vec<_>>(),
            (1..=graph.len()).collect::<Vec<_>>()
        );

        let mut fields: Vec<Option<String>> = vec![None; graph.len()];

        // For each field name, find the first compatible field index that isn't "taken"; there
        // should be exactly one such index. Rinse and repeat until all 20 have been matched.
        for (name, edges) in graph {
            let mut available_idxs = edges.iter().copied().filter(|&idx| fields[idx].is_none());
            assert_eq!(available_idxs.clone().count(), 1);

            let idx = available_idxs.next().unwrap();
            fields[idx] = Some(name);
        }

        let fields: Option<Vec<String>> = fields.into_iter().collect();
        fields.unwrap()
    }

    /// Compute a compatibility graph that maps field names to possible field indeces.
    fn compat_graph(&self) -> HashMap<String, Vec<usize>> {
        let mut graph = HashMap::new();

        let n = self.constraints.len();
        for (name, constr) in self.constraints.iter() {
            let edges: Vec<_> = (0..n)
                .filter(|&field_idx| self.all_tickets_satisfy(field_idx, constr))
                .collect();

            graph.insert(String::from(name), edges);
        }

        graph
    }

    /// Do all tickets satisfy this `constraint` in their values for this `field_idx`?
    fn all_tickets_satisfy(&self, field_idx: usize, constr: &Constraint) -> bool {
        self.other_tickets
            .iter()
            .all(|t| constr.check(t[field_idx]))
    }
}
