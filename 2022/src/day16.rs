use crate::common::*;
use recap::Recap;
use serde::Deserialize;
use std::collections::hash_map::Entry;
use std::collections::VecDeque;

#[derive(Debug, Recap, Deserialize)]
#[recap(
    regex = r#"Valve (?P<name>[A-Z]{2}) has flow rate[=](?P<rate>[0-9]+); tunnels? leads? to valves? (?P<neighbors>.*)"#
)]
struct Line {
    name: String,
    rate: i64,
    neighbors: String,
}

#[derive(Debug)]
struct Node {
    // name: String,
    rate: i64,
    dists: Vec<i64>,
}

fn parse_cave(lines: Lines) -> Vec<Node> {
    let lines = lines
        .iter()
        .map(|&line| line.parse::<Line>().unwrap())
        .map(|l| (l.name.clone(), l))
        .collect::<HashMap<String, _>>();

    let node_names = lines
        .values()
        .filter(|l| l.rate > 0 || l.name == "AA")
        .map(|l| l.name.as_str())
        .sorted()
        .collect_vec();
    let mut nodes = vec![];

    for &name in &node_names {
        let mut dists = vec![-1; node_names.len()];
        let mut visited = HashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back((name, 0));

        while let Some((other, dist)) = queue.pop_front() {
            if !visited.insert(other) {
                continue;
            }

            if let Some(index) = node_names.iter().position(|n| n == &other) {
                dists[index] = dist;
            }

            for neighbor in lines[&*other].neighbors.split(", ") {
                queue.push_back((neighbor, dist + 1));
            }
        }

        nodes.push(Node {
            // name: name.to_string(),
            rate: lines[name].rate,
            dists,
        });
    }

    nodes
}

const N: usize = 16;

#[derive(Debug, Copy, Clone)]
struct State {
    position: usize,
    time: i64,
    is_opened: u64,
    total_flow: i64,
}

fn compute_states(nodes: &[Node], max_time: i64) -> Vec<State> {
    assert_eq!(nodes.len(), N);
    let initial_state = State {
        position: 0,
        time: 0,
        is_opened: 0,
        total_flow: 0,
    };

    let mut options = vec![initial_state];
    let mut index = 0;

    while let Some(&state) = options.get(index) {
        index += 1;

        for i in 0..N {
            let mask = 1 << i;

            if (state.is_opened) & mask == 0 {
                let new_time = state.time + nodes[state.position].dists[i] + 1;

                if new_time < max_time {
                    let mut new_state = state.clone();
                    new_state.position = i;
                    new_state.time = new_time;
                    new_state.is_opened |= mask;
                    new_state.total_flow += (max_time - new_time) * nodes[i].rate;
                    options.push(new_state);
                }
            }
        }
    }

    options
}

fn solve(nodes: &[Node], max_time: i64) -> i64 {
    compute_states(nodes, max_time)
        .iter()
        .map(|s| s.total_flow)
        .max()
        .unwrap()
}

fn solve_with_elephant(nodes: &[Node], max_time: i64) -> i64 {
    let mut best_states = HashMap::default();

    for state in compute_states(nodes, max_time) {
        match best_states.entry(state.is_opened) {
            Entry::Vacant(e) => {
                e.insert(state);
            }
            Entry::Occupied(mut e) if state.total_flow > e.get().total_flow => {
                e.insert(state);
            }
            _ => {}
        }
    }

    itertools::iproduct!(best_states.values(), best_states.values())
        .filter(|(a, b)| a.is_opened & b.is_opened == 0)
        .map(|(a, b)| a.total_flow + b.total_flow)
        .max()
        .unwrap()
}

pub(crate) fn run(lines: Lines) -> Result {
    let cave = parse_cave(lines);

    let flow = solve(&cave, 30);
    println!("part A: {}", flow);

    //2090 -> to low
    let flow = solve_with_elephant(&cave, 26);
    println!("part B: {}", flow);

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
