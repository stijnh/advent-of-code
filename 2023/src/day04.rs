use crate::common::*;

struct Card {
    winning: Vec<u32>,
    scratched: Vec<u32>,
}

impl Card {
    fn calculate_points(&self) -> u32 {
        match self.calculate_matches() {
            0 => 0,
            x => u32::pow(2, x - 1),
        }
    }

    fn calculate_matches(&self) -> u32 {
        self.scratched
            .iter()
            .filter(|x| self.winning.contains(&x))
            .count() as u32
    }
}

fn parse_card(line: &str) -> Card {
    let (_, line) = line.split_once(": ").unwrap();
    let (left, right) = line.split_once(" | ").unwrap();

    Card {
        winning: parse_list::<u32>(left, ' ').unwrap(),
        scratched: parse_list::<u32>(right, ' ').unwrap(),
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let sum: u32 = lines
        .iter()
        .map(|line| parse_card(line).calculate_points())
        .sum();

    println!("part A: {sum}");

    let mut count = vec![1; lines.len()];
    for (index, line) in enumerate(lines) {
        let matches = parse_card(line).calculate_matches() as usize;
        let n = count[index];

        for v in &mut count[index..][1..=matches] {
            *v += n;
        }
    }

    println!("part B: {}", count.iter().sum::<u64>());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").calculate_points(),
            8
        );
        assert_eq!(
            parse_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").calculate_points(),
            2
        );
        assert_eq!(
            parse_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").calculate_points(),
            2
        );
        assert_eq!(
            parse_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").calculate_points(),
            1
        );
        assert_eq!(
            parse_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").calculate_points(),
            0
        );
        assert_eq!(
            parse_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").calculate_points(),
            0
        );
    }
}
