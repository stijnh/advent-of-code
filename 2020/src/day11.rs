use crate::common::*;
use itertools::max;
use ndarray::prelude::*;

const DIRECTIONS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
];

fn parse_input(lines: &[String]) -> Array2<char> {
    let rows = lines.len();
    let cols = max(map(lines, |l| l.len())).unwrap_or(0);
    let mut grid = Array2::default((rows, cols));

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            grid[[i, j]] = c;
        }
    }

    grid
}

fn neighbors(dims: [usize; 2], ij: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    DIRECTIONS.iter().filter_map(move |&[di, dj]| {
        let [i, j] = ij;
        let [rows, cols] = dims;
        let [ni, nj] = [i as isize + di, j as isize + dj];

        if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
            return Some([ni as usize, nj as usize]);
        }

        None
    })
}

fn apply_round(old: &Array2<char>) -> Array2<char> {
    let (rows, cols) = old.dim();
    let mut new = Array2::default((rows, cols));

    for i in 0..rows {
        for j in 0..cols {
            let occupied = neighbors([rows, cols], [i, j])
                .filter(|&a| old[a] == '#')
                .count();

            new[[i, j]] = match old[[i, j]] {
                'L' if occupied == 0 => '#',
                '#' if occupied >= 4 => 'L',
                c => c,
            }
        }
    }

    new
}

fn far_neighbors(array: &Array2<char>, ij: [usize; 2]) -> impl Iterator<Item = [usize; 2]> + '_ {
    DIRECTIONS.iter().filter_map(move |&[di, dj]| {
        let [i, j] = ij;
        let (rows, cols) = array.dim();
        for delta in 1.. {
            let [ni, nj] = [i as isize + di * delta, j as isize + dj * delta];

            if ni < 0 || ni >= rows as isize || nj < 0 || nj >= cols as isize {
                break;
            }

            if array[[ni as usize, nj as usize]] != '.' {
                return Some([ni as usize, nj as usize]);
            }
        }

        None
    })
}

fn apply_far_round(old: &Array2<char>) -> Array2<char> {
    let (rows, cols) = old.dim();
    let mut new = Array2::default((rows, cols));

    for i in 0..rows {
        for j in 0..cols {
            let occupied = far_neighbors(old, [i, j])
                .filter(|&a| old[a] == '#')
                .count();

            new[[i, j]] = match old[[i, j]] {
                'L' if occupied == 0 => '#',
                '#' if occupied >= 5 => 'L',
                c => c,
            }
        }
    }

    new
}

fn count_occupied(grid: &Array2<char>) -> usize {
    map(grid, |&c| (c == '#') as usize).sum()
}

fn repeat_until_convergence<T: Eq, F: Fn(&T) -> T>(val: &T, fun: F) -> T {
    let mut new_val = (fun)(&val);

    loop {
        let old_val = new_val;
        new_val = (fun)(&old_val);

        if old_val == new_val {
            break old_val;
        }
    }
}

pub fn run() -> Result {
    let grid = parse_input(&read_input("day11")?);

    let new_grid = repeat_until_convergence(&grid, apply_round);
    let count = count_occupied(&new_grid);
    println!("part A: {}", count);

    let new_grid = repeat_until_convergence(&grid, apply_far_round);
    let count = count_occupied(&new_grid);
    println!("part B: {}", count);

    Ok(())
}
