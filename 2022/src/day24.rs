use crate::common::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Blizzard {
    sx: i64,
    sy: i64,
    dx: i64,
    dy: i64,
}

fn parse_blizzards(lines: Lines) -> ([i64; 2], Vec<Blizzard>) {
    let mut blizzards = vec![];
    let height = lines.len() - 2;
    let width = lines[0].chars().count() - 2;

    for (y, line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            let [dx, dy] = match c {
                '<' => [-1, 0],
                '>' => [1, 0],
                '^' => [0, -1],
                'v' => [0, 1],
                '.' => continue,
                '#' => continue,
                _ => panic!("invalid character: {:?}", c),
            };

            blizzards.push(Blizzard {
                sx: x as i64 - 1,
                sy: y as i64 - 1,
                dx,
                dy,
            });
        }
    }

    ([width as i64, height as i64], blizzards)
}

fn search_path(
    start: [i64; 2],
    goal: [i64; 2],
    dims: [i64; 2],
    time_start: i64,
    blizzards: &[Blizzard],
) -> Option<i64> {
    let [width, height] = dims;
    let mut pos = HashSet::default();
    pos.insert(start);
    let mut t = time_start;

    loop {
        let mut obstacles = HashSet::default();
        for b in blizzards {
            obstacles.insert((
                (b.sx + t * b.dx).rem_euclid(width),
                (b.sy + t * b.dy).rem_euclid(height),
            ));
        }

        let mut new_pos = HashSet::default();
        for [x, y] in pos {
            for (dx, dy) in [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);

                if nx < 0 || ny < 0 || nx >= width || ny >= height {
                    continue;
                }

                if !obstacles.contains(&(nx, ny)) {
                    new_pos.insert([nx, ny]);
                }
            }
        }

        if let Some(_) = new_pos.get(&goal) {
            return Some(t);
        }

        if new_pos.is_empty() {
            return None;
        }

        pos = new_pos;
        t += 1;
    }
}

pub(crate) fn run(lines: Lines) -> Result {
    let (dims, blizzards) = parse_blizzards(lines);
    let start = [0, 0];
    let goal = [dims[0] - 1, dims[1] - 1];

    // Walk from start to goal
    let mut t = search_path(start, goal, dims, 0, &blizzards).unwrap() + 1;
    println!("part A: {}", t);

    // Walk from goal back to start
    let mut t = loop {
        if let Some(t) = search_path(goal, start, dims, t, &blizzards) {
            break t + 1;
        }

        t += 1;
    };

    // Walk from start back to goal
    let t = loop {
        if let Some(t) = search_path(start, goal, dims, t, &blizzards) {
            break t + 1;
        }

        t += 1;
    };

    println!("part B: {}", t);

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
