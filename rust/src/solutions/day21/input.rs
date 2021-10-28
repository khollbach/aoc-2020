use regex::Regex;
use lazy_static::lazy_static;
use super::Res;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ingredient<'a>(pub &'a str);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Allergen<'a>(pub &'a str);

#[derive(Debug)]
pub struct Food<'a> {
    pub ingredients: Vec<Ingredient<'a>>,
    pub allergens: Vec<Allergen<'a>>,
}

pub fn read_input(lines: &[String]) -> Res<Vec<Food>> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
    }

    let mut foods = Vec::with_capacity(lines.len());

    for (i, line) in lines.iter().enumerate() {
        match REGEX.captures(&line) {
            None => return Err(format!("Input line {:?} malformed: {:?}", i, line).into()),
            Some(caps) => {
                let ingredients: Vec<_> = caps.get(1).unwrap().as_str().split(' ').map(Ingredient).collect();
                let allergens: Vec<_> = caps.get(2).unwrap().as_str().split(", ").map(Allergen).collect();
                foods.push(Food { ingredients, allergens });
            }
        }
    }

    Ok(foods)
}
