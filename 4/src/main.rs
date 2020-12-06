use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Read;

fn main() {
    let passports = read_input(io::stdin()).unwrap();

    println!("{}", count_valid_passports(&passports));
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

fn is_valid(
    passport: &Passport,
    req_fields: &HashSet<String>,
    opt_fields: &HashSet<String>,
) -> bool {
    let all_fields_valid = passport
        .keys()
        .all(|k| req_fields.contains(k) || opt_fields.contains(k));
    let has_req_fields = req_fields.iter().all(|f| passport.contains_key(f));

    all_fields_valid && has_req_fields
}

fn count_valid_passports(passports: &[Passport]) -> usize {
    let req_fields: HashSet<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(String::from)
        .collect();
    let opt_fields: HashSet<_> = vec!["cid"].into_iter().map(String::from).collect();

    passports
        .iter()
        .filter(|&p| is_valid(p, &req_fields, &opt_fields))
        .count()
}
