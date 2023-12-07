use crate::common::*;

fn parse_snafu(input: &str) -> i64 {
    let mut output = 0;

    for c in input.chars() {
        output *= 5;
        output += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid character: {:?}", c),
        };
    }

    output
}

fn generate_snafu(mut input: i64) -> String {
    let mut output = String::new();
    let mut positions = 0;
    let mut power = 1;

    loop {
        let x = ((input as f64) / (power as f64)).round() as i64;
        if x >= -2 && x <= 2 {
            break;
        }

        power *= 5;
        positions += 1;
    }

    for _ in 0..=positions {
        let x = ((input as f64) / (power as f64)).round() as i64;
        input -= x * power;
        power /= 5;

        output += match x {
            2 => "2",
            1 => "1",
            0 => "0",
            -1 => "-",
            -2 => "=",
            _ => "?",
        };
    }

    output
}

pub(crate) fn run(lines: Lines) -> Result {
    let numbers = lines.iter().map(|l| parse_snafu(l)).collect_vec();

    let total = numbers.iter().sum::<i64>();

    println!("part A: {:?}", generate_snafu(total));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(parse_snafu("2=-01"), 976);
        assert_eq!(parse_snafu("1-0---0"), 12345);
        assert_eq!(parse_snafu("1121-1110-1=0"), 314159265);

        for i in 1..100 {
            assert_eq!(
                parse_snafu(&generate_snafu(i)),
                i,
                "invalid snafu number {}: {}",
                i,
                generate_snafu(i)
            );
        }
    }
}
