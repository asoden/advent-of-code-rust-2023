use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    magnitude: u64,
}

#[derive(Clone)]
struct Coords {
    x: i64,
    y: i64,
}

fn trace_route(instructions: &Vec<Instruction>) -> Vec<Coords> {
    let mut perimeter: Vec<Coords> = Vec::new();
    let mut x = 0;
    let mut y = 0;

    for instruction in instructions {
        match instruction.direction {
            Direction::Up => y -= instruction.magnitude as i64,
            Direction::Down => y += instruction.magnitude as i64,
            Direction::Left => x -= instruction.magnitude as i64,
            Direction::Right => x += instruction.magnitude as i64,
        }

        perimeter.push(Coords { x, y })
    }

    perimeter
}

pub fn part_one(input: &str) -> Option<i64> {
    let instructions: Vec<Instruction> = input
        .trim_end()
        .lines()
        .map(|line| {
            let direction_color_split = line.split_once(" (#").expect("color input");
            let direction_split = direction_color_split
                .0
                .split_once(" ")
                .expect("space seperated");
            let direction = match direction_split.0 {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("invalid character"),
            };
            let magnitude = direction_split.1.parse().expect("number");
            Instruction {
                magnitude,
                direction,
            }
        })
        .collect();

    let mut route_coords = trace_route(&instructions);

    let perimeter: i64 = instructions.iter().map(|instruction| instruction.magnitude as i64).sum();

    let front_copy = route_coords[0].clone();
    //seal up perimeter of points
    route_coords.push(front_copy);

    let mut reg_poly_area = 0;

    for (coord1, coord2) in route_coords.into_iter().tuple_windows() {
        reg_poly_area += (coord1.x - coord2.x) * (coord2.y + coord1.y);
    }

    Some((reg_poly_area.abs() + perimeter) / 2 + 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let instructions: Vec<Instruction> = input
        .trim_end()
        .lines()
        .map(|line| {
            let direction_why_are_elves_incompetent_split =
                line.split_once(" (#").expect("color input");
            let color = direction_why_are_elves_incompetent_split.1;
            let direction = match color.chars().nth(5).expect("6 digit number") {
                '3' => Direction::Up,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '0' => Direction::Right,
                _ => panic!("invalid character"),
            };
            let magnitude = color.get(0..5).expect("6 digit hex number");
            Instruction {
                magnitude: u64::from_str_radix(magnitude, 16).expect("real number"),
                direction,
            }
        })
        .collect();

    let mut route_coords = trace_route(&instructions);

    let perimeter: i64 = instructions.iter().map(|instruction| instruction.magnitude as i64).sum();

    let front_copy = route_coords[0].clone();
    //seal up perimeter of points
    route_coords.push(front_copy);

    let mut reg_poly_area = 0;

    for (coord1, coord2) in route_coords.into_iter().tuple_windows() {
        reg_poly_area += (coord1.x - coord2.x) * (coord2.y + coord1.y);
    }

    Some((reg_poly_area.abs() + perimeter) / 2 + 1)
}

advent_of_code::main!(18);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, Some(952408144115));
    }
}
