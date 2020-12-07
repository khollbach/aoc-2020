use std::error::Error;
use std::io::{self, prelude::*};

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    let groups = read_input(io::stdin().lock())?;

    let ans: Res<usize> = groups.iter().map(group_union_size).sum();
    println!("{}", ans?);

    let ans: Res<usize> = groups.iter().map(group_intersection_size).sum();
    println!("{}", ans?);

    Ok(())
}

type Group = Vec<String>;

fn read_input(mut input: impl Read) -> Res<Vec<Group>> {
    let mut groups = vec![];

    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    for paragraph in buf.split("\n\n") {
        let mut g = vec![];
        for line in paragraph.split_whitespace() {
            assert!(!line.is_empty());
            g.push(line.into());
        }
        groups.push(g);
    }

    Ok(groups)
}

fn parse_answers(person: &str) -> Res<[bool; 26]> {
    let mut answers = [false; 26];
    for c in person.chars() {
        if !('a'..='z').contains(&c) {
            return Err(format!("Invalid char: {}", c).into());
        }
        let idx = c as usize - 'a' as usize;
        answers[idx] = true;
    }
    Ok(answers)
}

fn group_union_size(g: &Group) -> Res<usize> {
    let mut answers = [false; 26];
    for person in g {
        for (i, &a) in parse_answers(&person)?.iter().enumerate() {
            answers[i] |= a;
        }
    }
    Ok(answers.iter().copied().filter(|&b| b).count())
}

fn group_intersection_size(g: &Group) -> Res<usize> {
    assert!(!g.is_empty());
    let mut answers = [true; 26];
    for person in g {
        for (i, &a) in parse_answers(&person)?.iter().enumerate() {
            answers[i] &= a;
        }
    }
    Ok(answers.iter().copied().filter(|&b| b).count())
}
