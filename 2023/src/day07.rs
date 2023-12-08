use crate::common::*;
use std::array;

type Hand = [char; 5];
const SYMBOLS: &str = "AKQJT98765432";

fn parse_hand(line: &str) -> (Hand, i64) {
    let (hand, bid) = line.split_once(' ').unwrap();

    let hand = hand.chars().collect_vec().try_into().unwrap();

    let bid = bid.parse().unwrap();
    (hand, bid)
}

fn strength(hand: Hand) -> [usize; 5] {
    array::from_fn(|i| SYMBOLS.chars().position(|c| c == hand[i]).unwrap())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn kind(mut hand: Hand) -> Kind {
    let mut counts = [0; 6];

    hand.sort();
    let mut index = 0;
    while let Some(&card) = hand.get(index) {
        let mut n = 1;
        while index + n < hand.len() && card == hand[index + n] {
            n += 1;
        }

        counts[n] += 1;
        index += n;
    }

    if counts[5] == 1 {
        Kind::FiveOfKind
    } else if counts[4] == 1 {
        Kind::FourOfKind
    } else if counts[3] == 1 && counts[2] == 1 {
        Kind::FullHouse
    } else if counts[3] == 1 {
        Kind::ThreeOfKind
    } else if counts[2] == 2 {
        Kind::TwoPair
    } else if counts[2] == 1 {
        Kind::OnePair
    } else if counts[1] == 5 {
        Kind::HighCard
    } else {
        panic!("invalid kind: {:?}", counts);
    }
}

fn kind_with_joker(hand: Hand) -> Kind {
    SYMBOLS
        .chars()
        .map(|c| hand.map(|x| if x == 'J' { c } else { x }))
        .map(|hand| kind(hand))
        .min()
        .unwrap()
}

fn strength_with_joker(hand: Hand) -> [usize; 5] {
    array::from_fn(|i| {
        if hand[i] == 'J' {
            usize::MAX
        } else {
            SYMBOLS.chars().position(|c| c as char == hand[i]).unwrap()
        }
    })
}

fn calculate_winnings(hands: &[(Hand, i64)]) -> i64 {
    hands
        .iter()
        .enumerate()
        .map(|(index, &(_, bid))| (index + 1) as i64 * bid)
        .sum()
}

pub(crate) fn run(lines: Lines) -> Result {
    let mut hands = lines.iter().map(|line| parse_hand(line)).collect_vec();

    hands.sort_by_key(|&(hand, _)| (kind(hand), strength(hand)));
    hands.reverse();

    let total = calculate_winnings(&hands);
    println!("part A: {}", total);

    hands.sort_by_key(|&(hand, _)| (kind_with_joker(hand), strength_with_joker(hand)));
    hands.reverse();

    let total = calculate_winnings(&hands);
    println!("part B: {}", total);

    Ok(())
}
