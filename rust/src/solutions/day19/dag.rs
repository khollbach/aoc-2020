//! Code to represent the input DAG for Day 19.

pub mod parse;

use std::collections::HashMap;

pub struct Dag {
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

impl Dag {
    /// Compute an upper bound on the size of the accepted set of this DAG.
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
}
