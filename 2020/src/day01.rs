use crate::common::*;
use std::cmp::Ordering::*;

fn parse_input(filename: &str) -> Result<Vec<usize>> {
    let mut numbers = read_input(filename)?
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().context("invalid number"))
        .collect::<Result<Vec<_>, _>>()?;

    numbers.sort_unstable();
    Ok(numbers)
}

fn find_two(numbers: &[usize], sum: usize) -> Option<[usize; 2]> {
    let (mut i, mut j) = (0, numbers.len() - 1);

    while i < j {
        let (a, b) = (numbers[i], numbers[j]);

        match cmp(a + b, sum) {
            Equal => return Some([a, b]),
            Greater => j -= 1,
            Less => i += 1,
        }
    }

    None
}

fn find_three(numbers: &[usize], sum: usize) -> Option<[usize; 3]> {
    for &a in numbers {
        if a <= sum {
            if let Some([b, c]) = find_two(numbers, sum - a) {
                return Some([a, b, c]);
            }
        }
    }

    None
}

pub fn run() -> Result {
    let numbers = parse_input("day01")?;

    let [a, b] = find_two(&numbers, 2020)
        .ok_or_else(|| anyhow!("failed to find two numbers that sum to 2020"))?;

    println!("parts A: {}", a * b);

    let [a, b, c] = find_three(&numbers, 2020)
        .ok_or_else(|| anyhow!("failed to find three numbers that sum to 2020"))?;

    println!("parts B: {}", a * b * c);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn numbers() -> Vec<usize> {
        let mut numbers = vec![1721, 979, 366, 299, 675, 1456];

        numbers.sort_unstable();
        numbers
    }

    #[test]
    fn test_two() {
        let [a, b] = find_two(&numbers(), 2020).expect("to find two numbers");
        assert_eq!([a, b], [299, 1721]);
    }

    #[test]
    fn test_three() {
        let [a, b, c] = find_three(&numbers(), 2020).expect("to find three numbers");
        assert_eq!([a, b, c], [366, 675, 979]);
    }
}
