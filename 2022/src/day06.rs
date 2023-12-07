use crate::common::*;
use std::collections::{HashSet, VecDeque};

fn find_marker_position(line: &str, n: usize) -> usize {
    let mut window = VecDeque::new();

    for (index, c) in line.chars().enumerate() {
        window.push_back(c);

        if window.len() < n {
            continue;
        }

        if window.len() > n {
            window.pop_front();
        }

        if HashSet::<&char>::from_iter(&window).len() == n {
            return index + 1;
        }
    }

    panic!("no marker found");
}

pub(crate) fn run(lines: Lines) -> Result {
    let line = lines[0];
    println!("part A: {}", find_marker_position(line, 4));
    println!("part B: {}", find_marker_position(line, 14));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker_position("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(
            find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            11
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            find_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            19
        );
        assert_eq!(find_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker_position("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }
}
