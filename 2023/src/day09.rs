use crate::common::*;

fn predict_next(numbers: &[i64]) -> i64 {
    if let Some(last) = numbers.last() {
        let diffs = numbers
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        last + predict_next(&diffs)
    } else {
        0
    }
}

fn predict_prev(numbers: &[i64]) -> i64 {
    if let Some(first) = numbers.first() {
        let diffs = numbers
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();
        first - predict_prev(&diffs)
    } else {
        0
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let numbers = lines
        .iter()
        .map(|l| parse_list::<i64>(l, ' ').unwrap())
        .collect_vec();

    println!("part A: {}", sum(map(&numbers, |n| predict_next(&n))));
    println!("part B: {}", sum(map(&numbers, |n| predict_prev(&n))));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(predict_next(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict_next(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(predict_next(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_b() {
        assert_eq!(predict_prev(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(predict_prev(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(predict_prev(&[10, 13, 16, 21, 30, 45]), 5);
    }
}
