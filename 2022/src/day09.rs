use crate::common::*;
use std::collections::HashSet;

type Pos = [i32; 2];

fn parse_moves(lines: Lines) -> Vec<Pos> {
    let [mut x, mut y] = [0, 0];
    let mut pos = vec![[x, y]];

    for line in lines {
        let matches = find_regex("([RLUD]) ([0-9]+)", line).unwrap();
        let steps = matches[2].parse().unwrap();

        for _ in 0..steps {
            match &matches[1] {
                "R" => x += 1,
                "L" => x -= 1,
                "U" => y += 1,
                "D" => y -= 1,
                _ => unreachable!(),
            }

            pos.push([x, y]);
        }
    }

    pos
}

fn follow_path(head: &[Pos]) -> Vec<Pos> {
    let [mut tx, mut ty] = [0, 0];
    let mut tail = vec![[tx, ty]];

    for &[hx, hy] in head {
        if (tx - hx).abs() > 1 || (ty - hy).abs() > 1 {
            if tx < hx {
                tx += 1;
            } else if tx > hx {
                tx -= 1;
            }

            if ty < hy {
                ty += 1;
            } else if ty > hy {
                ty -= 1;
            }
        }

        tail.push([tx, ty]);
    }

    tail
}

pub(crate) fn run(lines: Lines) -> Result {
    let head = parse_moves(lines);
    let mut tail = follow_path(&head);

    println!("part A: {}", tail.iter().collect::<HashSet<_>>().len());

    for _ in 0..8 {
        tail = follow_path(&tail);
    }

    println!("part B: {}", tail.iter().collect::<HashSet<_>>().len());

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
