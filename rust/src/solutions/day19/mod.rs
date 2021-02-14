use crate::Res;
use dag::parse::parse_dag;
use std::io::{self, prelude::*};

mod dag;

pub fn main() -> Res<()> {
    let lines: Vec<_> = io::stdin().lock().lines().collect::<Result<_, _>>()?;

    let blank = find_blank_line(&lines);
    let rules = &lines[..blank];
    let queries = &lines[blank + 1..];

    let dag = parse_dag(rules.iter().map(String::as_str));
    dbg!(rules.len(), queries.len(), dag.upper_bound());

    Ok(())
}

fn find_blank_line(lines: &[String]) -> usize {
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "" {
            return i;
        }
    }
    panic!("No blank line");
}
