use crate::common::*;

fn mix_numbers(numbers: &[i64], times: usize) -> Vec<i64> {
    let n = numbers.len();
    let mut nums = numbers.to_vec();
    let mut pos = (0..n).collect_vec();

    for _ in 0..times {
        for i in 0..n {
            let delta = numbers[i];
            let old_index = pos.iter().position(|&p| p == i).unwrap();
            let mut new_index = old_index
                + if delta >= 0 {
                    delta as usize % (n - 1)
                } else {
                    (n - 1) - (((-delta) as usize) % (n - 1))
                };

            if new_index >= n {
                new_index -= n - 1;
            }

            if old_index < new_index {
                nums[old_index..=new_index].rotate_left(1);
                pos[old_index..=new_index].rotate_left(1);
            } else if old_index > new_index {
                nums[new_index..=old_index].rotate_right(1);
                pos[new_index..=old_index].rotate_right(1);
            }
        }
    }

    nums
}

fn find_thousands(nums: &[i64]) -> [i64; 3] {
    let n = nums.len();
    let index = nums.iter().position(|&v| v == 0).unwrap();

    [
        nums[(index + 1000) % n],
        nums[(index + 2000) % n],
        nums[(index + 3000) % n],
    ]
}

pub(crate) fn run(lines: Lines) -> Result {
    let numbers = lines
        .iter()
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let [a, b, c] = find_thousands(&mix_numbers(&numbers, 1));
    println!("part A: {}", a + b + c);

    let new_numbers = numbers.iter().map(|v| v * 811589153).collect_vec();

    let [a, b, c] = find_thousands(&mix_numbers(&new_numbers, 10));
    println!("part A: {}", a + b + c);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let nums = vec![1, 2, -3, 3, -2, 0, 4];
        let expected = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(mix_numbers(&nums, 1), expected);

        assert_eq!(find_thousands(&expected), [4, -3, 2]);
    }

    #[test]
    fn test_b() {
        //
    }
}
