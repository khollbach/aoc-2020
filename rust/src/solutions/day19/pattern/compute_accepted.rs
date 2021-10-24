use std::collections::{HashMap, HashSet};
use std::iter;
use super::{Label, Pattern, Node};

impl Pattern {
    /// Compute the accepted sets of each label. Likely pretty expensive.
    pub fn compute_accepted_sets(&self) -> HashMap<Label, HashSet<String>> {
        let mut memo = HashMap::new();
        self.accepted_helper(self.root, &mut memo);
        memo
    }

    /// Helper function for `compute_accepted_sets`.
    ///
    /// Recursively compute the accepted set of this label. Update `memo` accordingly.
    fn accepted_helper(&self, label: Label, memo: &mut HashMap<Label, HashSet<String>>) {
        if memo.get(&label).is_some() {
            return;
        }

        let accepted: HashSet<String> = match &self.nodes[&label] {
            Node::Leaf { c } => iter::once(c.to_string()).collect(),
            Node::Branch { groups } => {
                let accepted_set_of_each_group: Vec<HashSet<String>> = groups
                    .iter()
                    .map(|g| cartesian_product(&self.accepted_set_of_each_label(&g, memo)))
                    .collect();

                union(accepted_set_of_each_group)
            }
        };

        memo.insert(label, accepted);
    }

    /// Helper function for `accepted_helper`.
    ///
    /// Recursively compute the accepted set of each label in a group.
    fn accepted_set_of_each_label<'a>(
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

/// Compute all possible concatenations.
///
/// Specifically, if the input is the list of sets [S_1, ..., S_n], then
/// the output is the set S_1 x ... x S_n, where `x` is cartesian product.
///
/// For example:
/// In: [{"ab"}, {"", "c"}]
/// Out: {"ab", "abc"}
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

/// Compute the union of the input sets.
///
/// For example:
/// In: [{"ab", "c"}, {"c", ""}]
/// Out: {"ab", "c", ""}
fn union(sets: Vec<HashSet<String>>) -> HashSet<String> {
    let len = sets.iter().map(|s| s.len()).sum();
    let mut acc = HashSet::with_capacity(len);

    for set in sets {
        acc.extend(set.into_iter());
    }

    acc
}