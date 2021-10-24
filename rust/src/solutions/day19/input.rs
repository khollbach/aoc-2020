/// Split the input on the first blank line, to get a list of rules and a list of queries.
///
/// Panics if there's no blank line.
pub fn read_input(lines: &[String]) -> Input {
    let blank = first_blank_line(&lines).expect("No blank line");

    let rules = &lines[..blank];
    let queries = &lines[blank + 1..];

    Input { rules, queries }
}

/// Return value of `read_input`.
pub struct Input<'a> {
    pub rules: &'a [String],
    pub queries: &'a [String],
}

/// Helper function for `read_input`.
fn first_blank_line(lines: &[String]) -> Option<usize> {
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "" {
            return Some(i);
        }
    }
    None
}
