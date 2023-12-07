use crate::common::*;
use ndarray::prelude::*;

fn parse_input(lines: Vec<String>) -> Result<Array2<char>> {
    let height = lines.len();
    let width = lines[0].len();

    if !all(&lines, |v| v.chars().count() == width) {
        return Err(anyhow!("not all lines have equal length"));
    }

    let mut result = Array2::default((height, width));

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            result[[i, j]] = c;
        }
    }

    Ok(result)
}

fn count_trees(matrix: ArrayView2<char>, shift_per_row: usize, shift_per_col: usize) -> usize {
    let mut x = 0;
    let (height, width) = matrix.dim();
    let mut trees = 0;

    for y in (0..height).step_by(shift_per_col) {
        if matrix[[y, x]] == '#' {
            trees += 1;
        }

        x = (x + shift_per_row) % width;
    }

    trees
}

pub fn run() -> Result {
    let matrix = parse_input(read_input("day03")?)?;

    let trees = count_trees(matrix.view(), 3, 1);
    println!("part A: {}", trees);

    let total: usize = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&shift| count_trees(matrix.view(), shift.0, shift.1))
        .product();
    println!("part B: {:?}", total);

    Ok(())
}
