use crate::common::*;
use recap::Recap;
use serde::Deserialize;
use std::collections::VecDeque;

#[derive(Debug, Deserialize, PartialEq, Recap)]
#[recap(regex = r#"move (?P<count>[0-9]+) from (?P<src>[1-9]) to (?P<dst>[1-9])"#)]
struct Instruction {
    count: i32,
    src: usize,
    dst: usize,
}

fn parse_instructions(lines: &[&str]) -> Result<Vec<Instruction>> {
    lines
        .iter()
        .map(|&l| Instruction::try_from(l).context("invalid line"))
        .collect()
}

fn parse_stacks(lines: &[&str], ncols: usize) -> Vec<VecDeque<char>> {
    let mut queues = vec![VecDeque::new(); ncols];

    for line in lines {
        for (i, c) in line.char_indices() {
            if ('A'..='Z').contains(&c) {
                queues[i / 4].push_front(c);
            }
        }
    }

    queues
}

fn top_crates(queues: &[VecDeque<char>]) -> String {
    queues
        .iter()
        .map(|q| q.back().copied().unwrap_or('?'))
        .collect()
}

pub(crate) fn run(lines: Lines) -> Result {
    let queues = parse_stacks(&lines[..8], 9);
    let instrs = parse_instructions(&lines[10..])?;

    let mut result = queues.clone();
    for instr in &instrs {
        for _ in 0..instr.count {
            let item = result[instr.src - 1].pop_back().unwrap_or('?');
            result[instr.dst - 1].push_back(item);
        }
    }

    println!("part A: {}", top_crates(&result));

    let mut result = queues.clone();
    let mut temp = vec![];
    for instr in &instrs {
        for _ in 0..instr.count {
            temp.push(result[instr.src - 1].pop_back().unwrap_or('?'));
        }

        while let Some(item) = temp.pop() {
            result[instr.dst - 1].push_back(item);
        }
    }

    println!("part B: {}", top_crates(&result));

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
