use crate::common::*;
use ndarray::{Array2, ArrayView2, Axis};
use std::collections::VecDeque;

fn parse_map(lines: Lines) -> Array2<char> {
    let width = lines[0].chars().count();
    let height = lines.len();
    let mut map = Array2::from_elem((width, height), ' ');

    for (y, line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            map[[x, y]] = c;
        }
    }

    map
}

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
struct State([i64; 2], [i64; 2]);

fn count_energized(map: ArrayView2<char>, start: State) -> usize {
    let mut visited = HashSet::default();
    let mut queue = VecDeque::from_iter([start]);

    while let Some(State(xy, dxy)) = queue.pop_front() {
        let [x, y] = xy;
        let [dx, dy] = dxy;

        let Some(c) = map.get([x as usize, y as usize]) else {
            continue;
        };

        if !visited.insert(State(xy, dxy)) {
            continue;
        }

        let [dx, dy] = match c {
            '|' if dy == 0 => {
                queue.push_back(State(xy, [0, 1]));
                [0, -1]
            }
            '-' if dx == 0 => {
                queue.push_back(State(xy, [1, 0]));
                [-1, 0]
            }
            '/' => [-dy, -dx],
            '\\' => [dy, dx],
            _ => [dx, dy],
        };

        queue.push_back(State([x + dx, y + dy], [dx, dy]));
    }

    visited.into_iter().map(|State(xy, _)| xy).unique().count()
}

fn find_optimal_start(map: ArrayView2<char>) -> State {
    let width = map.len_of(Axis(0)) as i64;
    let height = map.len_of(Axis(1)) as i64;

    Iterator::chain(
        (0..width).flat_map(|x| // width
            [State([x, 0], [0, 1]), State([x, width - 1], [0, -1])]),
        (0..height).flat_map(|y| // height
            [State([0, y], [1, 0]), State([height - 1, 0], [-1, 0])]),
    )
    .max_by_key(|&s| count_energized(map, s))
    .unwrap()
}

pub(crate) fn run(lines: Lines) -> Result {
    let map = parse_map(lines);
    let start = State([0, 0], [1, 0]);

    println!("part A: {}", count_energized(map.view(), start));

    let start = find_optimal_start(map.view());
    println!("part B: {}", count_energized(map.view(), start));

    Ok(())
}
