use fxhash::FxHashMap;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Space {
    Rounded,
    Cubed,
    Empty,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Platform {
    grid: Vec<Vec<Space>>,
    height: usize,
    width: usize,
}

impl Platform {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<Space>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '#' => Space::Cubed,
                        'O' => Space::Rounded,
                        _ => Space::Empty,
                    })
                    .collect()
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Platform {
            grid,
            height,
            width,
        }
    }

    fn set_grid(&mut self, new_grid: &Vec<Vec<Space>>) {
        if new_grid.len() != self.height && new_grid[0].len() != self.width {
            return;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                self.grid[y][x] = new_grid[y][x];
            }
        }
    }

    fn slide_north(&mut self) {
        for x in 0..self.width {
            let mut fixed = 0;
            for y in 0..self.height {
                match self.grid[y][x] {
                    Space::Rounded => {
                        if y > fixed {
                            self.grid[y][x] = Space::Empty;
                            self.grid[fixed][x] = Space::Rounded;
                        }
                        fixed += 1;
                    }
                    Space::Cubed => fixed = y + 1,
                    _ => (),
                }
            }
        }
    }

    fn slide_west(&mut self) {
        for y in 0..self.height {
            let mut fixed = 0;
            for x in 0..self.width {
                match self.grid[y][x] {
                    Space::Rounded => {
                        if x > fixed {
                            self.grid[y][x] = Space::Empty;
                            self.grid[y][fixed] = Space::Rounded;
                        }
                        fixed += 1;
                    }
                    Space::Cubed => fixed = x + 1,
                    _ => (),
                }
            }
        }
    }

    fn slide_south(&mut self) {
        for x in 0..self.width {
            let mut fixed: i32 = self.height as i32 - 1;
            for y in (0..self.height).rev() {
                match self.grid[y][x] {
                    Space::Rounded => {
                        if (y as i32) < fixed {
                            self.grid[y][x] = Space::Empty;
                            self.grid[fixed as usize][x] = Space::Rounded;
                        }
                        fixed -= 1;
                    }
                    Space::Cubed => fixed = y as i32 - 1,
                    _ => (),
                }
            }
        }
    }

    fn slide_east(&mut self) {
        for y in 0..self.height {
            let mut fixed: i32 = self.width as i32 - 1;
            for x in (0..self.width).rev() {
                match self.grid[y][x] {
                    Space::Rounded => {
                        if (x as i32) < fixed {
                            self.grid[y][x] = Space::Empty;
                            self.grid[y][fixed as usize] = Space::Rounded;
                        }
                        fixed -= 1;
                    }
                    Space::Cubed => fixed = x as i32 - 1,
                    _ => (),
                }
            }
        }
    }

    fn load(&self) -> u32 {
        let mut load = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                match self.grid[y][x] {
                    Space::Rounded => load += self.height - y,
                    _ => (),
                }
            }
        }

        load as u32
    }
}

fn load(grid: &Vec<Vec<Space>>) -> u32 {
    let mut load = 0;

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            match grid[y][x] {
                Space::Rounded => load += grid.len() - y,
                _ => (),
            }
        }
    }

    load as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut platform = Platform::new(input);

    platform.slide_north();
    Some(platform.load())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut platform = Platform::new(input);

    // lmao for teh lols
    // for _ in 0..1000000000 {
    //     platform.slide_north();
    //     platform.slide_west();
    //     platform.slide_south();
    //     platform.slide_east()
    // }

    let mut seen_grid = FxHashMap::default();
    seen_grid.insert(platform.grid.clone(), 0);

    let (cycle_start, cycle_end) = loop {
        platform.slide_north();
        platform.slide_west();
        platform.slide_south();
        platform.slide_east();

        if seen_grid.contains_key(&*platform.grid.clone()){
            break (seen_grid.get(&*platform.grid.clone()).unwrap(), seen_grid.len())
        }
        seen_grid.insert(platform.grid.clone(), seen_grid.len());
    };

    let offset = 1_000_000_000 - cycle_start;
    let cycle_length = cycle_end - cycle_start;
    let remainder = offset % cycle_length;
    let final_index = cycle_start + remainder;

    let (grid, _) = seen_grid
        .iter()
        .find(|(_, &i)| {
            i == final_index
        })
        .unwrap();

    Some(load(grid))
}

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(64));
    }
}
