use crate::common::*;
use ndarray::Array2;

struct Instruction {
    direction: char,
    count: i64,
    color: String,
}

fn parse_direction(line: &str) -> Instruction {
    let m = find_regex("([LDRU]) ([0-9]+) [(]#([0-9a-f]{6})[)]", line).unwrap();

    Instruction {
        direction: m[1].chars().next().unwrap(),
        count: m[2].parse().unwrap(),
        color: m[3].to_string(),
    }
}

fn parse_correct_direction(line: &str) -> Instruction {
    let m = find_regex("[LDRU] [0-9]+ [(]#([0-9a-f]{5})([0-3])[)]", line).unwrap();

    Instruction {
        direction: b"RDLU"[m[2].parse::<usize>().unwrap()] as char,
        count: i64::from_str_radix(&m[1], 16).unwrap(),
        color: String::new(),
    }
}

fn dig_trench(instrs: &[Instruction]) -> Vec<[i64; 2]> {
    let [mut x, mut y] = [0, 0];
    let mut path = vec![[x, y]];

    for instr in instrs {
        let [dx, dy] = match instr.direction {
            'D' => [0, 1],
            'U' => [0, -1],
            'L' => [-1, 0],
            'R' => [1, 0],
            _ => unreachable!(),
        };

        x += dx * instr.count;
        y += dy * instr.count;
        path.push([x, y]);
    }

    path
}

fn dig_lagoon(trench: &[[i64; 2]]) -> i64 {
    let mut area = 0;
    let mut border = 0;

    for i in 0..trench.len() {
        let p = trench[i];
        let q = trench[(i + 1) % trench.len()];

        let [x, y] = p;
        let [dx, dy] = [q[0] - p[0], q[1] - p[1]];

        area += y * dx;
        border += dx.abs() + dy.abs();
    }

    area.abs() + border / 2 + 1
}

pub(crate) fn run(lines: Lines) -> Result {
    let instr = lines.iter().copied().map(parse_direction).collect_vec();
    let trench = dig_trench(&instr);
    let lagoon = dig_lagoon(&trench);
    println!("part A: {}", lagoon);

    let instr = lines
        .iter()
        .copied()
        .map(parse_correct_direction)
        .collect_vec();
    let trench = dig_trench(&instr);
    let lagoon = dig_lagoon(&trench);
    println!("part B: {}", lagoon);

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
