use crate::common::*;
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::mem::take;
use std::str::FromStr;

enum Operation {
    Add,
    Mul,
}

enum Operand {
    Constant(i64),
    Old,
}

struct Monkey {
    starting_items: Vec<i64>,
    operation: Operation,
    operand: Operand,
    divisible: i64,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn op(&self, lhs: i64) -> i64 {
        let rhs = match self.operand {
            Operand::Constant(v) => v,
            Operand::Old => lhs,
        };

        match self.operation {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
        }
    }
}

fn regex_number<T: FromStr>(pattern: &str, input: &str) -> Result<T> {
    Ok(find_regex(pattern, input)
        .ok_or_else(|| anyhow!("line {:?} failed regex {:?}", input, pattern))?
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap_or_else(|_| unreachable!()))
}

fn parse_monkeys(lines: Lines) -> Result<Vec<Monkey>> {
    let mut i = 0;
    let mut monkeys = vec![];

    while i < lines.len() {
        let divisible = regex_number("divisible by ([0-9]+)", lines[i + 3])?;
        let if_true = regex_number("If true: throw to monkey ([0-9]+)", lines[i + 4])?;
        let if_false = regex_number("If false: throw to monkey ([0-9]+)", lines[i + 5])?;

        let starting_items = find_regex_all("[0-9]+", lines[i + 1])
            .into_iter()
            .map(|e| e[0].parse::<i64>())
            .try_collect()?;

        let line = &lines[i + 2];
        let matches = find_regex("Operation: new = old ([*+]) ([0-9]+|old)", line).unwrap();

        let operation = match &matches[1] {
            "*" => Operation::Mul,
            "+" => Operation::Add,
            _ => unreachable!(),
        };

        let operand = match &matches[2] {
            "old" => Operand::Old,
            v => Operand::Constant(v.parse().unwrap()),
        };

        monkeys.push(Monkey {
            starting_items,
            operation,
            operand,
            divisible,
            if_true,
            if_false,
        });

        i += 7;
    }

    Ok(monkeys)
}

fn lcm(first: i64, second: i64) -> i64 {
    fn gcd(lhs: i64, rhs: i64) -> i64 {
        if lhs == 0 {
            rhs
        } else {
            gcd(rhs % lhs, lhs)
        }
    }

    first * second / gcd(first, second)
}

fn simulate(monkeys: &[Monkey], rounds: usize, divide: i64) -> usize {
    let mut items = vec![];
    let mut inspected = vec![];
    let mut factor = 1;

    for monkey in monkeys {
        inspected.push(0);
        items.push(VecDeque::from(monkey.starting_items.clone()));
        factor = lcm(factor, monkey.divisible);
    }

    factor *= divide;

    for _ in 0..rounds {
        for (index, monkey) in enumerate(monkeys) {
            for old_item in take(&mut items[index]) {
                inspected[index] += 1;
                let new_item = (monkey.op(old_item) / divide) % factor;

                if new_item % monkey.divisible == 0 {
                    items[monkey.if_true].push_back(new_item);
                } else {
                    items[monkey.if_false].push_back(new_item);
                }
            }
        }
    }

    inspected.sort_by_key(|&e| Reverse(e));
    inspected[0] * inspected[1]
}

pub(crate) fn run(lines: Lines) -> Result {
    let monkeys = parse_monkeys(lines)?;

    let score = simulate(&monkeys, 20, 3);
    println!("part A: {}", score);

    let score = simulate(&monkeys, 10000, 1);
    println!("part B: {}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        //
    }

    #[test]
    fn test_b() {
        //
    }
}
