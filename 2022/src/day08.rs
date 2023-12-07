use crate::common::*;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};

fn parse_grid(lines: Lines) -> Array2<i32> {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut grid = Array2::from_elem((rows, cols), 0);

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            grid[[i, j]] = c.to_digit(10).unwrap() as i32;
        }
    }

    grid
}

fn scan_visible(grid: ArrayView2<i32>, mut visible: ArrayViewMut2<bool>) {
    for i in 0..grid.len_of(Axis(0)) {
        let mut highest = -1;

        for j in 0..grid.len_of(Axis(1)) {
            if grid[[i, j]] > highest {
                visible[[i, j]] = true;
                highest = grid[[i, j]];
            }
        }

        let mut highest = -1;

        for j in (0..grid.len_of(Axis(1))).rev() {
            if grid[[i, j]] > highest {
                visible[[i, j]] = true;
                highest = grid[[i, j]];
            }
        }
    }
}

fn score_spot(grid: ArrayView2<i32>, [i, j]: [usize; 2]) -> usize {
    let (rows, cols) = grid.dim();
    let height = grid[[i, j]];

    let (mut a, mut b, mut c, mut d) = (1, 1, 1, 1);

    // down
    while i + a + 1 < rows && grid[[i + a, j]] < height {
        a += 1;
    }

    // up
    while i >= b + 1 && grid[[i - b, j]] < height {
        b += 1;
    }

    // right
    while j + c + 1 < cols && grid[[i, j + c]] < height {
        c += 1;
    }

    //left
    while j >= d + 1 && grid[[i, j - d]] < height {
        d += 1;
    }

    a * b * c * d
}

pub(crate) fn run(lines: Lines) -> Result {
    let grid = parse_grid(lines);
    let mut visible = Array2::from_elem(grid.dim(), false);

    scan_visible(grid.view(), visible.view_mut());
    scan_visible(
        grid.view().reversed_axes(),
        visible.view_mut().reversed_axes(),
    );

    let n = visible.iter().filter(|&&e| e).count();
    println!("part A: {}", n);

    let best = grid
        .indexed_iter()
        .map(|((i, j), _)| score_spot(grid.view(), [i, j]))
        .max();

    println!("part B: {:?}", best);

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
        let grid = parse_grid(&["30373", "25512", "65332", "33549", "35390"]);

        assert_eq!(score_spot(grid.view(), [1, 2]), 5);
    }
}
