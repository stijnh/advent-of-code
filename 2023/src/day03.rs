use crate::common::*;

struct Schematic {
    parts: HashMap<[i64; 2], char>,
    part_index: HashMap<[i64; 2], usize>,
    part_numbers: Vec<u64>,
}

fn parse_schematic(lines: Lines) -> Schematic {
    let mut parts = HashMap::<[i64; 2], char>::default();
    let mut part_index = HashMap::<[i64; 2], usize>::default();
    let mut part_numbers = vec![];

    for (y, line) in enumerate(lines) {
        let mut chars = line.chars().enumerate().peekable();

        while let Some((x, c)) = chars.next() {
            if let Some(mut number) = c.to_digit(10) {
                let mut n = 1;

                while let Some(digit) = chars.peek().and_then(|(_, c)| c.to_digit(10)) {
                    number = number * 10 + digit;
                    chars.next();
                    n += 1;
                }

                let index = part_numbers.len();
                part_numbers.push(number as u64);

                for dx in 0..n {
                    part_index.insert([x as i64 + dx, y as i64], index);
                }
            } else if c != '.' {
                parts.insert([x as i64, y as i64], c);
            }
        }
    }

    Schematic {
        parts,
        part_index,
        part_numbers,
    }
}

const ADJACENT: [[i64; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

pub(crate) fn run(lines: Lines) -> Result {
    let schematic = parse_schematic(lines);
    let mut visited = HashSet::default();

    for [x, y] in schematic.parts.keys() {
        for [dx, dy] in ADJACENT {
            if let Some(index) = schematic.part_index.get(&[x + dx, y + dy]) {
                visited.insert(index);
            }
        }
    }

    println!(
        "part A: {}",
        visited
            .iter()
            .map(|&&index| schematic.part_numbers[index])
            .sum::<u64>()
    );

    let sum = schematic
        .parts
        .iter()
        .filter(|(_, &symbol)| symbol == '*')
        .filter_map(|([x, y], _)| {
            let mut labels = vec![];
            for [dx, dy] in ADJACENT {
                if let Some(&index) = schematic.part_index.get(&[x + dx, y + dy]) {
                    labels.push(index);
                }
            }

            labels.sort();
            labels.dedup();

            (labels.len() == 2).then(|| (labels[0], labels[1]))
        })
        .map(|(i, j)| schematic.part_numbers[i] * schematic.part_numbers[j])
        .sum::<u64>();

    println!("part B: {sum}");

    Ok(())
}
