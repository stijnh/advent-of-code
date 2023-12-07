use crate::common::*;
use ndarray::prelude::*;

enum Rule {
    Term(usize, char),
    Seq(usize, Vec<usize>),
}

fn parse_rule(line: &str, rules: &mut Vec<Rule>) -> Result {
    let mut parts = line.split(":");
    let index = parts
        .next()
        .unwrap_or_default()
        .parse::<usize>()
        .map_err(|_| anyhow!("failed to parse: {:?}", line))?;

    let rest = parts.next().unwrap_or_default().trim();
    for part in rest.split("|") {
        let rule = if part == "\"a\"" {
            Rule::Term(index, 'a')
        } else if part == "\"b\"" {
            Rule::Term(index, 'b')
        } else {
            let seq = part
                .trim()
                .split(" ")
                .map(|s| s.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;

            Rule::Seq(index, seq)
        };

        rules.push(rule);
    }

    Ok(())
}

fn parse_input(lines: &[String]) -> Result<(Vec<Rule>, Vec<String>)> {
    let index = lines.iter().position(|l| l.is_empty()).unwrap_or_default();
    let preamble = lines[..index].to_vec();
    let messages = lines[index..].to_vec();

    let mut rules = vec![];
    for line in preamble {
        parse_rule(&line, &mut rules)?;
    }

    Ok((rules, messages))
}

#[derive(Clone, Debug)]
struct NormRules {
    terms: Vec<(usize, char)>,           // R_i -> "c"
    triples: Vec<(usize, usize, usize)>, // R_i -> R_j R_k
    max_id: usize,
}

fn normalize_rules(input: &[Rule]) -> NormRules {
    let mut terms = vec![];
    let mut triples = vec![];
    let mut aliases = vec![];
    let mut next_id = input
        .iter()
        .map(|r| match r {
            Rule::Term(i, _) => *i,
            Rule::Seq(i, _) => *i,
        })
        .max()
        .unwrap()
        + 1;

    for rule in input {
        match rule {
            Rule::Term(i, c) => {
                terms.push((*i, *c));
            }
            Rule::Seq(i, list) if list.len() == 1 => {
                aliases.push((*i, list[0]));
            }
            Rule::Seq(i, list) if list.len() == 2 => {
                triples.push((*i, list[0], list[1]));
            }
            Rule::Seq(i, list) if list.len() == 3 => {
                triples.push((*i, list[0], next_id));
                triples.push((next_id, list[1], list[2]));
                next_id += 1;
            }
            _ => {
                panic!("unsupported rule");
            }
        }
    }

    while let Some((to, from)) = aliases.pop() {
        for (i, j, k) in triples.clone() {
            if i == from {
                triples.push((to, j, k));
            }
        }

        for (i, c) in terms.clone() {
            if i == from {
                terms.push((to, c));
            }
        }

        for (i, j) in aliases.clone() {
            if i == from {
                aliases.push((to, j));
            }
        }
    }

    NormRules {
        terms,
        triples,
        max_id: next_id,
    }
}

fn matches(line: &str, rules: &NormRules) -> bool {
    if line.is_empty() {
        return false;
    }

    let line: Vec<_> = line.chars().collect();
    let n = line.len();
    let r = rules.max_id;

    let terms = &rules.terms;
    let triples = &rules.triples;

    let mut m = Array3::from_elem((n, n, r), false);

    for i in 0..n {
        for &(a, s) in terms {
            if s == line[i] {
                m[[0, i, a]] = true;
            }
        }
    }

    for l in 1..n {
        for s in 0..=(n - l - 1) {
            for p in 0..l {
                for &(a, b, c) in triples {
                    if m[[p, s, b]] && m[[l - p - 1, s + p + 1, c]] {
                        m[[l, s, a]] = true;
                    }
                }
            }
        }
    }

    m[[n - 1, 0, 0]]
}

pub fn run() -> Result {
    let input = read_input("day19")?;
    let (mut rules, msgs) = parse_input(&input)?;

    let norm = normalize_rules(&rules);
    let mut count = 0;

    for msg in &msgs {
        let valid = matches(msg, &norm);
        count += valid as usize;

        println!("{}: {}", msg, if valid { "yes" } else { "no" });
    }

    println!("part A: {}", count);

    // Add some new rules
    parse_rule("8: 42 8 | 42", &mut rules)?;
    parse_rule("11: 42 11 31 | 42 31", &mut rules)?;

    let norm = normalize_rules(&rules);
    let mut count = 0;

    for msg in &msgs {
        let valid = matches(msg, &norm);
        count += valid as usize;

        println!("{}: {}", msg, if valid { "yes" } else { "no" });
    }

    println!("part B: {}", count);

    Ok(())
}
