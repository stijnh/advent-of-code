use crate::common::*;
use std::collections::VecDeque;

type Pos = [i32; 2];

fn parse_grid(lines: Lines) -> (HashMap<Pos, i32>, Pos, Pos) {
    let mut grid = HashMap::default();
    let mut start = Pos::default();
    let mut end = Pos::default();

    for (i, &line) in enumerate(lines) {
        for (j, c) in line.chars().enumerate() {
            let p = [i as i32, j as i32];

            let height = match c {
                'a'..='z' => c as i32 - 'a' as i32,
                'S' => {
                    start = p;
                    0
                }
                'E' => {
                    end = p;
                    25
                }
                _ => panic!("invalid character: {:?}", c),
            };

            grid.insert(p, height);
        }
    }

    (grid, start, end)
}

fn calculate_dists(heights: &HashMap<Pos, i32>, start: Pos) -> HashMap<Pos, i32> {
    let mut dists = HashMap::default();
    let mut queue = VecDeque::new();

    dists.insert(start, 0);
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        for [dx, dy] in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
            let neighbor = [pos[0] + dx, pos[1] + dy];

            if heights.contains_key(&neighbor) && !dists.contains_key(&neighbor) {
                if heights[&pos] - heights[&neighbor] <= 1 {
                    dists.insert(neighbor, dists[&pos] + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    dists
}

pub(crate) fn run(lines: Lines) -> Result {
    let (heights, start, end) = parse_grid(lines);
    let dists = calculate_dists(&heights, end);
    println!("part A: {:?}", dists[&start]);

    let (_, best_dist) = dists
        .iter()
        .filter(|(&k, _)| heights[&k] == 0)
        .min_by_key(|(_, &v)| v)
        .unwrap();

    println!("part B: {:?}", best_dist);

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
