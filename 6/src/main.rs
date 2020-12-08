use std::error::Error;
use std::io::{self, prelude::*};

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    let groups = read_input(io::stdin())?;
    println!("{}", part1(&groups));
    println!("{}", part2(&groups));
    Ok(())
}

fn part1(groups: &[Group]) -> usize {
    groups.iter().map(Group::union).map(|u| u.num_yeses()).sum()
}

fn part2(groups: &[Group]) -> usize {
    groups
        .iter()
        .map(Group::intersection)
        .map(|i| i.num_yeses())
        .sum()
}

struct Person {
    answers: [bool; 26],
}

struct Group {
    people: Vec<Person>,
}

fn read_input<R: Read>(mut input: R) -> Res<Vec<Group>> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let mut groups = vec![];
    for paragraph in buf.split("\n\n") {
        let mut people = vec![];
        for line in paragraph.split('\n') {
            let line = line.trim();
            if !line.is_empty() {
                people.push(Person::new(&line)?);
            }
        }
        groups.push(Group { people });
    }
    Ok(groups)
}

impl Person {
    fn new(line: &str) -> Res<Person> {
        let mut answers = [false; 26];
        for c in line.chars() {
            if !('a' <= c && c <= 'z') {
                return Err(format!("Invalid char: {}", c).into());
            }
            let idx = c as usize - 'a' as usize;
            answers[idx] = true;
        }
        Ok(Person { answers })
    }
}

impl Group {
    fn union(&self) -> Person {
        let mut answers = [false; 26];
        for p in &self.people {
            for (i, &a) in p.answers.iter().enumerate() {
                answers[i] |= a;
            }
        }
        Person { answers }
    }

    fn intersection(&self) -> Person {
        let mut answers = [true; 26];
        for p in &self.people {
            for (i, &a) in p.answers.iter().enumerate() {
                answers[i] &= a;
            }
        }
        Person { answers }
    }
}

impl Person {
    fn num_yeses(&self) -> usize {
        self.answers.iter().filter(|&&b| b).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn part1() -> Res<()> {
        let input = BufReader::new(File::open("input")?);
        let groups = read_input(input)?;
        assert_eq!(super::part1(&groups), 6775);
        Ok(())
    }

    #[test]
    fn part2() -> Res<()> {
        let input = BufReader::new(File::open("input")?);
        let groups = read_input(input)?;
        assert_eq!(super::part2(&groups), 3356);
        Ok(())
    }
}
