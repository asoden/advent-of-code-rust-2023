use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Edge {
    name: String,
    start: String,
    end: String,
}

impl Edge {
    fn reverse(&self) -> Self {
        Self {
            start: self.end.clone(),
            end: self.start.clone(),
            name: self.name.clone(),
        }
    }
}
fn construct_graph(input: &str) -> HashMap<String, Vec<Edge>> {
    let mut network: HashMap<String, Vec<Edge>> = HashMap::new();

    input.lines().for_each(|line| {
        let (key, values) = line.split_once(": ").expect("Colon seperated line");
        let connections = values
            .split_whitespace()
            .map(|name| name.to_string())
            .collect_vec();

        // map edge connections of key to connections
        // connections are bidirectional and not duplicated in input

        for connection in connections.iter() {
            let name = format!("{key}-{connection}");
            let edge = Edge {
                start: key.to_string(),
                end: connection.clone(),
                name: name.clone(),
            };
            let node = network.entry(key.to_string()).or_default();
            node.push(edge.clone());
            let node = network.entry(connection.clone()).or_default();
            node.push(edge.reverse());
        }
    });

    return network;
}

fn get_edge_to_remove(network: &HashMap<String, Vec<Edge>>) -> Edge {
    let mut betweeness: HashMap<&String, (usize, Edge)> = HashMap::new();

    for node in network.keys() {
        let mut visited: HashSet<&String> = HashSet::new();
        let mut queue: VecDeque<(&String, Vec<&String>)> = VecDeque::from(vec![(node, vec![])]);
        while let Some((node, path)) = queue.pop_front() {
            visited.insert(node);
            if let Some(edges) = network.get(node) {
                for edge in edges.iter().filter(|edge| !visited.contains(&edge.end)) {
                    let mut next_path = path.clone();
                    next_path.push(&edge.name);
                    for sub_path in next_path.iter() {
                        betweeness
                            .entry(sub_path)
                            .and_modify(|(x, _)| *x += 1)
                            .or_insert((1, edge.clone()));
                    }
                    queue.push_back((&edge.end, next_path));
                }
            }
        }
    }

    betweeness
        .values()
        .max_by(|(x, _), (y, _)| x.cmp(y))
        .unwrap()
        .1
        .clone()
}

fn count_subgraphs(network: &HashMap<String, Vec<Edge>>) -> (usize, usize) {
    //bfs ftw
    let mut visited = HashSet::new();
    // grab some node to start
    let start = network.keys().next().unwrap();
    let mut queue = VecDeque::from([start]);

    while let Some(node) = queue.pop_front() {
        visited.insert(node);

        if let Some(edges) = network.get(node) {
            queue.extend(
                edges
                    .iter()
                    .filter_map(|edge| {
                        if visited.contains(&edge.end) {
                            None
                        } else {
                            Some(&edge.end)
                        }
                    })
                    .collect::<Vec<&String>>(),
            )
        }
    }

    let disconnected = network.len() - visited.len();
    (visited.len(), disconnected)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = construct_graph(input.trim_end());

    for _ in 0..3 {
        let removed_edge = get_edge_to_remove(&graph);

        if let Some(edges) = graph.get_mut(removed_edge.start.as_str()) {
            edges.retain(|edge| edge.name != removed_edge.name);
        }
        if let Some(edges) = graph.get_mut(removed_edge.end.as_str()) {
            edges.retain(|edge| edge.name != removed_edge.name);
        }
    }

    let (left, right) = count_subgraphs(&graph);

    Some(left * right)
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(1)
}

advent_of_code::main!(25);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 25));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 25));
        assert_eq!(result, Some(1));
    }
}
