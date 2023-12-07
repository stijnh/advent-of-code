use crate::common::*;

type Item = i32;
const ROCK: Item = 0;
const PAPER: Item = 1;
const SCISSORS: Item = 2;

fn score(first: Item, second: Item) -> i32 {
    let additional = if second == (first + 1) % 3 {
        6
    } else if first == second {
        3
    } else {
        0
    };

    second + 1 + additional
}

fn should_play(first: Item, goal: Item) -> Item {
    (first + goal + 2) % 3
}

fn parse_line(line: &str) -> Result<(Item, Item)> {
    let mut chars = line.chars();
    let a = match chars.next().unwrap_or_default() {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => bail!("invalid line"),
    };

    chars.next();

    let b = match chars.next().unwrap_or_default() {
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCISSORS,
        _ => bail!("invalid line"),
    };

    Ok((a, b))
}

pub(crate) fn run(lines: Lines) -> Result {
    let pairs = lines
        .iter()
        .map(|&e| parse_line(e))
        .collect::<Result<Vec<_>>>()?;

    let total_score = pairs.iter().map(|&(a, b)| score(a, b)).sum::<i32>();
    println!("part A: {}", total_score);

    let total_score = pairs
        .iter()
        .map(|&(a, b)| score(a, should_play(a, b)))
        .sum::<i32>();
    println!("part B: {}", total_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let text = "A Y\nB X\nC Z";
        let pairs = text
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let (a, b) = pairs[0];
        assert_eq!(score(a, b), 8);

        let (a, b) = pairs[1];
        assert_eq!(score(a, b), 1);

        let (a, b) = pairs[2];
        assert_eq!(score(a, b), 6);
    }
}
