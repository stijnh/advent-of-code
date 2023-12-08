use crate::common::*;
use num::integer::lcm;

struct Graph<'a> {
    nodes: Vec<&'a str>,
    edges: Vec<(usize, usize)>,
}

fn parse_graph<'a>(lines: &[&'a str]) -> Graph<'a> {
    let nodes = lines.iter().map(|l| &l[..3]).collect_vec();
    let edges = lines
        .iter()
        .map(|line| {
            (
                nodes.iter().position(|&n| n == &line[7..10]).unwrap(),
                nodes.iter().position(|&n| n == &line[12..15]).unwrap(),
            )
        })
        .collect_vec();

    Graph { nodes, edges }
}

fn navigate(directions: &[char], g: &Graph) -> usize {
    let mut current = g.nodes.iter().position(|&x| x == "AAA").unwrap();
    let end = g.nodes.iter().position(|&x| x == "ZZZ").unwrap();

    let mut steps = 0;
    while current != end {
        let dir = directions[steps % directions.len()];

        if dir == 'L' {
            current = g.edges[current].0;
        } else {
            current = g.edges[current].1;
        }

        steps += 1;
    }
    steps
}

fn navigate_ghost(directions: &[char], g: &Graph) -> usize {
    let mut result = 1;

    for (index, node) in enumerate(&g.nodes) {
        if !node.ends_with('A') {
            continue;
        }

        let mut steps = 0;
        let mut visited = HashSet::default();
        let mut path = vec![];
        let mut current = index;

        while visited.insert((current, steps)) {
            path.push((current, steps));

            current = if directions[steps] == 'L' {
                g.edges[current].0
            } else {
                g.edges[current].1
            };

            steps = (steps + 1) % directions.len();
        }

        let length = path.len() - steps;

        // It seems to be always that the node at path.len() - steps ends with Z
        let last_node = g.nodes[path[length].0];
        assert!(last_node.ends_with('Z'));

        result = lcm(result, length);
    }

    result
}

pub(crate) fn run(lines: Lines) -> Result {
    let directions = lines[0].chars().collect_vec();
    let graph = parse_graph(&lines[2..]);

    let steps = navigate(&directions, &graph);
    println!("part A: {}", steps);

    let steps = navigate_ghost(&directions, &graph);
    println!("part B: {}", steps);

    Ok(())
}
