use crate::common::*;
use std::collections::HashSet;

fn find_xmass_weakness(numbers: &[usize]) -> Result<usize> {
    const PREAMBLE_LEN: usize = 25;
    let mut combinations = HashSet::new();

    for (i, &x) in enumerate(numbers) {
        if i >= PREAMBLE_LEN && !combinations.contains(&x) {
            return Ok(i);
        }

        for y in &numbers[..i] {
            combinations.insert(x + y);
        }
    }

    Err(anyhow!("number not found"))
}

fn find_range(numbers: &[usize], needle: usize) -> Result<(usize, usize)> {
    let mut sums = vec![0; numbers.len()];

    for n in 0..numbers.len() {
        for i in 0..(numbers.len() - n) {
            sums[i] += numbers[i + n];

            if sums[i] == needle {
                return Ok((i, i + n));
            }
        }
    }

    Err(anyhow!("range not found"))
}

pub fn run() -> Result {
    let numbers = read_input("day09")?
        .into_iter()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    let index = find_xmass_weakness(&numbers)?;
    let p = numbers[index];
    println!("part A: {}", p);

    let (begin, end) = find_range(&numbers[..index], p)?;
    let (min, max) = numbers[begin..=end].iter().minmax().into_option().unwrap();
    println!("part B: {:?}", min + max);

    Ok(())
}
