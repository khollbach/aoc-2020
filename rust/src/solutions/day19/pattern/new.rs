use std::collections::HashMap;
use super::{Label, Pattern, Node};

impl Pattern {
    /// Create a new pattern tree from a list of rules.
    ///
    /// We don't check for cycles, we just assume there are none.
    pub fn new<'a>(rules: impl Iterator<Item=&'a str>) -> Self {
        let mut nodes = HashMap::new();

        for rule in rules {
            let (label, node) = parse_rule(rule);
            let ret = nodes.insert(label, node);
            assert!(ret.is_none(), "Label defined twice: {:?}", label);
        }

        assert!(nodes.contains_key(&Label(0)), "Root label 0 never defined.");

        Self { root: Label(0), nodes }
    }
}

/// Parse a rule into a label and a node.
///
/// Branches look like this:
/// 123: 39 86 | 127 32
///
/// Leaves look like this:
/// 32: "a"
fn parse_rule(rule: &str) -> (Label, Node) {
    let mut halves = rule.split(": ");

    let label: u32 = halves.next().unwrap().parse().unwrap();
    let rest = halves.next().unwrap();
    assert!(halves.next().is_none());

    // We assume the only possible leaves are "a" and "b".
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
