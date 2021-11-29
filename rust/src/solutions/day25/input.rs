use std::io::BufRead;

pub fn read_input(input: impl BufRead) -> (u64, u64) {
    let mut lines = input.lines();

    let a = lines.next().unwrap().unwrap().parse().unwrap();
    let b = lines.next().unwrap().unwrap().parse().unwrap();
    assert!(lines.next().is_none());

    (a, b)
}
