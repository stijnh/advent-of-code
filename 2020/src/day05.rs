use crate::common::*;
use std::collections::BTreeSet;

fn boarding_pass_to_number(string: &str) -> usize {
    string
        .chars()
        .fold(0, |v, c| v * 2 + "RB".contains(c) as usize)
}

pub fn run() -> Result {
    let passes = read_input("day05")?
        .iter()
        .map(|s| boarding_pass_to_number(s))
        .collect::<BTreeSet<_>>();

    let &lowest = passes.iter().min().unwrap();
    let &highest = passes.iter().max().unwrap();
    println!("part A: {}", highest);

    let missing = (lowest..=highest)
        .filter(|i| !passes.contains(&i))
        .next()
        .unwrap();

    println!("part B: {}", missing);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(boarding_pass_to_number("BFFFBBFRRR"), 567);
        assert_eq!(boarding_pass_to_number("FFFBBBFRRR"), 119);
        assert_eq!(boarding_pass_to_number("BBFFBBFRLL"), 820);
    }
}
