use crate::common::*;
use std::cmp::Reverse;
use std::collections::VecDeque;

type Pos = [i64; 2];
type Map = HashMap<Pos, i64>;

fn parse_map(lines: Lines) -> Map {
    let mut map = HashMap::default();

    for (y, line) in enumerate(lines) {
        for (x, c) in enumerate(line.chars()) {
            map.insert([x as i64, y as i64], c.to_digit(10).unwrap() as i64);
        }
    }

    map
}

const ADJACENT: [Pos; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct State {
    position: Pos,
    prev_direction: Option<usize>,
    num_steps: usize,
}

enum CartType {
    Regular,
    Ultra,
}

fn find_min_heat(start: Pos, end: Pos, map: &Map, cart: CartType) -> i64 {
    let mut entries = HashMap::default();
    let mut queue = binary_heap_plus::BinaryHeap::new_by_key(|(_, l)| Reverse(*l));

    let key = State {
        position: start,
        prev_direction: None,
        num_steps: 100,
    };
    entries.insert(key, 0);
    queue.push((key, 0));

    while let Some((state, current_loss)) = queue.pop() {
        let [x, y] = state.position;
        if [x, y] == end {
            return current_loss;
        }

        for (dir, [dx, dy]) in enumerate(ADJACENT) {
            let position = [x + dx, y + dy];

            // We cannot go back
            if state.prev_direction == Some((dir + 2) % 4) {
                continue;
            }

            let num_steps = match cart {
                CartType::Regular => {
                    if state.prev_direction == Some(dir) {
                        if state.num_steps + 1 > 3 {
                            continue;
                        }

                        state.num_steps + 1
                    } else {
                        1
                    }
                }
                CartType::Ultra => {
                    if state.prev_direction == Some(dir) {
                        if state.num_steps + 1 > 10 {
                            continue;
                        }

                        state.num_steps + 1
                    } else {
                        if state.num_steps < 4 && state.prev_direction.is_some() {
                            continue;
                        }

                        1
                    }
                }
            };

            if let Some(delta) = map.get(&position) {
                let new_state = State {
                    position,
                    prev_direction: Some(dir),
                    num_steps,
                };

                let new_loss = current_loss + delta;

                if entries
                    .get(&new_state)
                    .map_or(true, |&old_loss| old_loss > new_loss)
                {
                    entries.insert(new_state, new_loss);
                    queue.push((new_state, new_loss));
                    continue;
                }
            }
        }
    }

    panic!("end was not reached!")
}

pub(crate) fn run(lines: Lines) -> Result {
    let map = parse_map(lines);
    let (&start, &end) = map.keys().minmax().into_option().unwrap();

    println!(
        "part A: {}",
        find_min_heat(start, end, &map, CartType::Regular)
    );
    println!(
        "part B: {}",
        find_min_heat(start, end, &map, CartType::Ultra)
    );

    Ok(())
}
