use crate::common::*;
use std::collections::HashSet;

fn parse_input(lines: &[String]) -> Vec<Vec<HashSet<char>>> {
    lines
        .iter()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter(|(empty, _)| !empty)
        .map(|(_, lines)| lines.collect())
        .collect()
}

fn union(answers: &[HashSet<char>]) -> HashSet<char> {
    answers
        .iter()
        .fold(answers[0].clone(), |a, b| a.union(b).copied().collect())
}

fn intersection(answers: &[HashSet<char>]) -> HashSet<char> {
    answers.iter().fold(answers[0].clone(), |a, b| {
        a.intersection(b).copied().collect()
    })
}

pub fn run() -> Result {
    let groups = parse_input(&read_input("day06")?);

    let sum: usize = groups.iter().map(|answers| union(answers).len()).sum();

    println!("part A: {}", sum);

    let sum: usize = groups
        .iter()
        .map(|answers| intersection(answers).len())
        .sum();

    println!("part B: {}", sum);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = [
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();

        let groups = parse_input(&input);

        assert_eq!(groups.len(), 5);
        assert_eq!(union(&groups[0]).len(), 3);
        assert_eq!(union(&groups[1]).len(), 3);
        assert_eq!(union(&groups[2]).len(), 3);
        assert_eq!(union(&groups[3]).len(), 1);
        assert_eq!(union(&groups[4]).len(), 1);

        assert_eq!(intersection(&groups[0]).len(), 3);
        assert_eq!(intersection(&groups[1]).len(), 0);
        assert_eq!(intersection(&groups[2]).len(), 1);
        assert_eq!(intersection(&groups[3]).len(), 1);
        assert_eq!(intersection(&groups[4]).len(), 1);
    }
}
