use super::{Dag, Label, Node};
use std::collections::HashMap;

pub fn parse_dag<'a>(lines: impl Iterator<Item = &'a str>) -> Dag {
    let mut nodes = HashMap::new();

    for line in lines {
        let (label, node) = parse_line(line);
        let ret = nodes.insert(label, node);
        assert!(ret.is_none(), "Label defined twice: {:?}", label);
    }

    Dag {
        root: Label(0),
        nodes,
    }
}

fn parse_line(line: &str) -> (Label, Node) {
    let mut halves = line.split(": ");
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
