use crate::common::*;
use std::collections::HashSet;

fn priority(c: char) -> i32 {
    match c {
        'a'..='z' => 1 + c as i32 - 'a' as i32,
        'A'..='Z' => 27 + c as i32 - 'A' as i32,
        _ => panic!("invalid item"),
    }
}

fn find_common_item(line: &str) -> Result<char> {
    let mid = line.len() / 2;
    let first = line[..mid].chars().collect::<HashSet<_>>();
    let second = line[mid..].chars().collect::<HashSet<_>>();
    let common = &first & &second;
    common
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("no items in common"))
}

fn find_common_badge(lines: &[&str]) -> Result<char> {
    let mut common = lines[0].chars().collect::<HashSet<_>>();

    for line in &lines[1..] {
        common = &common & &line.chars().collect::<HashSet<_>>()
    }

    common
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("no items in common"))
}

pub(crate) fn run(lines: Lines) -> Result {
    let total = lines
        .iter()
        .map(|line| find_common_item(line))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(priority)
        .sum::<i32>();

    println!("part A: {}", total);

    let total = lines
        .chunks(3)
        .map(|chunk| find_common_badge(chunk))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(priority)
        .sum::<i32>();

    println!("part B: {}", total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        //
    }

    #[test]
    fn test_b() {
        //
    }
}
