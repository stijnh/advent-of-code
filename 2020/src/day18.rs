use crate::common::*;
use std::collections::VecDeque;

type Tokens = VecDeque<char>;

fn tokenize(line: &str) -> Tokens {
    line.chars().filter(|&c| c != ' ').collect()
}

fn evaluate_generic(line: &str, prec: bool) -> Result<i64> {
    fn eval_value(tokens: &mut Tokens, prec: bool) -> Result<i64> {
        match tokens.pop_front() {
            Some('(') => {
                let v = eval_expr(tokens, prec)?;
                let c = tokens.pop_front();
                if c != Some(')') {
                    bail!("expecting ')', found {:?}", c);
                }

                Ok(v)
            }
            Some(c) if c.is_ascii_digit() => Ok(c as i64 - '0' as i64),
            x => {
                bail!("expecting value, found {:?}", x);
            }
        }
    }

    fn eval_expr(tokens: &mut Tokens, prec: bool) -> Result<i64> {
        let mut v = eval_value(tokens, prec)?;

        loop {
            match tokens.pop_front() {
                Some('*') => {
                    if prec {
                        v *= eval_expr(tokens, prec)?;
                    } else {
                        v *= eval_value(tokens, prec)?;
                    }
                }
                Some('+') => {
                    v += eval_value(tokens, prec)?;
                }
                Some(c) => {
                    tokens.push_front(c);
                    break Ok(v);
                }
                None => {
                    break Ok(v);
                }
            }
        }
    }

    let mut tokens = tokenize(line);
    let v = eval_expr(&mut tokens, prec)?;
    if let Some(c) = tokens.pop_front() {
        bail!("expecting EOL, found '{}'", c);
    }

    Ok(v)
}

fn evaluate(line: &str) -> Result<i64> {
    evaluate_generic(line, false)
}

fn evaluate_precedence(line: &str) -> Result<i64> {
    evaluate_generic(line, true)
}

pub fn run() -> Result {
    let lines = read_input("day18")?;

    let mut sum = 0;
    for line in &lines {
        sum += evaluate(line)?;
    }
    println!("part A: {}", sum);

    let mut sum = 0;
    for line in &lines {
        sum += evaluate_precedence(line)?;
    }
    println!("part B: {}", sum);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        assert_eq!(evaluate("2 * 3 + (4 * 5)").unwrap(), 26);
        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), 437);
        assert_eq!(
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            12240
        );
        assert_eq!(
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            13632
        );
    }

    #[test]
    fn part_b() {
        assert_eq!(evaluate_precedence("2 * 3 + (4 * 5)").unwrap(), 46);
        assert_eq!(
            evaluate_precedence("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(),
            1445
        );
        assert_eq!(
            evaluate_precedence("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            669060
        );
        assert_eq!(
            evaluate_precedence("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            23340
        );
    }
}
