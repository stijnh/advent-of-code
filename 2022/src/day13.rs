use crate::common::*;
use itertools::cloned;
use std::cmp::Ordering;
use std::fmt;
use std::iter::zip;
use std::mem::take;

#[derive(Clone, Eq, PartialEq)]
enum Item {
    Int(i64),
    List(Vec<Item>),
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Int(v) => fmt::Debug::fmt(v, f),
            Item::List(l) => fmt::Debug::fmt(l, f),
        }
    }
}

impl Ord for Item {
    fn cmp(self: &Item, right: &Item) -> Ordering {
        use Item::*;

        match (self, right) {
            (List(l), List(r)) => {
                for (a, b) in zip(l, r) {
                    match cmp(a, b) {
                        Ordering::Equal => {}
                        other => return other,
                    }
                }

                cmp(&l.len(), &r.len())
            }
            (Int(l), Int(r)) => cmp(l, r),
            (List(_), r) => cmp(self, &List(vec![r.clone()])),
            (l, List(_)) => cmp(&List(vec![l.clone()]), right),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_item(line: &str) -> Item {
    let mut stack: Vec<Vec<Item>> = default();
    let mut current: Vec<Item> = default();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' {
            stack.push(take(&mut current));
        } else if c.is_digit(10) {
            let mut i = c.to_digit(10).unwrap() as i64;
            while let Some(c) = chars.next_if(|c| c.is_digit(10)) {
                i = (i * 10) + c.to_digit(10).unwrap() as i64;
            }

            current.push(Item::Int(i));
        } else if c == ']' {
            let mut last = stack.pop().unwrap();
            last.push(Item::List(current));
            current = last;
        }
    }

    Item::List(current)
}

pub(crate) fn run(lines: Lines) -> Result {
    let mut count = 0;

    for i in (0..lines.len()).step_by(3) {
        let left = parse_item(lines[i]);
        let right = parse_item(lines[i + 1]);

        if left < right {
            count += i / 3 + 1;
        }
    }

    println!("part A: {:?}", count);

    let mut packets = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| parse_item(line))
        .collect_vec();

    let decoders = [parse_item("[[2]]"), parse_item("[[6]]")];
    packets.extend(cloned(&decoders));
    packets.sort();

    let a = packets.iter().position(|v| v == &decoders[0]).unwrap() + 1;
    let b = packets.iter().position(|v| v == &decoders[1]).unwrap() + 1;
    println!("part B: {:?}", a * b);

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
