use crate::common::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
struct Password {
    lowest: usize,
    highest: usize,
    letter: char,
    password: String,
}

fn parse_line(line: &str) -> Result<Password> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    }

    let matches = PATTERN.captures(line).ok_or_else(|| {
        anyhow!(
            "line {:?} does not match pattern [0-9]-[0-9] [a-z]: [a-z]+",
            line
        )
    })?;

    Ok(Password {
        lowest: matches[1].parse().unwrap(),
        highest: matches[2].parse().unwrap(),
        letter: matches[3].chars().next().unwrap(),
        password: matches[4].to_string(),
    })
}

fn is_valid_sled_rental_place(w: &Password) -> bool {
    let count = w.password.chars().filter(|&c| c == w.letter).count();

    count >= w.lowest && count <= w.highest
}

fn is_valid_official_toboggan_corporate(w: &Password) -> bool {
    let a = w.password.chars().nth(w.lowest - 1).unwrap() == w.letter;
    let b = w.password.chars().nth(w.highest - 1).unwrap() == w.letter;

    a ^ b
}

pub fn run() -> Result {
    let lines = read_input("day02")?
        .iter()
        .map(|s| parse_line(s))
        .collect::<Result<Vec<_>>>()?;

    let count_valid = lines
        .iter()
        .filter(|p| is_valid_sled_rental_place(p))
        .count();

    println!("part A: {}", count_valid);

    let count_valid = lines
        .iter()
        .filter(|p| is_valid_official_toboggan_corporate(p))
        .count();

    println!("part B: {}", count_valid);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        let lines = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

        let pw = vec![
            Password {
                lowest: 1,
                highest: 3,
                letter: 'a',
                password: "abcde".to_string(),
            },
            Password {
                lowest: 1,
                highest: 3,
                letter: 'b',
                password: "cdefg".to_string(),
            },
            Password {
                lowest: 2,
                highest: 9,
                letter: 'c',
                password: "ccccccccc".to_string(),
            },
        ];

        let results = vec![
            parse_line(lines[0]).unwrap(),
            parse_line(lines[1]).unwrap(),
            parse_line(lines[2]).unwrap(),
        ];

        assert_eq!(pw[0], results[0]);
        assert_eq!(pw[1], results[1]);
        assert_eq!(pw[2], results[2]);

        assert_eq!(is_valid_sled_rental_place(&results[0]), true);
        assert_eq!(is_valid_sled_rental_place(&results[1]), false);
        assert_eq!(is_valid_sled_rental_place(&results[2]), true);

        assert_eq!(is_valid_official_toboggan_corporate(&results[0]), true);
        assert_eq!(is_valid_official_toboggan_corporate(&results[1]), false);
        assert_eq!(is_valid_official_toboggan_corporate(&results[2]), false);
    }
}
