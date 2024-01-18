use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

static UP: Point = Point::new(0, -1);
static DOWN: Point = Point::new(0, 1);
static LEFT: Point = Point::new(-1, 0);
static RIGHT: Point = Point::new(1, 0);

static DIRECTIONS: [Point; 4] = [UP, DOWN, LEFT, RIGHT];

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn neighbors(&self) -> Vec<Point> {
        DIRECTIONS.iter().map(|point| *self + *point).collect()
    }
}

type Edge = (Point, u32);

#[derive(Debug)]
struct Map {
    start: Point,
    end: Point,
    map: HashMap<Point, char>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        (
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            c,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashMap<_, _>>();

        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();
        let start = Point { x: 1, y: 0 };
        let end = Point {
            x: max_x - 1,
            y: max_y,
        };

        Self { map, start, end }
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        // if it is a slope it is a directed step and must go in that direction and we can short circuit
        match self.map.get(point).unwrap() {
            '>' => return vec![*point + RIGHT],
            '<' => return vec![*point + LEFT],
            'v' => return vec![*point + DOWN],
            '^' => return vec![*point + UP],
            _ => {}
        }

        let mut neighbors = Vec::new();

        for direction in DIRECTIONS.iter() {
            let step = *point + *direction;
            match self.map.get(&step) {
                None => continue,
                // we need to ensure we don't try to walk opposite a slope
                Some(c) => match (c, direction.x, direction.y) {
                    ('#', _, _) => continue,  // tree in the way
                    ('>', -1, 0) => continue, // moving left up rightward slope
                    ('<', 1, 0) => continue,  // moving right up a leftward slope
                    ('^', 0, 1) => continue,  // moving down a upward slope
                    ('v', 0, -1) => continue, // moving up a downward slope,
                    _ => neighbors.push(step),
                },
            }
        }

        return neighbors;
    }

    fn neighbors2(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();

        for step in point.neighbors() {
            match self.map.get(&step) {
                None => continue,
                // we need to ensure we don't try to walk opposite a slope
                Some(c) => match c {
                    '#' => continue, // tree in the way
                    _ => neighbors.push(step),
                },
            }
        }

        return neighbors;
    }

    fn longest_path(&self) -> Vec<u32> {
        // this will complete in a sane time as these directed slopes create a DAG for us!
        let mut ends = Vec::new();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.start, seen.clone(), 0));

        while let Some((point, mut seen, steps)) = queue.pop_front() {
            // are we at the end?
            if point == self.end {
                ends.push(steps);
                continue;
            }

            // have we been here before?
            if seen.contains(&point) {
                continue;
            }

            seen.insert(point);
            for neighbor in self.neighbors(&point).iter().filter(|n| !seen.contains(n)) {
                queue.push_back((*neighbor, seen.clone(), steps + 1));
            }
        }
        return ends;
    }

    fn find_contracted_edges(&self) -> HashMap<Point, Vec<Edge>> {
        // remap points from char to number of neighbors
        let map = self
            .map
            .iter()
            .filter(|(_, c)| **c != '#')
            .map(|(point, _)| {
                let n = self.neighbors2(point).len();
                (*point, n)
            })
            .collect::<HashMap<_, _>>();

        // make nodes from all points that do not have two neighbors
        // i.e. our start, end, and branching points
        let nodes = map
            .iter()
            .filter(|(_, n)| **n != 2)
            .map(|(point, _)| *point)
            .collect::<HashSet<_>>();

        // find the edges between our nodes with the length of walking to them as their weight
        let mut edges: HashMap<Point, Vec<Edge>> = HashMap::new();
        for node in nodes.iter() {
            for mut current in self.neighbors2(node) {
                let mut prev = *node;
                // keep track of how far we have walked
                let mut dist = 0;

                //walk the till we hit another node
                loop {
                    dist += 1;

                    // find the neighbors of our current
                    let neighbors = self.neighbors2(&current);
                    let neighbors = neighbors
                        .iter()
                        .filter(|point| **point != prev)
                        .collect::<Vec<_>>();

                    // if it is not 1 we have hit a branch or the end
                    if neighbors.len() != 1 {
                        edges.entry(*node).or_default().push((current, dist));
                        break;
                    }

                    // keep on walking otherwise
                    prev = current;
                    current = *neighbors[0];
                }
            }
        }

        return edges;
    }

    fn longest_path2(&self) -> Vec<u32> {
        let edges = self.find_contracted_edges();
        let mut ends = Vec::new();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((self.start, seen.clone(), 0));

        while let Some((point, mut seen, steps)) = queue.pop_front() {
            // are we at the end?
            if point == self.end {
                ends.push(steps);
                continue;
            }

            // have we been here before?
            if seen.contains(&point) {
                continue;
            }

            seen.insert(point);
            for (neighbor, weight) in edges
                .get(&point)
                .unwrap()
                .iter()
                .filter(|edge| !seen.contains(&edge.0))
            {
                queue.push_back((*neighbor, seen.clone(), steps + weight));
            }
        }
        return ends;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input.trim_end());

    let paths = map.longest_path();

    paths.into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input.trim_end());

    let paths = map.longest_path2();

    paths.into_iter().max()
}

advent_of_code::main!(23);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 23));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 23));
        assert_eq!(result, Some(154));
    }
}
