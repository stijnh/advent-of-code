use crate::common::*;
use ndarray::{Array2, ArrayView2};

fn parse_field(lines: Lines) -> Array2<char> {
    let nrows = lines.len();
    let ncols = lines[0].chars().count();
    let mut grid = Array2::from_elem((ncols, nrows), '.');

    for (y, line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            grid[[x, y]] = c;
        }
    }

    grid
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn neighbor([x, y]: [usize; 2], dir: Direction, (nx, ny): (usize, usize)) -> Option<[usize; 2]> {
    use Direction::*;

    match dir {
        North if y >= 1 => Some([x, y - 1]),
        South if y + 1 < ny => Some([x, y + 1]),
        East if x + 1 < nx => Some([x + 1, y]),
        West if x >= 1 => Some([x - 1, y]),
        _ => None,
    }
}

fn shift(grid: ArrayView2<char>, direction: Direction) -> Array2<char> {
    let mut grid = grid.into_owned();
    let (ncols, nrows) = grid.dim();

    loop {
        let mut updated = false;

        for x in 0..ncols {
            for y in 0..nrows {
                let p = [x, y];
                if let Some(q) = neighbor(p, direction, grid.dim()) {
                    if grid[p] == 'O' && grid[q] == '.' {
                        grid[q] = 'O';
                        grid[p] = '.';
                        updated = true;
                    }
                }
            }
        }

        if !updated {
            return grid;
        }
    }
}

fn calculate_load(grid: ArrayView2<char>) -> usize {
    let (ncols, nrows) = grid.dim();
    let mut total = 0;

    for y in 0..nrows {
        for x in 0..ncols {
            if grid[[x, y]] == 'O' {
                total += nrows - y;
            }
        }
    }
    total
}

fn simulate_cycles(grid: ArrayView2<char>, ncycles: usize) -> Array2<char> {
    let mut seen = vec![];
    let mut grid = grid.to_owned();

    for cycle in 1..=ncycles {
        seen.push(grid.clone());
        grid = shift(grid.view(), Direction::North);
        grid = shift(grid.view(), Direction::West);
        grid = shift(grid.view(), Direction::South);
        grid = shift(grid.view(), Direction::East);

        if let Some(cycle_start) = seen.iter().position(|p| p == grid) {
            let cycle_len = cycle - cycle_start;
            return seen.swap_remove(cycle_start + (ncycles - cycle_start) % cycle_len);
        }
    }

    grid
}

// high: 85203

pub(crate) fn run(lines: Lines) -> Result {
    let grid = parse_field(lines);
    let grid_shifted = shift(grid.view(), Direction::North);

    println!("part A: {}", calculate_load(grid_shifted.view()));

    let result = simulate_cycles(grid.view(), 1000000000);
    println!("part B: {}", calculate_load(result.view()));

    //89554
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
