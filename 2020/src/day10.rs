use crate::common::*;
use defaultmap::DefaultHashMap;

fn find_differences(adapters: &[i32]) -> (usize, usize) {
    let mut diffs = DefaultHashMap::new(0);
    let mut prev = 0;

    for &number in &*adapters {
        diffs[number - prev] += 1;
        prev = number;
    }

    diffs[3] += 1; // For the last adapter

    return (diffs[1], diffs[3]);
}

fn find_combinations(adapters: &[i32]) -> u128 {
    let mut result = 0;
    let mut count = DefaultHashMap::new(0);
    count[0] = 1;

    for &v in adapters {
        count[v] = count[v - 1] + count[v - 2] + count[v - 3];
        result = count[v];
    }

    result
}

pub fn run() -> Result {
    let mut adapters = read_input("day10")?
        .iter()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    adapters.sort();

    let (diff1, diff3) = find_differences(&adapters);
    println!("part A: {}", diff1 * diff3);

    let combinations = find_combinations(&adapters);
    println!("part B: {}", combinations);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_difference() {
        let mut example1 = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        example1.sort();

        let mut example2 = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        example2.sort();

        assert_eq!(find_differences(&example1), (7, 5));
        assert_eq!(find_differences(&example2), (22, 10));

        assert_eq!(find_combinations(&example1), 8);
        assert_eq!(find_combinations(&example2), 19208);
    }
}
