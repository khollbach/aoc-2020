use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() {
    let passports = read_input(io::stdin()).unwrap();

    println!("{}", count_valid_passports(&passports, false));
    println!("{}", count_valid_passports(&passports, true));
}

type Passport = HashMap<String, String>;

fn read_input<R: Read>(mut input: R) -> Result<Vec<Passport>, Box<dyn Error>> {
    let re = Regex::new(r"^(.+):(.+)$").unwrap();

    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    let mut passports = vec![];

    for paragraph in buf.split("\n\n") {
        let mut fields = HashMap::new();

        for word in paragraph.split_whitespace() {
            let caps = match re.captures(word) {
                Some(c) => c,
                None => return Err(format!("Invalid entry: {}", word).into()),
            };

            let key = &caps[1];
            let val = &caps[2];
            if let Some(old_val) = fields.insert(key.into(), val.into()) {
                return Err(format!("Duplicate entry {}: {} / {}", key, old_val, val).into());
            }
        }

        passports.push(fields);
    }

    Ok(passports)
}

fn is_valid<F>(passport: &Passport, req_fields: &HashSet<String>, is_valid_entry: F) -> bool
where
    F: Fn(&str, &str) -> bool,
{
    let has_req_fields = req_fields.iter().all(|f| passport.contains_key(f));
    let all_entries_valid = passport.iter().all(|(k, v)| is_valid_entry(k, v));

    has_req_fields && all_entries_valid
}

fn is_valid_entry(key: &str, val: &str) -> bool {
    let range = |v: &str, low, high| v.parse().map(|n| low <= n && n <= high).unwrap_or(false);
    let range_4_digits = |v: &str, low, high| v.chars().count() == 4 && range(v, low, high);

    match key {
        "cid" => true,
        "byr" => range_4_digits(val, 1920, 2002),
        "iyr" => range_4_digits(val, 2010, 2020),
        "eyr" => range_4_digits(val, 2020, 2030),
        "hgt" => {
            if let Some(caps) = Regex::new(r"^(\d+)cm$").unwrap().captures(val) {
                range(&caps[1], 150, 193)
            } else if let Some(caps) = Regex::new(r"^(\d+)in$").unwrap().captures(val) {
                range(&caps[1], 59, 76)
            } else {
                false
            }
        }
        "hcl" => Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(val),
        "ecl" => match val {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        },
        "pid" => Regex::new(r"^[0-9]{9}$").unwrap().is_match(val),
        _ => false,
    }
}

fn count_valid_passports(passports: &[Passport], strict_validation: bool) -> usize {
    let req_fields: HashSet<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(String::from)
        .collect();

    // Toggle strict validation of entries (part 1 vs 2).
    let is_valid_fn = |k: &str, v: &str| {
        if strict_validation {
            is_valid_entry(k, v)
        } else {
            k == "cid" || req_fields.contains(k)
        }
    };

    passports
        .iter()
        .filter(|&p| is_valid(p, &req_fields, is_valid_fn))
        .count()
}
