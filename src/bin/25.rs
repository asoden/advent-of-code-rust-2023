use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Edge {
    name: Rc<str>,
    start: Rc<str>,
    end: Rc<str>,
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
fn construct_graph(input: &str) -> HashMap<Rc<str>, Vec<Edge>> {
    let mut network: HashMap<Rc<str>, Vec<Edge>> = HashMap::new();

    input.lines().for_each(|line| {
        let (key, values) = line.split_once(": ").expect("Colon seperated line");
        let connections = values
            .split_whitespace()
            .collect_vec();

        // map edge connections of key to connections
        // connections are bidirectional and not duplicated in input

        for connection in connections.into_iter() {
            let name = format!("{key}-{connection}");
            let edge = Edge {
                start: key.into(),
                end: connection.into(),
                name: name.into(),
            };
            let node = network.entry(key.into()).or_default();
            node.push(edge.clone());
            let node = network.entry(connection.into()).or_default();
            node.push(edge.reverse());
        }
    });

    return network;
}

fn get_edge_to_remove(network: &HashMap<Rc<str>, Vec<Edge>>) -> Edge {
    let mut betweeness: HashMap<Rc<str>, (usize, Edge)> = HashMap::new();

    for node in network.keys() {
        let mut visited: HashSet<Rc<str>> = HashSet::new();
        let mut queue: VecDeque<(Rc<str>, Vec<Rc<str>>)> = VecDeque::from(vec![(node.clone(), vec![])]);
        while let Some((node, path)) = queue.pop_front() {
            visited.insert(node.clone());
            if let Some(edges) = network.get(&node) {
                for edge in edges.iter().filter(|edge| !visited.contains(&edge.end)) {
                    let mut next_path = path.clone();
                    next_path.push(edge.name.clone());
                    for sub_path in next_path.iter() {
                        betweeness
                            .entry(sub_path.clone())
                            .and_modify(|(x, _)| *x += 1)
                            .or_insert((1, edge.clone()));
                    }
                    queue.push_back((edge.end.clone(), next_path));
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

fn count_subgraphs(network: &HashMap<Rc<str>, Vec<Edge>>) -> (usize, usize) {
    //bfs ftw
    let mut visited = HashSet::new();
    // grab some node to start
    let start = network.keys().next().unwrap().clone();
    let mut queue = VecDeque::from([start]);

    while let Some(node) = queue.pop_front() {
        visited.insert(node.clone());

        if let Some(edges) = network.get(&node) {
            queue.append(
                &mut edges
                    .iter()
                    .filter_map(|edge| {
                        if visited.contains(&edge.end) {
                            None
                        } else {
                            Some(edge.end.clone())
                        }
                    })
                    .collect::<VecDeque<_>>(),
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

        if let Some(edges) = graph.get_mut(&removed_edge.start) {
            edges.retain(|edge| edge.name != removed_edge.name);
        }
        if let Some(edges) = graph.get_mut(&removed_edge.end) {
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
