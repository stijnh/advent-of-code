use crate::common::*;
use ndarray::Array2;

fn simulate(lines: Lines) -> Vec<i32> {
    let mut reg = 1;
    let mut cycles = vec![];

    for &line in lines {
        if line == "noop" {
            cycles.push(reg);
        } else if let Some(matches) = find_regex("addx (-?[0-9]+)", line) {
            cycles.push(reg);
            cycles.push(reg);
            reg += matches[1].parse::<i32>().unwrap();
        } else {
            panic!("invalid line: {}", line);
        }
    }

    cycles
}

pub(crate) fn run(lines: Lines) -> Result {
    let cycles = simulate(lines);

    let indices = [20, 60, 100, 140, 180, 220];
    let sum = indices
        .iter()
        .map(|&i| i * cycles[i as usize - 1])
        .sum::<i32>();
    println!("part A: {}", sum);

    let (width, height) = (40, 6);
    let mut screen = Array2::from_elem((width, height), '.');
    for (cycle, reg) in enumerate(cycles) {
        let x = cycle % width;
        let y = cycle / width;

        if (reg - x as i32).abs() <= 1 {
            screen[[x, y]] = '#';
        }
    }

    let img = screen
        .columns()
        .into_iter()
        .map(|r| r.iter().join(""))
        .join("\n");

    println!("part B:");
    println!("{}", img);

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
