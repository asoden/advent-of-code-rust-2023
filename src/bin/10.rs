use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

struct Grid {
    width: i32,
    height: i32,
    bytes: Vec<Vec<u8>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let rows: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        let width: i32 = rows[0].len() as i32;
        let height: i32 = rows.len() as i32;

        Self {
            height,
            width,
            bytes: rows,
        }
    }

    fn find_start(&self) -> (i32, i32) {
        for (y, row) in self.bytes.iter().enumerate() {
            for (x, char) in row.iter().enumerate() {
                if *char == b'S' {
                    return (x as i32, y as i32);
                }
            }
        }
        (0, 0)
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        self.bytes[y as usize][x as usize]
    }

    fn get_adjacent(&self, x: i32, y: i32) -> impl Iterator<Item = (u8, i32, i32)> + '_ {
        let width = self.width;
        let height = self.height;
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter(move |(delta_x, delta_y)| {
                x + delta_x >= 0 && x + delta_x < width && y + delta_y >= 0 && y + delta_y < height
            })
            .map(move |(delta_x, delta_y)| {
                let new_x = x + delta_x;
                let new_y = y + delta_y;
                (self.get(new_x, new_y), delta_x, delta_y)
            })
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = Grid::new(input);

    let start = grid.find_start();
    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut location_stack = vec![start];

    while let Some(current_location) = location_stack.pop() {
        let distance = distances[&current_location];
        let (x, y) = current_location;
        let current_symbol = grid.get(x, y);
        for (adjacent, delta_x, delta_y) in grid.get_adjacent(x, y) {
            match (adjacent, current_symbol, delta_x, delta_y) {
                (b'|', b'7' | b'F' | b'|' | b'S', 0, 1)
                | (b'|', b'L' | b'J' | b'|' | b'S', 0, -1)
                | (b'-', b'F' | b'L' | b'-' | b'S', 1, 0)
                | (b'-', b'7' | b'J' | b'-' | b'S', -1, 0)
                | (b'L', b'7' | b'J' | b'-' | b'S', -1, 0)
                | (b'L', b'F' | b'7' | b'|' | b'S', 0, 1)
                | (b'J', b'L' | b'F' | b'-' | b'S', 1, 0)
                | (b'J', b'7' | b'F' | b'|' | b'S', 0, 1)
                | (b'7', b'L' | b'F' | b'-' | b'S', 1, 0)
                | (b'7', b'L' | b'J' | b'|' | b'S', 0, -1)
                | (b'F', b'J' | b'7' | b'-' | b'S', -1, 0)
                | (b'F', b'L' | b'J' | b'|' | b'S', 0, -1) => {
                    let new_x = x + delta_x;
                    let new_y = y + delta_y;
                    let step = distance + 1;
                    match distances.entry((new_x, new_y)) {
                        Entry::Vacant(item) => {
                            item.insert(step);
                            location_stack.push((new_x, new_y));
                        }
                        Entry::Occupied(mut item) => {
                            if step < *item.get_mut() {
                                item.insert(step);
                                location_stack.push((new_x, new_y));
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    Some(*distances.values().max().unwrap())
}

pub fn part_two(input: &str) -> Option<i32> {
    let grid = Grid::new(input);

    let start = grid.find_start();
    let mut visited = HashSet::new();
    let mut perimiter = vec![start];
    visited.insert(start);

    let mut location_stack = vec![start];

    while let Some(current_location) = location_stack.pop() {
        let (x, y) = current_location;
        let current_symbol = grid.get(x, y);
        for (adjacent, delta_x, delta_y) in grid.get_adjacent(x, y) {
            match (adjacent, current_symbol, delta_x, delta_y) {
                (b'|', b'7' | b'F' | b'|', 0, 1) |
                (b'|', b'L' | b'J' | b'|' | b'S', 0, -1) |  // make work for more than my input later
                (b'-', b'F' | b'L' | b'-', 1, 0) |
                (b'-', b'7' | b'J' | b'-', -1, 0) |
                (b'L', b'7' | b'J' | b'-', -1, 0) |
                (b'L', b'F' | b'7' | b'|', 0, 1) |
                (b'J', b'L' | b'F' | b'-', 1, 0) |
                (b'J', b'7' | b'F' | b'|', 0, 1) |
                (b'7', b'L' | b'F' | b'-', 1, 0) |
                (b'7', b'L' | b'J' | b'|', 0, -1) |
                (b'F', b'J' | b'7' | b'-', -1, 0) |
                (b'F', b'L' | b'J' | b'|', 0, -1) => {
                    let new_x = x + delta_x;
                    let new_y = y + delta_y;

                    if visited.insert((new_x, new_y)) {
                        location_stack.push((new_x, new_y));
                        perimiter.push((new_x, new_y));
                    }
                }
                _ => ()
            }
        }
    }

    let num_points = perimiter.len() as i32;

    //seal up perimeter of points
    perimiter.push(start);

    let mut reg_poly_area = 0;

    for ((x_1, y_1), (x_2, y_2)) in perimiter.into_iter().tuple_windows() {
        reg_poly_area += (x_1 - x_2) * (y_2 + y_1);
    }

    Some(reg_poly_area.abs() / 2 - (num_points / 2 - 1))
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, Some(10));
    }
}
