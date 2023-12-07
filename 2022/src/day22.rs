use crate::common::*;
use ndarray::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Step {
    TurnLeft,
    TurnRight,
    Forward,
}

fn parse_steps(line: &str) -> Vec<Step> {
    let mut output = vec![];
    let mut n = 0;

    for c in line.chars() {
        if let Some(i) = c.to_digit(10) {
            n = n * 10 + i;
        } else {
            while n > 0 {
                output.push(Step::Forward);
                n -= 1;
            }

            match c {
                'L' => output.push(Step::TurnLeft),
                'R' => output.push(Step::TurnRight),
                _ => panic!("invalid character: {:?}", c),
            }
        }
    }

    while n > 0 {
        output.push(Step::Forward);
        n -= 1;
    }

    output
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Open,
}

fn parse_map(lines: &[&str]) -> Array2<Tile> {
    let height = lines.len();
    let width = lines.iter().map(|line| line.chars().count()).max().unwrap();

    let mut grid = Array2::from_elem((height, width), Tile::Empty);

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            grid[[i, j]] = match c {
                ' ' => Tile::Empty,
                '.' => Tile::Open,
                '#' => Tile::Wall,
                c => panic!("invalid character: {:?}", c),
            };
        }
    }

    grid
}

fn walk_map(map: ArrayView2<Tile>, steps: &[Step]) -> (usize, usize, usize) {
    let (height, width) = map.dim();
    let mut i = 1;
    let mut j = map.row(i).iter().position(|&t| t == Tile::Open).unwrap();
    let mut dir = 0;

    for &step in steps {
        if step != Step::Forward {
            dir = match step {
                Step::TurnLeft => (dir + 3) % 4,
                Step::TurnRight => (dir + 1) % 4,
                _ => unreachable!(),
            };

            continue;
        }

        let [di, dj] = match dir {
            0 => [0, 1],
            1 => [1, 0],
            2 => [0, width - 1],
            3 => [height - 1, 0],
            _ => unreachable!(),
        };

        let mut ni = i;
        let mut nj = j;
        let is_valid = loop {
            ni = (ni + di) % height;
            nj = (nj + dj) % width;

            match map[[ni, nj]] {
                Tile::Empty => continue,
                Tile::Wall => break false,
                Tile::Open => break true,
            }
        };

        if is_valid {
            (i, j) = (ni, nj);
        }
    }

    (i, j, dir)
}

pub(crate) fn run(lines: Lines) -> Result {
    let n = lines.len();
    let map = parse_map(&lines[..n - 1]);
    let steps = parse_steps(lines[n - 1]);

    let (i, j, dir) = walk_map(map.view(), &steps);
    println!("part A: {:?}", 1000 * i + 4 * j + dir);

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
