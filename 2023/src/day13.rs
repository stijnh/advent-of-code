use crate::common::*;
use ndarray::{Array2, ArrayView2};

fn parse_maps(lines: Lines) -> Vec<Array2<char>> {
    let mut maps = vec![];
    let mut index = 0;

    while index < lines.len() {
        let height = lines[index..]
            .iter()
            .position(|l| l.is_empty())
            .unwrap_or(lines.len() - index);
        let width = lines[index].chars().count();
        let mut map = Array2::<char>::from_elem((width, height), ' ');

        for (y, line) in enumerate(&lines[index..][..height]) {
            for (x, c) in enumerate(line.chars()) {
                map[[x, y]] = c;
            }
        }

        maps.push(map);
        index += height + 1;
    }

    maps
}

fn find_vertical_reflection(map: ArrayView2<char>, smudges: usize) -> Option<usize> {
    let (width, height) = map.dim();

    for x in 1..width {
        let mut num_mismatches = 0;

        for dx in 0..usize::min(width - x, x) {
            for y in 0..height {
                if map[[x + dx, y]] != map[[x - dx - 1, y]] {
                    num_mismatches += 1;
                }
            }
        }

        if num_mismatches == smudges {
            return Some(x);
        }
    }

    None
}

fn find_reflection(map: ArrayView2<char>, smudges: usize) -> Option<usize> {
    if let Some(r) = find_vertical_reflection(map, smudges) {
        Some(r)
    } else if let Some(r) = find_vertical_reflection(map.t(), smudges) {
        Some(100 * r)
    } else {
        None
    }
}

fn summarize_maps(maps: &[Array2<char>], smudges: usize) -> usize {
    maps.iter()
        .map(|m| find_reflection(m.view(), smudges).unwrap())
        .sum()
}

pub(crate) fn run(lines: Lines) -> Result {
    let maps = parse_maps(lines);

    println!("part A: {}", summarize_maps(&maps, 0));
    println!("part A: {}", summarize_maps(&maps, 1));
    Ok(())
}
