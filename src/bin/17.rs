use pathfinding::matrix::Matrix;
use pathfinding::prelude::dijkstra;

fn make_grid(input: &str) -> Matrix<u32> {
    Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().filter_map(|character| character.to_digit(10))),
    )
    .expect("Parsable matrix of heat values expected.")
}

fn move_crucible(grid: &Matrix<u32>, min_move: usize, max_move: usize) -> u32 {
    dijkstra(
        &((0, 0), (0, 0), 0),
        |&(position, (horizontal_delta, vertical_delta), length)| {
            let mut successors = Vec::new();
            let mut build_successors = |direction, length| {
                if let Some(space) = grid.move_in_direction(position, direction) {
                    successors.push(((space, direction, length), grid[space]));
                }
            };

            if length < max_move {
                // check straight
                build_successors((horizontal_delta, vertical_delta), length + 1);
            }

            if length >= min_move {
                // check turns
                build_successors((-vertical_delta, -horizontal_delta), 1);
                build_successors((vertical_delta, horizontal_delta), 1);
            } else if length == 0 {
                // first space check right and down
                build_successors((1, 0), 1);
                build_successors((0, 1), 1);
            }

            successors
        },
        |&(position, _, length)| {
            position == (grid.rows - 1, grid.columns - 1) && length >= min_move
        },
    )
    .unwrap()
    .1
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input.trim_end());

    Some(move_crucible(&grid, 1, 3))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = make_grid(input.trim_end());

    Some(move_crucible(&grid, 4, 10))
}

advent_of_code::main!(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(94));
    }
}
