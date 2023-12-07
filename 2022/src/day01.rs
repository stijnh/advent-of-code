use crate::common::*;
use std::cmp::Reverse;

pub(crate) fn run(lines: Lines) -> Result {
    let mut totals = vec![];
    let mut current = 0;

    for line in lines {
        if line.is_empty() {
            totals.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>()?;
        }
    }

    totals.sort_by_key(|&e| Reverse(e));

    println!("part A: {:?}", totals[0]);
    println!("part B: {:?}", totals[0] + totals[1] + totals[2]);

    Ok(())
}
