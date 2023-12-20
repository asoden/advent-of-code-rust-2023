use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Laser {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Laser {
    fn advance(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .trim_end()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Grid { grid }
    }

    fn in_bounds(&self, laser: &Laser) -> bool {
        laser.x >= 0
            && laser.y >= 0
            && laser.x < self.grid[0].len() as i32
            && laser.y < self.grid.len() as i32
    }

    fn energize(&self, laser: Laser) -> usize {
        let mut energized = HashSet::new();
        let mut laser_states = HashSet::new();

        // lasers in progress
        let mut pews = Vec::new();
        pews.push(laser);

        while let Some(mut laser) = pews.pop() {
            while self.in_bounds(&laser) && !laser_states.contains(&laser) {
                energized.insert((laser.x.clone(), laser.y.clone()));
                laser_states.insert(laser.clone());

                match (
                    self.grid[laser.y as usize][laser.x as usize],
                    &laser.direction,
                ) {
                    ('|', Direction::Left | Direction::Right) => {
                        // make a copy split going up
                        let mut up = laser.clone();
                        up.direction = Direction::Up;
                        up.advance();
                        pews.push(up);

                        laser.direction = Direction::Down;
                    }
                    ('-', Direction::Up | Direction::Down) => {
                        // make a copy split going left
                        let mut left = laser.clone();
                        left.direction = Direction::Left;
                        left.advance();
                        pews.push(left);

                        laser.direction = Direction::Right;
                    }
                    ('/', Direction::Up) => laser.direction = Direction::Right,
                    ('/', Direction::Down) => laser.direction = Direction::Left,
                    ('/', Direction::Right) => laser.direction = Direction::Up,
                    ('/', Direction::Left) => laser.direction = Direction::Down,
                    ('\\', Direction::Up) => laser.direction = Direction::Left,
                    ('\\', Direction::Down) => laser.direction = Direction::Right,
                    ('\\', Direction::Right) => laser.direction = Direction::Down,
                    ('\\', Direction::Left) => laser.direction = Direction::Up,
                    _ => (), // move forward
                }
                laser.advance();
            }
        }
        energized.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let starting_laser = Laser {
        x: 0,
        y: 0,
        direction: Direction::Right,
    };

    let grid = Grid::new(input.trim_end());

    Some(grid.energize(starting_laser))
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::new(input.trim_end());
    let mut lasers = Vec::new();
    for x in 0..grid.grid[0].len() {
        lasers.push(Laser {
            x: x as i32,
            y: 0,
            direction: Direction::Down,
        });
        lasers.push(Laser {
            x: x as i32,
            y: grid.grid.len() as i32 - 1,
            direction: Direction::Up,
        })
    }

    for y in 0..grid.grid.len() {
        lasers.push(Laser {
            x: 0,
            y: y as i32,
            direction: Direction::Right,
        });
        lasers.push(Laser {
            x: grid.grid[0].len() as i32,
            y: y as i32,
            direction: Direction::Left,
        });
    }

    lasers.par_iter().map(|laser| grid.energize(*laser)).max()
}

advent_of_code::main!(16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, Some(51));
    }
}
