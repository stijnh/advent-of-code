use crate::common::*;

type Val = i64;

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
enum Expr {
    Constant(Val),
    Binary(String, BinOp, String),
}

fn parse_expressions(lines: Lines) -> HashMap<String, Expr> {
    let mut result = HashMap::default();

    for line in lines {
        let mut parts = line.splitn(2, ": ");
        let name = parts.next().unwrap();
        let expr = parts.next().unwrap();

        let expr = if let Some(matches) = find_regex("[0-9]+", expr) {
            Expr::Constant(matches[0].parse().unwrap())
        } else if let Some(matches) = find_regex("([a-z]{4}) ([+-/*]) ([a-z]{4})", expr) {
            let op = match &matches[2] {
                "+" => BinOp::Add,
                "-" => BinOp::Sub,
                "/" => BinOp::Div,
                "*" => BinOp::Mul,
                _ => unreachable!(),
            };

            Expr::Binary(matches[1].to_string(), op, matches[3].to_string())
        } else {
            panic!("invalid line: {}", line);
        };

        result.insert(name.to_string(), expr);
    }

    result
}

fn evaluate(key: &str, monkeys: &HashMap<String, Expr>) -> Option<Val> {
    Some(match monkeys.get(key)? {
        Expr::Constant(v) => *v,
        Expr::Binary(a, op, b) => {
            let a = evaluate(a, monkeys)?;
            let b = evaluate(b, monkeys)?;

            match op {
                BinOp::Add => a + b,
                BinOp::Sub => a - b,
                BinOp::Mul => a * b,
                BinOp::Div => a / b,
            }
        }
    })
}

fn solve<'a>(mut var: &'a str, val: Val, monkeys: &'a HashMap<String, Expr>) -> (&'a str, Val) {
    let Some(expr) = monkeys.get(var) else {
        return (var, val);
    };

    let Expr::Binary(lhs, op, rhs) = expr else {
            panic!("invalid expression");
        };

    match (evaluate(lhs, monkeys), op, evaluate(rhs, monkeys)) {
        (Some(lhs), BinOp::Add, None) => solve(rhs, val - lhs, monkeys),
        (None, BinOp::Add, Some(rhs)) => solve(lhs, val - rhs, monkeys),
        (Some(lhs), BinOp::Sub, None) => solve(rhs, lhs - val, monkeys),
        (None, BinOp::Sub, Some(rhs)) => solve(lhs, val + rhs, monkeys),
        (Some(lhs), BinOp::Mul, None) => solve(rhs, val / lhs, monkeys),
        (None, BinOp::Mul, Some(rhs)) => solve(lhs, val / rhs, monkeys),
        (Some(lhs), BinOp::Div, None) => solve(rhs, lhs / val, monkeys),
        (None, BinOp::Div, Some(rhs)) => solve(lhs, val * rhs, monkeys),
        other => {
            panic!("invalid expression: {:?}", other);
        }
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let mut monkeys = parse_expressions(lines);
    println!("part A: {:?}", evaluate("root", &monkeys));

    // Remove human monkey
    monkeys.remove("humn");

    // Solve `var == constant` or `constant == var` for root
    let answer = match &monkeys["root"] {
        Expr::Binary(lhs, _, rhs) => match (evaluate(lhs, &monkeys), evaluate(rhs, &monkeys)) {
            (Some(lhs), None) => solve(rhs, lhs, &monkeys),
            (None, Some(rhs)) => solve(lhs, rhs, &monkeys),
            _ => unreachable!(),
        },
        Expr::Constant(_) => unreachable!(),
    };

    println!("part B: {:?}", answer);

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
