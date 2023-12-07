use crate::common::*;
use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};

fn parse_edge(
    line: &str,
    graph: &mut DiGraph<String, usize>,
    nodes: &mut HashMap<String, NodeIndex>,
) -> Result {
    let mut parts = line.split(" bags contain ");
    let color = parts.next().unwrap_or_default().to_string();
    let rest = parts.next().unwrap_or_default();

    if rest.is_empty() || rest == "no other bags." {
        return Ok(());
    }

    let src = *nodes
        .entry(color.clone())
        .or_insert_with(|| graph.add_node(color));

    for p in find_all("([0-9]+) ([a-z ]+) bag[s]?[.,]?", rest) {
        let amount = p[1]
            .parse::<usize>()
            .with_context(|| format!("while parsing line {:?}", &p[0]))?;

        let color = p[2].to_string();
        let dst = *nodes
            .entry(color.clone())
            .or_insert_with(|| graph.add_node(color));

        graph.add_edge(src, dst, amount);
    }

    Ok(())
}

fn bfs(source: NodeIndex, graph: &DiGraph<String, usize>, visited: &mut HashSet<NodeIndex>) {
    visited.insert(source);

    for neighbor in graph.neighbors_directed(source, Direction::Incoming) {
        bfs(neighbor, graph, visited);
    }
}

fn count(source: NodeIndex, graph: &DiGraph<String, usize>) -> usize {
    let mut total = 1;

    for edge in graph.edges_directed(source, Direction::Outgoing) {
        total += edge.weight() * count(edge.target(), graph);
    }

    total
}

pub fn run() -> Result {
    let mut graph = DiGraph::new();
    let mut nodes = HashMap::new();

    for line in read_input("day07")? {
        parse_edge(&line, &mut graph, &mut nodes)?;
    }

    let mut visited = HashSet::new();
    bfs(nodes["shiny gold"], &graph, &mut visited);
    println!("part A: {}", visited.len() - 1);

    let count = count(nodes["shiny gold"], &graph);
    println!("part B: {}", count - 1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count() {
        let mut graph = DiGraph::new();
        let mut nodes = HashMap::new();
        let lines = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];

        for line in lines {
            parse_edge(line, &mut graph, &mut nodes).unwrap();
        }

        let c = count(nodes["shiny gold"], &graph);

        assert_eq!(c, 127);
    }
}
