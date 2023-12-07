use crate::common::*;
use std::mem::replace;

fn parse_input(line: &str) -> Result<Vec<usize>> {
    line.split(',').map(|s| Ok(s.parse()?)).collect()
}

fn play_for_n_rounds(n: usize, nums: &[usize]) -> usize {
    if n < nums.len() {
        return nums[n];
    }

    let m = max(nums).unwrap();
    let mut spoken = vec![!0; usize::max(n, m + 1)];
    let k = nums.len() - 1;

    for (turn, &v) in enumerate(&nums[..k]) {
        spoken[v] = turn;
    }

    (k..n).fold(nums[k], |v, turn| {
        usize::saturating_sub(turn, replace(&mut spoken[v], turn))
    })
}

pub fn run() -> Result {
    let nums = parse_input(&read_input("day15")?[0])?;

    let result = play_for_n_rounds(2020 - 1, &nums);
    println!("part A: {}", result);

    let result = play_for_n_rounds(30_000_000 - 1, &nums);
    println!("part B: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = [0, 3, 6];
        let expected_output = [0, 3, 6, 0, 3, 3, 1, 0, 4, 0];

        let gotten_output = (0..10)
            .map(|i| play_for_n_rounds(i, &input))
            .collect::<Vec<_>>();

        assert_eq!(expected_output, &*gotten_output);
    }
}
