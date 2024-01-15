use std::collections::HashSet;

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input
            .trim_end()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }

    fn find_start(&self) -> (i32, i32) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, symbol) in row.iter().enumerate() {
                if *symbol == 'S' {
                    return (x as i32, y as i32);
                }
            }
        }
        (0, 0)
    }

    fn get_adjacent(&self, x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            // no need to bounds check our problem space is only up to the borders
            .into_iter()
            .map(move |(delta_x, delta_y)| {
                let new_x = x + delta_x;
                let new_y = y + delta_y;
                (new_x, new_y)
            })
    }
}

fn walk_grid(grid: &Grid, steps: usize) -> usize {
    let mut visited = HashSet::new();

    let mut start = grid.find_start();
    let width = grid.width;
    let height = grid.height;

    // remove possibility of dealing with negatives
    start.0 += (((steps + width - 1) / width) * width) as i32;
    start.1 += (((steps + height - 1) / height) * height) as i32;

    visited.insert(start);

    for _ in 0..steps {
        let mut visit_queue = HashSet::new();
        for position in visited.iter().clone() {
            for neighbor in grid.get_adjacent(position.0, position.1) {
                match grid.grid[neighbor.0 as usize % width][neighbor.1 as usize % height] {
                    '.' | 'S' => {
                        visit_queue.insert(neighbor);
                    }
                    _ => (),
                }
            }
        }
        visited = visit_queue;
    }
    visited.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input.trim_end());

    let possibilities = walk_grid(&grid, 64);
    Some(possibilities)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::new(input.trim_end());

    // insights:
    // we reach the outer bound of the first grid at 65 steps (grid height / 2)
    // due to the shape of the input this gives rise to a quadratic sequence everytime we reach the border of a grid
    // the puzzle author is cheeky and dividing the requested steps by the grid height gives us the year * 100 (202300) remainder 65
    // this puts the requested steps on the outer bound of a grid thus in our observed quadratic sequence
    // we can determine the desired step count by finding the constants a,b, and c for an^2 + bn + c.

    let mut tiles = Vec::with_capacity(3);
    for i in 0..3 {
        let steps = grid.height / 2 + grid.height * i;

        tiles.push(walk_grid(&grid, steps));
    }

    let a = (tiles[2] - 2 * tiles[1] + tiles[0]) / 2;
    let b = tiles[1] - tiles[0] - a;
    let c = tiles[0];
    let n = 202300;

    Some((a * n * n) + (b * n) + c)
}

advent_of_code::main!(21);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 21));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 21));
        assert_eq!(result, None);
    }
}
