use crate::common::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn parse_input(content: &str) -> Vec<HashMap<&str, &str>> {
    lazy_static! {
        static ref SPLIT_LINES: Regex = Regex::new("[ ]*\n[ ]*\n[ ]*").unwrap();
        static ref FIELD: Regex = Regex::new("([^ \n:]*):([^ \n]*)").unwrap();
    }

    let mut result = vec![];
    for chunk in SPLIT_LINES.split(&content) {
        let mut fields = HashMap::new();

        for part in FIELD.captures_iter(chunk) {
            fields.insert(part.get(1).unwrap().as_str(), part.get(2).unwrap().as_str());
        }

        result.push(fields);
    }

    result
}

fn has_fields(passport: &HashMap<&str, &str>) -> bool {
    const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    all(&FIELDS, |f| passport.contains_key(f))
}

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    fn check_int(value: &str, min: usize, max: usize) -> bool {
        if let Ok(n) = value.parse::<usize>() {
            n >= min && n <= max
        } else {
            false
        }
    };

    fn check_height(value: &str) -> bool {
        let n = value.len();
        if value.ends_with("in") {
            check_int(&value[..n - 2], 59, 76)
        } else if value.ends_with("cm") {
            check_int(&value[..n - 2], 150, 193)
        } else {
            false
        }
    }

    fn check_regex(value: &str, regex: &str) -> bool {
        Regex::new(regex).unwrap().is_match(value)
    }

    fn check_eyes(value: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
    }

    has_fields(&passport)
        && check_int(passport["byr"], 1920, 2002)
        && check_int(passport["iyr"], 2010, 2020)
        && check_int(passport["eyr"], 2020, 2030)
        && check_height(passport["hgt"])
        && check_regex(passport["hcl"], "^#[0-9a-f]{6}$")
        && check_eyes(passport["ecl"])
        && check_regex(passport["pid"], "^[0-9]{9}$")
}

pub fn run() -> Result {
    let content = &read_input("day04")?.join("\n");
    let passports = parse_input(&content);

    let count = passports.iter().filter(|p| has_fields(p)).count();
    println!("part A: {}", count);

    let count = passports.iter().filter(|p| is_valid_passport(p)).count();
    println!("part B: {}", count);

    Ok(())
}
