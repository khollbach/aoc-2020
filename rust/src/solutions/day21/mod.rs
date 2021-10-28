use crate::Res;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use input::{read_input, Food, Allergen, Ingredient};

mod input;

pub fn main() -> Res<()> {
    let lines: Vec<_> = io::stdin().lock().lines().collect::<Result<_, _>>()?;
    let foods = read_input(&lines)?;

    let map = allergen_intersections(&foods);

    let ans = part1(&foods, &map);
    println!("{}", ans);

    let ans = part2(&map);
    let ans: Vec<_> = ans.into_iter().map(|(_, Ingredient(name))| name).collect();
    println!("{}", ans.join(","));

    Ok(())
}

/// Return type of `allergen_intersections`.
type AllergenMap<'a> = HashMap<Allergen<'a>, HashSet<Ingredient<'a>>>;

/// For each allergen, return the "intersection" of all foods that list that allergen.
/// (That is, intersect the ingredient-list of each of those foods.)
fn allergen_intersections<'a>(foods: &[Food<'a>]) -> AllergenMap<'a> {
    let mut map: AllergenMap = HashMap::new();

    for f in foods {
        for &a in &f.allergens {
            let ingredients = f.ingredients.iter().copied();
            match map.get_mut(&a) {
                None => {
                    map.insert(a, ingredients.collect());
                }
                Some(set) => {
                    // Set-intersection.
                    *set = ingredients.filter(|i| set.contains(i)).collect();
                }
            }
        }
    }

    map
}

/// Count the total number of copies of ingredients that *cannot* contain allergens.
fn part1<'a>(foods: &'a [Food<'a>], map: &'a AllergenMap<'a>) -> usize {
    // If this assumption holds, it makes our lives much easier.
    //
    // We assert that the total number of candidate "translations" for allergens is exactly equal to
    // the number of allergens. (If it's instead larger, then we'll probably have to actually solve
    // the whole puzzle to get the answer to part 1.)
    let mut candidates = HashSet::with_capacity(map.len());
    for ingredients in map.values() {
        // Set-union.
        candidates.extend(ingredients.iter().copied());
    }
    assert_eq!(candidates.len(), map.len()); // Same number of candidate-ingredients as allergens.

    let mut count = 0;
    for f in foods {
        for i in &f.ingredients {
            if !candidates.contains(i) {
                count += 1;
            }
        }
    }
    count
}

/// Translate each allergen into its corresponding ingredient.
///
/// We use a trivial, brittle approach that will not work on all inputs. You have been warned!
///
/// Returns a list of translations, sorted alphabetically by allergen.
///
/// Panics if we fail to solve.
fn part2<'a>(map: &AllergenMap<'a>) -> Vec<(Allergen<'a>, Ingredient<'a>)> {
    let mut map = map.clone();
    let mut ret = Vec::with_capacity(map.len());

    // In each iteration of the loop, we solve for one allergen's translation.
    for _ in 0..map.len() {
        let (allergen, ingredient) = get_allergen(&map);

        // Remove this allergen and ingredient from the picture.
        map.remove(&allergen);
        for ingredients in map.values_mut() {
            ingredients.remove(&ingredient);
        }

        ret.push((allergen, ingredient));
    }
    assert!(map.is_empty());

    // Sort alphabetically by allergen.
    ret.sort_unstable();
    ret
}

/// Helper function for `part2`
///
/// Find an allergen which has exactly one candidate translation.
///
/// Panics if this is impossible.
fn get_allergen<'a, 'm>(map: &'m AllergenMap<'a>) -> (Allergen<'a>, Ingredient<'a>) {
    // Find the "minimum" allergen.
    let (allergen, ingredients) = map.iter().min_by_key(|(_, ingredients)| ingredients.len()).unwrap();

    match ingredients.len() {
        0 => panic!("Impossible to solve. Current map: {:?}", map),

        // Good to go.
        1 => (*allergen, *ingredients.iter().next().unwrap()),

        // Our approach won't work. (There may or may not be a solution.)
        _ => panic!("Failed to solve. Current map: {:?}", map),
    }
}