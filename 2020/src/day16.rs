use crate::common::*;
use recap::Recap;
use serde::Deserialize;

#[derive(Recap, Deserialize, Debug, Clone)]
#[recap(
    regex = "^(?P<key>[a-zA-Z ]+): (?P<a_lo>[0-9]+)-(?P<a_hi>[0-9]+) or (?P<b_lo>[0-9]+)-(?P<b_hi>[0-9]+)$"
)]
struct Rule {
    key: String,
    a_lo: i32,
    a_hi: i32,
    b_lo: i32,
    b_hi: i32,
}

type Ticket = Vec<i32>;

impl Rule {
    fn is_valid(&self, v: i32) -> bool {
        (v >= self.a_lo && v <= self.a_hi) || (v >= self.b_lo && v <= self.b_hi)
    }
}

fn parse_list(line: &str) -> Result<Vec<i32>> {
    line.split(',').map(|s| Ok(s.parse::<i32>()?)).collect()
}

fn expect(line: Option<&str>, expected: &str) -> Result {
    match line {
        Some(l) if l == expected => Ok(()),
        _ => bail!("unexpected line, expecting {:?}", expected),
    }
}

fn parse_input(lines: &[String]) -> Result<(Vec<Rule>, Ticket, Vec<Ticket>)> {
    let mut iter = lines.into_iter().map(|s| &**s);

    let mut rules = vec![];
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }

        rules.push(line.parse()?);
    }

    expect(iter.next(), "your ticket:")?;
    let my_ticket = parse_list(iter.next().unwrap_or_default())?;

    expect(iter.next(), "")?;
    expect(iter.next(), "nearby tickets:")?;
    let mut tickets = vec![];
    while let Some(line) = iter.next() {
        tickets.push(parse_list(line)?);
    }

    Ok((rules, my_ticket, tickets))
}

fn delete_invalid_tickets(rules: &[Rule], tickets: &mut Vec<Ticket>) -> i32 {
    let mut error = 0;

    tickets.retain(|ticket| {
        let mut is_valid = true;

        for &v in ticket {
            if !any(rules, |rule| rule.is_valid(v)) {
                is_valid = false;
                error += v;
            }
        }

        is_valid
    });

    error
}

fn reorder_fields(rules: &[Rule], tickets: &[Ticket]) -> Vec<Rule> {
    let n = tickets[0].len();
    let mut new_rules = rules.to_vec();
    let mut options: Vec<Vec<usize>> = vec![];
    options.resize(n, vec![]);

    for i in 0..n {
        for j in 0..n {
            if all(tickets, |v| rules[j].is_valid(v[i])) {
                options[i].push(j);
            }
        }
    }

    for _ in 0..n {
        let (mut j, mut i) = (!0, !0);
        for (index, p) in enumerate(&mut options) {
            if p.len() == 1 {
                i = index;
                j = p.pop().unwrap();
                break;
            }
        }

        new_rules[i] = rules[j].clone();

        for p in &mut options {
            p.retain(|&k| k != j);
        }
    }

    new_rules
}

pub fn run() -> Result {
    let (mut rules, my_ticket, mut tickets) = parse_input(&read_input("day16")?)?;

    let error_rate = delete_invalid_tickets(&rules, &mut tickets);
    println!("part A: {}", error_rate);

    let rules = reorder_fields(&mut rules, &tickets);
    let sum: usize = enumerate(rules)
        .filter(|(_, rule)| rule.key.starts_with("departure"))
        .map(|(i, _)| my_ticket[i] as usize)
        .product();

    println!("part B: {:?}", sum);

    Ok(())
}
