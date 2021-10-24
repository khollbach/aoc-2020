use crate::Res;
use std::io::{self, prelude::*};
use std::collections::{HashSet, HashMap};
use input::{read_input, Input};
use pattern::{Label, Pattern, Node};

mod input;
mod pattern;

pub fn main() -> Res<()> {
    let lines: Vec<_> = io::stdin().lock().lines().collect::<Result<_, _>>()?;
    let input = read_input(&lines);
    let soln = Solution::new(&input);

    println!("{}", soln.part1());
    println!("{}", soln.part2());

    Ok(())
}

struct Solution<'a> {
    input: &'a Input<'a>,
    pattern: Pattern,
    accepted_sets: HashMap<Label, HashSet<String>>,
}

impl<'a> Solution<'a> {
    fn new(input: &'a Input<'a>) -> Self {
        let pattern = Pattern::new(input.rules.iter().map(String::as_str));
        let accepted_sets = pattern.compute_accepted_sets();

        Self { input, pattern, accepted_sets }
    }

    fn part1(&self) -> usize {
        let accepted = &self.accepted_sets[&self.pattern.root];
        self.num_valid(|s| accepted.contains(s))
    }

    fn part2(&self) -> usize {
        self.assert_part2_assumptions();
        self.num_valid(|s| self.accepted_part2(s))
    }

    fn num_valid(&self, mut is_valid: impl FnMut(&str) -> bool) -> usize {
        self.input.queries.iter().filter(|&s| is_valid(s)).count()
    }

    fn assert_part2_assumptions(&self) {
        assert_eq!(self.pattern.nodes[&self.pattern.root], Node::Branch { groups: vec![vec![Label(8), Label(11)]] });
        assert_eq!(self.pattern.nodes[&Label(8)], Node::Branch { groups: vec![vec![Label(42)]] });
        assert_eq!(self.pattern.nodes[&Label(11)], Node::Branch { groups: vec![vec![Label(42), Label(31)]] });

        // We'll use the fact that rules 42 and 31 have no overlap in what they accept.
        let a1 = &self.accepted_sets[&Label(42)];
        let a2 = &self.accepted_sets[&Label(31)];
        let both = a1.intersection(a2);
        assert_eq!(both.count(), 0);

        // We'll also use the following fact:
        // All the strings accepted by 42 or 31 have the same length.
        let str_lens: HashSet<_> = a1.union(a2).map(String::len).collect();
        assert_eq!(str_lens.len(), 1);
    }

    /// Is `s` accepted according to the changes described in part 2?
    ///
    /// This assumes the inputs are valid, according to `assert_part2_assumptions`.
    fn accepted_part2(&self, s: &str) -> bool {
        // Conceptually, we are performing the following replacements:
        //  8: 42      ==>    8: 42    | 42 8
        // 11: 42 31   ==>   11: 42 31 | 42 11 31
        // What this means in practice is that 8 is a sequence of one or more 42s.
        // And 11 is a sequence of one or more 42s followed by the same number of 31s.
        //
        // Since the root rule is `0: 8 11`, we can simply check if the input string is a sequence of
        // 42s followed by a sequence of strictly fewer 31s.

        let a1 = &self.accepted_sets[&Label(42)];
        let a2 = &self.accepted_sets[&Label(31)];

        // Accepted string length. (The same for both rules.)
        let match_len = a1.iter().next().unwrap().len();

        // Special cases.
        if match_len == 0 {
            return s == "";
        }
        if s.len() % match_len != 0 {
            return false;
        }

        let n = s.len() / match_len; // Total number of matches.
        let num_42s = match Self::expected_num_42s(s, a2, match_len, n) {
            Some(num) => num,
            None => return false,
        };

        // Strictly more than half the matches should be 42s.
        if num_42s * 2 <= n {
            return false;
        }

        // Check the first half is all 42s, and the second half is all 31s.
        let first_half = (0..num_42s).all(|idx| {
            let i = idx * match_len;
            let j = (idx + 1) * match_len;
            a1.contains(&s[i..j])
        });
        let second_half = (num_42s..n).all(|idx| {
            let i = idx * match_len;
            let j = (idx + 1) * match_len;
            a2.contains(&s[i..j])
        });
        first_half && second_half
    }

    /// Helper function for `accepted_part2`.
    ///
    /// Find where to split the input string, based on the first match of 31.
    fn expected_num_42s(s: &str, a2: &HashSet<String>, match_len: usize, n: usize) -> Option<usize> {
        (0..n).find(|&idx| {
            let i = idx * match_len;
            let j = (idx + 1) * match_len;
            a2.contains(&s[i..j])
        })
    }
}
