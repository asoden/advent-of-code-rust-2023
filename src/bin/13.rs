use rayon::prelude::*;

struct Reflection {
    vertical: Option<u32>,
    horizontal: Option<u32>,
}

fn get_column(grid: &Vec<Vec<char>>, column: usize) -> String {
    grid.iter().map(|row| row[column]).collect()
}

fn get_row(grid: &Vec<Vec<char>>, row: usize) -> String {
    grid[row].iter().collect()
}

fn diff_strings(str0: &str, str1: &str) -> u32 {
    str0.chars().zip(str1.chars()).filter(|&(c0, c1)| c0 != c1).count() as u32
}

fn find_reflections(grid: &Vec<Vec<char>>) -> Reflection {
    let row_length = grid[0].len();
    for column in 1..row_length {
        let mut left = column as i32 - 1;
        let mut right = column;
        let mut is_valid = true;
        while left >= 0 && right < row_length {
            if get_column(grid, left as usize) != get_column(grid, right) {
                is_valid = false;
                break;
            }
            left -= 1;
            right += 1;
        }
        if is_valid {
            return Reflection {
                horizontal: None,
                vertical: Some(column as u32),
            };
        }
    }

    for row in 1..grid.len() {
        let mut up = row as i32 - 1;
        let mut down = row;
        let mut is_valid = true;
        while up >= 0 && down < grid.len() {
            if get_row(grid, up as usize) != get_row(grid, down) {
                is_valid = false;
                break;
            }
            up -= 1;
            down += 1;
        }
        if is_valid {
            return Reflection {
                horizontal: Some(row as u32),
                vertical: None,
            };
        }
    }

    Reflection {
        horizontal: None,
        vertical: None,
    }
}

fn find_smudge_reflections(grid: &Vec<Vec<char>>) -> Reflection {
    let row_length = grid[0].len();
    for column in 1..row_length {
        let mut left = column as i32 - 1;
        let mut right = column;
        let mut diffs = 0;
        while left >= 0 && right < row_length {
            if diffs > 1 {
                break;
            }
            diffs += diff_strings(&get_column(grid, left as usize), &get_column(grid, right));
            left -= 1;
            right += 1;
        }
        if diffs == 1 {
            return Reflection {
                horizontal: None,
                vertical: Some(column as u32),
            };
        }
    }

    for row in 1..grid.len() {
        let mut up = row as i32 - 1;
        let mut down = row;
        let mut diffs = 0;
        while up >= 0 && down < grid.len() {
            if diffs > 1 {
                break;
            }
            diffs += diff_strings(&get_row(grid, up as usize), &get_row(grid, down));
            up -= 1;
            down += 1;
        }
        if diffs == 1 {
            return Reflection {
                horizontal: Some(row as u32),
                vertical: None,
            };
        }
    }

    Reflection {
        horizontal: None,
        vertical: None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let raw_grids = input.trim_end().split("\n\n").collect::<Vec<&str>>();
    let grids: Vec<Vec<Vec<char>>> = raw_grids
        .into_par_iter()
        .map(|grid| {
            grid.lines()
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect();

    let sum: u32 = grids
        .iter()
        .map(|grid| {
            let result = find_reflections(grid);
            match (result.horizontal, result.vertical) {
                (Some(horizontal), None) => 100 * horizontal,
                (None, Some(vertical)) => vertical,
                _ => panic!("Got horizontal and vertical somehow!"),
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let raw_grids = input.trim_end().split("\n\n").collect::<Vec<&str>>();
    let grids: Vec<Vec<Vec<char>>> = raw_grids
        .into_par_iter()
        .map(|grid| {
            grid.lines()
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect();

    let sum: u32 = grids
        .iter()
        .map(|grid| {
            let result = find_smudge_reflections(grid);
            match (result.horizontal, result.vertical) {
                (Some(horizontal), None) => 100 * horizontal,
                (None, Some(vertical)) => vertical,
                _ => panic!("Got horizontal and vertical somehow!"),
            }
        })
        .sum();

    Some(sum)
}

advent_of_code::main!(13);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, Some(400));
    }
}
