//! Helper functions for reading an NFA from the list of rules that describe it.

use std::collections::HashMap;
use super::{Nfa, Label, Node};

/// Helper for `Nfa::new`.
pub fn parse_rules<'a>(rules: impl Iterator<Item = &'a str>) -> Nfa {
    let mut nodes = HashMap::new();

    for rule in rules {
        let (label, node) = parse_rule(rule);
        let ret = nodes.insert(label, node);
        assert!(ret.is_none(), "Label defined twice: {:?}", label);
    }

    Nfa {
        root: Label(0),
        nodes,
    }
}

fn parse_rule(rule: &str) -> (Label, Node) {
    let mut halves = rule.split(": ");
    assert_eq!(halves.clone().count(), 2);

    let label: u32 = halves.next().unwrap().parse().unwrap();
    let rest = halves.next().unwrap();

    let node = if rest == "\"a\"" {
        Node::Leaf { c: 'a' }
    } else if rest == "\"b\"" {
        Node::Leaf { c: 'b' }
    } else {
        let groups = rest
            .split(" | ")
            .map(|g| {
                g.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .map(|n| Label(n))
                    .collect()
            })
            .collect();

        Node::Branch { groups }
    };

    (Label(label), node)
}
