use crate::common::*;

fn find_calibration_value(line: &str) -> u32 {
    let digits = line.chars().filter_map(|c| c.to_digit(10));
    let (a, b) = (digits.clone().next().unwrap(), digits.last().unwrap());
    a * 10 + b
}

fn replace_digit_words(line: &str) -> String {
    let mapping = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    line.char_indices()
        .map(|(index, _)| &line[index..])
        .map(|prefix| {
            mapping
                .iter()
                .find_map(|&(a, b)| prefix.starts_with(a).then_some(b))
                .unwrap_or(&prefix[..1])
        })
        .join("")
}

pub(crate) fn run(lines: Lines) -> Result {
    let sum: u32 = lines
        .into_iter()
        .map(|line| find_calibration_value(line))
        .sum();

    println!("part A: {}", sum);

    let sum: u32 = lines
        .into_iter()
        .map(|line| replace_digit_words(line))
        .map(|line| find_calibration_value(&line))
        .sum();

    println!("part B: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b() {
        assert_eq!(find_calibration_value(&replace_digit_words("two1nine")), 29);
        assert_eq!(
            find_calibration_value(&replace_digit_words("eightwothree")),
            83
        );
        assert_eq!(
            find_calibration_value(&replace_digit_words("abcone2threexyz")),
            13
        );
        assert_eq!(
            find_calibration_value(&replace_digit_words("xtwone3four")),
            24
        );
        assert_eq!(
            find_calibration_value(&replace_digit_words("4nineeightseven2")),
            42
        );
        assert_eq!(
            find_calibration_value(&replace_digit_words("zoneight234")),
            14
        );
        assert_eq!(
            find_calibration_value(&replace_digit_words("7pqrstsixteen")),
            76
        );
    }
}
