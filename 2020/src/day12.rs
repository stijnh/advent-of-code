use crate::common::*;

use recap::Recap;
use serde::Deserialize;

#[derive(Recap, Deserialize)]
#[recap(regex = "(?P<c>.)(?P<v>[0-9]+)")]
struct Instr {
    c: char,
    v: i32,
}

fn parse_input(lines: &[String]) -> Result<Vec<Instr>> {
    lines.iter().map(|line| Ok(line.parse()?)).collect()
}

fn rotate((x, y): (i32, i32), angle: i32) -> (i32, i32) {
    match i32::rem_euclid(angle, 360) {
        0 => (x, y),
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => panic!("invalid angle"),
    }
}

fn execute(instr: &[Instr]) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    let mut dir = (1, 0);

    for &Instr { c, v } in instr {
        match c {
            'N' => y += v,
            'S' => y -= v,
            'E' => x += v,
            'W' => x -= v,
            'R' => dir = rotate(dir, v),
            'L' => dir = rotate(dir, -v),
            'F' => {
                x += dir.0 * v;
                y += dir.1 * v;
            }
            _ => println!("unknown command: {}", c),
        }
    }

    (x, y)
}

fn execute_real(instr: &[Instr]) -> (i32, i32) {
    let (mut x, mut y) = (0, 0);
    let mut wp = (10, 1);

    for &Instr { c, v } in instr {
        match c {
            'E' => wp.0 += v,
            'W' => wp.0 -= v,
            'N' => wp.1 += v,
            'S' => wp.1 -= v,
            'R' => wp = rotate(wp, v),
            'L' => wp = rotate(wp, -v),
            'F' => {
                x += wp.0 * v;
                y += wp.1 * v;
            }
            _ => println!("unknown command: {}", c),
        }
    }

    (x, y)
}

pub fn run() -> Result {
    let input = read_input("day12")?;
    let instr = parse_input(&input)?;

    let (x, y) = execute(&instr);
    println!("part A: ({}, {}) -> {}", x, y, x.abs() + y.abs());

    let (x, y) = execute_real(&instr);
    println!("part B: ({}, {}) -> {}", x, y, x.abs() + y.abs());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = ["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .copied()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let instr = parse_input(&input).unwrap();

        let (x, y) = execute(&instr);
        assert_eq!((x, y), (17, -8));

        let (x, y) = execute_real(&instr);
        assert_eq!((x, y), (214, -72));
    }
}
