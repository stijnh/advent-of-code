use crate::common::*;
use std::array;
use std::cmp::Ordering;
use std::ops::{Range, RangeInclusive};

#[derive(Debug)]
struct Part {
    xmas: [i64; 4],
}

#[derive(Debug)]
struct Rule {
    property: usize,
    ordering: Ordering,
    value: i64,
    target: String,
}

fn parse_rule(line: &str) -> Rule {
    let Some(m) = find_regex("([a-z])([<>])(\\d+):([a-zAR]+)", line) else {
        return Rule {
            property: 0,
            ordering: Ordering::Greater,
            value: 0,
            target: line.to_string(),
        };
    };

    let property = "xmas".find(m[1].as_bytes()[0] as char).unwrap();
    let ordering = match &m[2] {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        o => panic!("invalid ordering: {o}"),
    };
    let value = m[3].parse().unwrap();
    let target = m[4].to_string();

    Rule {
        property,
        ordering,
        value,
        target,
    }
}

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    let m = find_regex("([a-z]+)[{]([^}]+)[}]", line).unwrap();
    let name = m[1].to_string();
    let rules = m[2].split(",").map(|l| parse_rule(l)).collect();

    (name, rules)
}

fn parse_part(line: &str) -> Part {
    let m = find_regex("[{]x=(\\d+),m=(\\d+),a=(\\d+),s=(\\d+)[}]", line).unwrap();

    Part {
        xmas: array::from_fn(|i| m[1 + i].parse().unwrap()),
    }
}

fn is_accepted(part: &Part, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut current = "in";

    loop {
        match current {
            "A" => return true,
            "R" => return false,
            _ => {}
        };

        let r = workflows[current]
            .iter()
            .find(|rule| i64::cmp(&part.xmas[rule.property], &rule.value) == rule.ordering)
            .unwrap();

        current = &r.target;
    }
}

#[derive(Clone, Debug)]
struct PartRanges {
    xmas: [RangeInclusive<i64>; 4],
}

impl PartRanges {
    fn new() -> Self {
        Self {
            xmas: array::from_fn(|_| 1..=4000),
        }
    }

    fn count_combinations(&self) -> i64 {
        self.xmas.iter().map(|r| r.end() - r.start() + 1).product()
    }
}

fn collect_accepted_ranges(
    current: &str,
    workflows: &HashMap<String, Vec<Rule>>,
) -> Vec<PartRanges> {
    fn recur(
        current: &str,
        mut ranges: PartRanges,
        workflows: &HashMap<String, Vec<Rule>>,
        results: &mut Vec<PartRanges>,
    ) {
        if current == "R" {
            return;
        }

        if current == "A" {
            results.push(ranges);
            return;
        }

        let rules = &workflows[current];

        for rule in rules {
            let i = rule.property;
            let (&start, &end) = (ranges.xmas[i].start(), ranges.xmas[i].end());

            let (ar, br) = if rule.ordering == Ordering::Less {
                (
                    start..=i64::min(end, rule.value - 1),
                    i64::max(start, rule.value)..=end,
                )
            } else if rule.ordering == Ordering::Greater {
                (
                    i64::max(start, rule.value + 1)..=end,
                    start..=i64::min(end, rule.value),
                )
            } else {
                unreachable!()
            };

            if ar.start() <= ar.end() {
                let mut a = ranges.clone();
                a.xmas[i] = ar;
                recur(&rule.target, a, workflows, results);
            }

            if br.start() < br.end() {
                ranges.xmas[i] = br;
                continue;
            }

            return;
        }

        panic!();
    }

    let mut results = vec![];
    recur(current, PartRanges::new(), workflows, &mut results);
    results
}

pub(crate) fn run(lines: Lines) -> Result {
    let mut workflows = HashMap::default();
    let mut parts = vec![];
    let mut parsing_workflows = true;

    for line in lines {
        if line.is_empty() {
            parsing_workflows = false;
        } else if parsing_workflows {
            let (name, rules) = parse_workflow(line);
            workflows.insert(name, rules);
        } else {
            parts.push(parse_part(line));
        }
    }

    let total: i64 = parts
        .iter()
        .filter(|p| is_accepted(p, &workflows))
        .flat_map(|p| &p.xmas)
        .sum();

    println!("part A: {}", total);

    let count: i64 = collect_accepted_ranges("in", &workflows)
        .iter()
        .map(|r| r.count_combinations())
        .sum();

    println!("part B: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workflows() -> HashMap<String, Vec<Rule>> {
        HashMap::from_iter([
            parse_workflow("px{a<2006:qkq,m>2090:A,rfg}"),
            parse_workflow("pv{a>1716:R,A}"),
            parse_workflow("lnx{m>1548:A,A}"),
            parse_workflow("rfg{s<537:gd,x>2440:R,A}"),
            parse_workflow("qs{s>3448:A,lnx}"),
            parse_workflow("qkq{x<1416:A,crn}"),
            parse_workflow("crn{x>2662:A,R}"),
            parse_workflow("in{s<1351:px,qqz}"),
            parse_workflow("qqz{s>2770:qs,m<1801:hdj,R}"),
            parse_workflow("gd{a>3333:R,R}"),
            parse_workflow("hdj{m>838:A,pv}"),
        ])
    }

    #[test]
    fn test_a() {
        let workflows = workflows();

        assert_eq!(
            is_accepted(&parse_part("{x=787,m=2655,a=1222,s=2876}"), &workflows),
            true
        );
        assert_eq!(
            is_accepted(&parse_part("{x=1679,m=44,a=2067,s=496}"), &workflows),
            false
        );
        assert_eq!(
            is_accepted(&parse_part("{x=2036,m=264,a=79,s=2244}"), &workflows),
            true
        );
        assert_eq!(
            is_accepted(&parse_part("{x=2461,m=1339,a=466,s=291}"), &workflows),
            false
        );
        assert_eq!(
            is_accepted(&parse_part("{x=2127,m=1623,a=2188,s=1013}"), &workflows),
            true
        );

        //
    }

    #[test]
    fn test_b() {
        let workflows = workflows();

        assert_eq!(
            collect_accepted_ranges("in", &workflows)
                .iter()
                .map(|r| r.count_combinations())
                .sum::<i64>(),
            167409079868000
        );
    }
}
