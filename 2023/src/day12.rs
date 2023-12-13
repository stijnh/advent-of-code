use crate::common::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Record {
    condition: Vec<char>,
    groups: Vec<usize>,
}

fn parse_record(line: &str) -> Record {
    let (condition, groups) = line.split_once(' ').unwrap();

    Record {
        condition: condition.chars().collect(),
        groups: parse_list(groups, ',').unwrap(),
    }
}

fn count_arrangements_recur<'a>(
    condition: &'a [char],
    groups: &'a [usize],
    cache: &mut HashMap<(&'a [char], &'a [usize]), usize>,
) -> usize {
    if condition.is_empty() {
        return groups.is_empty() as usize;
    }

    let key = (condition, groups);
    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let mut result = 0;

    if condition[0] != '#' {
        result += count_arrangements_recur(&condition[1..], groups, cache);
    }

    if condition[0] != '.' {
        if let Some(&n) = groups.get(0) {
            if (0..n).all(|i| condition.get(i).unwrap_or(&'.') != &'.') {
                result += match condition.get(n) {
                    None => (groups.len() == 1) as usize,
                    Some('#') => 0,
                    Some(_) => count_arrangements_recur(&condition[n + 1..], &groups[1..], cache),
                };
            }
        }
    }

    cache.insert(key, result);
    result
}

fn count_arrangements(record: &Record) -> usize {
    count_arrangements_recur(&record.condition, &record.groups, &mut default())
}

fn unfold_record(record: &Record) -> Record {
    let mut result = record.clone();

    for _ in 1..5 {
        result.condition.push('?');
        result.condition.extend(&record.condition);
        result.groups.extend(&record.groups);
    }

    result
}

pub(crate) fn run(lines: Lines) -> Result {
    let records = lines.iter().map(|line| parse_record(line)).collect_vec();

    let total: usize = records.iter().map(count_arrangements).sum();

    println!("part A: {}", total);

    let total: usize = records
        .iter()
        .map(|r| count_arrangements(&unfold_record(r)))
        .sum();

    println!("part B: {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let f = |l| count_arrangements(&parse_record(l));

        assert_eq!(f("???.### 1,1,3"), 1);
        assert_eq!(f(".??..??...?##. 1,1,3"), 4);
        assert_eq!(f("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(f("????.#...#... 4,1,1"), 1);
        assert_eq!(f("????.######..#####. 1,6,5"), 4);
        assert_eq!(f("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn test_b() {
        let f = |l| count_arrangements(&unfold_record(&parse_record(l)));

        assert_eq!(f("???.### 1,1,3"), 1);
        assert_eq!(f(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(f("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(f("????.#...#... 4,1,1"), 16);
        assert_eq!(f("????.######..#####. 1,6,5"), 2500);
        assert_eq!(f("?###???????? 3,2,1"), 506250);
    }
}
