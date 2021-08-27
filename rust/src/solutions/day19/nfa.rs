//! Code to represent the NFA for Day 19.

use std::collections::{HashMap, HashSet};
use std::iter;
use new_nfa::parse_rules;

mod new_nfa;

/// Non-deterministic Finite Automaton representing the given list of rules.
pub struct Nfa {
    root: Label,
    nodes: HashMap<Label, Node>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Label(u32);

enum Node {
    Leaf {
        c: char,
    },
    Branch {
        /// The groups are joined by "alternation" (aka "or").
        /// Each group is a list of labels, joined by concatenation (aka "and").
        groups: Vec<Vec<Label>>,
    },
}

impl Nfa {
    pub fn new<'a>(rules: impl Iterator<Item = &'a str>) -> Self {
        parse_rules(rules)
    }

    /// Compute an upper bound on the size of the accepted set of this DAG.
    #[allow(unused)]
    pub fn upper_bound(&self) -> usize {
        let mut memo = HashMap::new();
        self.ub_helper(self.root, &mut memo)
    }

    /// DP helper for the `upper_bound` method.
    fn ub_helper(&self, label: Label, memo: &mut HashMap<Label, usize>) -> usize {
        if let Some(&size) = memo.get(&label) {
            return size;
        }

        let size = match &self.nodes[&label] {
            Node::Leaf { .. } => 1,
            Node::Branch { groups } => groups
                .iter()
                .map(|g| {
                    g.iter()
                        .map(|&l| self.ub_helper(l, memo))
                        .product::<usize>()
                })
                .sum(),
        };

        memo.insert(label, size);
        size
    }

    /// Compute the accepted set for this DAG. Likely expensive!
    pub fn compute_accepted_set(&self) -> HashSet<String> {
        let mut memo = HashMap::new();
        self.accepted_helper(self.root, &mut memo);
        memo.remove(&self.root).unwrap()
    }

    /// Recursively compute the accepted set for this label; update `memo` accordingly.
    ///
    /// Helper for `compute_accepted_set`.
    fn accepted_helper(&self, label: Label, memo: &mut HashMap<Label, HashSet<String>>) {
        if memo.get(&label).is_some() {
            return;
        }

        let accepted: HashSet<String> = match &self.nodes[&label] {
            Node::Leaf { c } => iter::once(c.to_string()).collect(),
            Node::Branch { groups } => {
                let products: Vec<HashSet<String>> = groups
                    .iter()
                    .map(|g| cartesian_product(&self.accepted_sets(&g, memo)))
                    .collect();

                union(products)
            }
        };

        memo.insert(label, accepted);
    }

    /// Recursively compute the accepted set for each label in a group.
    ///
    /// Helper for `accepted_helper`.
    fn accepted_sets<'a>(
        &self,
        group: &[Label],
        memo: &'a mut HashMap<Label, HashSet<String>>,
    ) -> Vec<&'a HashSet<String>> {
        for &l in group {
            self.accepted_helper(l, memo);
        }

        let mut out = Vec::with_capacity(group.len());
        for l in group {
            out.push(&memo[l]);
        }
        out
    }
}

fn cartesian_product(sets: &[&HashSet<String>]) -> HashSet<String> {
    if sets.is_empty() {
        return iter::once(String::new()).collect();
    }

    let first_set = sets[0];
    let rest = &sets[1..];

    let suffixes = cartesian_product(rest);

    first_set
        .iter()
        .flat_map(|s1| suffixes.iter().map(move |s2| s1.clone() + s2))
        .collect()
}

fn union(sets: Vec<HashSet<String>>) -> HashSet<String> {
    let len = sets.iter().map(|s| s.len()).sum();
    let mut acc = HashSet::with_capacity(len);

    for set in sets {
        acc.extend(set.into_iter());
    }

    acc
}
