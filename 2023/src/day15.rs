use crate::common::*;
use std::ops::Index;

fn hash(input: &str) -> u64 {
    let mut hash = 0;

    for c in input.chars() {
        hash += (c as u8) as u64;
        hash *= 17;
        hash = hash % 256;
    }

    hash
}

fn simulate_boxed<'a>(inputs: &[&'a str]) -> [Vec<(&'a str, u8)>; 256] {
    let mut boxes = [(); 256].map(|_| Vec::<(&str, u8)>::new());

    for input in inputs {
        if let Some(m) = find_regex("([a-z]+)=([0-9]+)", input) {
            let label = m.get(1).unwrap().as_str();
            let id = hash(label) as u8 as usize;
            let focal: u8 = m[2].parse().unwrap();

            if let Some(index) = boxes[id].iter().position(|&(l, _)| l == label) {
                boxes[id][index] = (label, focal);
            } else {
                boxes[id].push((label, focal));
            }
        } else if let Some(m) = find_regex("([a-z]+)[-]", input) {
            let id = hash(&m[1]) as u8 as usize;
            boxes[id].retain(|&(label, _)| label != &m[1]);
        } else {
            panic!("invalid input: {input}");
        }
    }

    boxes
}

fn score_boxes(boxes: &[Vec<(&str, u8)>]) -> u64 {
    let mut score = 0u64;

    for (i, b) in enumerate(boxes) {
        for (j, &(_, f)) in enumerate(b) {
            score += (i as u64 + 1) * (j as u64 + 1) * (f as u64);
        }
    }

    score
}

pub(crate) fn run(lines: Lines) -> Result {
    let codes = lines[0].split(',').collect_vec();

    let result: u64 = codes.iter().map(|s| hash(s)).sum();
    println!("part A: {}", result);

    let score = score_boxes(&simulate_boxed(&codes));
    println!("part B: {score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_b() {
        //
    }
}
