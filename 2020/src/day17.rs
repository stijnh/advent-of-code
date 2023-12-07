use crate::common::*;
use itertools::iproduct;

pub type Pos = [i32; 4];

fn parse_input(lines: &[String]) -> HashSet<Pos> {
    let mut result = HashSet::default();

    for (y, line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            if c == '#' {
                result.insert([x as i32, y as i32, 0, 0]);
            }
        }
    }

    result
}

fn simulate(pos: &HashSet<Pos>, dims: usize) -> HashSet<Pos> {
    let mut count = HashMap::<Pos, usize>::default();
    let neighbors: Vec<_> = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|&d| d != (0, 0, 0, 0))
        .filter(|&d| dims > 3 || d.3 == 0)
        .collect();

    for [x, y, z, w] in pos {
        for &(dx, dy, dz, dw) in &neighbors {
            let p = [x + dx, y + dy, z + dz, w + dw];
            *count.entry(p).or_default() += 1;
        }
    }

    count
        .into_iter()
        .filter(|&(p, c)| c == 3 || (c == 2 && pos.contains(&p)))
        .map(|(p, _)| p)
        .collect()
}

pub fn run() -> Result {
    let pos = parse_input(&read_input("day17")?);

    let mut new_pos = pos.clone();
    for _ in 0..6 {
        new_pos = simulate(&new_pos, 3);
    }

    println!("part A: {}", new_pos.len());

    let mut new_pos = pos;
    for _ in 0..6 {
        new_pos = simulate(&new_pos, 4);
    }

    println!("part B: {}", new_pos.len());

    Ok(())
}
