use std::collections::HashMap;

mod new;
mod compute_accepted;

/// A pattern, represented as a tree of rules.
pub struct Pattern {
    pub root: Label,
    pub nodes: HashMap<Label, Node>,
}

/// The unique id of a "rule".
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Label(pub u32);

/// A node in the pattern tree.
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Leaf {
        c: char,
    },
    Branch {
        /// The groups are joined by "alternation" (aka "or").
        ///
        /// Each individual group is a list of labels, joined by concatenation (aka "and").
        groups: Vec<Vec<Label>>,
    },
}
