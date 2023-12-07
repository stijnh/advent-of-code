use crate::common::*;
use std::collections::VecDeque;

fn parse_input(lines: &[String]) -> Result<(Vec<usize>, Vec<usize>)> {
    let mut iter = lines.iter().map(|s| &**s);
    let mut player1 = vec![];
    let mut player2 = vec![];

    if iter.next() != Some("Player 1:") {
        bail!("invalid input");
    }

    while let Ok(i) = iter.next().unwrap_or_default().parse() {
        player1.push(i);
    }

    if iter.next() != Some("Player 2:") {
        bail!("invalid input");
    }

    while let Ok(i) = iter.next().unwrap_or_default().parse() {
        player2.push(i);
    }

    Ok((player1, player2))
}

fn score(cards: &[usize]) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

fn play_game(cards1: &[usize], cards2: &[usize], recursive: bool) -> (usize, Vec<usize>) {
    let mut seen = HashSet::default();
    let mut cards1 = VecDeque::from(cards1.to_vec());
    let mut cards2 = VecDeque::from(cards2.to_vec());

    loop {
        if !seen.insert((cards1.clone(), cards2.clone())) {
            return (0, cards1.into());
        }

        let (a, b) = match (cards1.pop_front(), cards2.pop_front()) {
            (Some(a), Some(b)) => (a, b),
            (Some(a), None) => {
                cards1.push_front(a);
                return (0, cards1.into());
            }
            (None, Some(b)) => {
                cards2.push_front(b);
                return (1, cards2.into());
            }
            _ => panic!("both empty?"),
        };

        let winner = if recursive && cards1.len() >= a && cards2.len() >= b {
            // Unfortunately make_contiguous is broken in 1.48, copy the data instead :(
            //cards1.make_contiguous();
            //cards2.make_contiguous();

            let c1 = Vec::from(cards1.clone());
            let c2 = Vec::from(cards2.clone());

            play_game(&c1[..a], &c2[..b], true).0
        } else if a > b {
            0
        } else {
            1
        };

        if winner == 0 {
            cards1.push_back(a);
            cards1.push_back(b);
        } else {
            cards2.push_back(b);
            cards2.push_back(a);
        }
    }
}

pub fn run() -> Result {
    let (cards1, cards2) = parse_input(&read_input("day22")?)?;

    let (_winner, cards) = play_game(&cards1, &cards2, false);
    println!("part A: {}", score(&cards));

    let (_winner, cards) = play_game(&cards1, &cards2, true);
    println!("part B: {}", score(&cards));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_play_game() {
        let a = [9, 2, 6, 3, 1];
        let b = [5, 8, 4, 7, 10];

        let (winner, cards) = play_game(&a, &b, false);
        assert_eq!(winner, 1);
        assert_eq!(&cards, &[3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        assert_eq!(score(&cards), 306);

        let (winner, cards) = play_game(&a, &b, true);
        assert_eq!(winner, 1);
        assert_eq!(&cards, &[7, 5, 6, 2, 4, 1, 10, 8, 9, 3]);
        assert_eq!(score(&cards), 291);
    }
}
