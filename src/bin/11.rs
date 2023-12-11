use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn taxicab_distance(point0: Point, point1: Point) -> usize {
    point0.x.abs_diff(point1.x) + point0.y.abs_diff(point1.y)
}

fn galaxy_distance_sums(input: &str, expansion_factor: usize) -> usize {

    let mut y_set = HashSet::new();
    let mut x_set = HashSet::new();
    let mut galaxies = Vec::new();
    let lines: Vec<_> = input.trim_end().lines().collect();
    let x_length = lines[0].len();
    let y_length = lines.len();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().into_iter().enumerate() {
            if char == '#' {
                x_set.insert(x);
                y_set.insert(y);
                galaxies.push(Point { x, y })
            }
        }
    }

    let mut x_sum = Vec::with_capacity(x_length);
    let mut y_sum = Vec::with_capacity(y_length);

    let mut sum: usize = 0;
    (0..x_length).for_each(|i| {
        if !x_set.contains(&i) {
            sum += 1;
        }
        x_sum.push(sum);
    });

    sum = 0;
    (0..y_length).for_each(|i| {
        if !y_set.contains(&i) {
            sum += 1;
        }
        y_sum.push(sum);
    });

    sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxy0 = &galaxies[i];
            let galaxy1 = &galaxies[j];
            let point0 = Point {
                x: galaxy0.x + (x_sum[galaxy0.x] * (expansion_factor - 1)),
                y: galaxy0.y + (y_sum[galaxy0.y] * (expansion_factor - 1)),
            };
            let point1 = Point {
                x: galaxy1.x + (x_sum[galaxy1.x] * (expansion_factor - 1)),
                y: galaxy1.y + (y_sum[galaxy1.y] * (expansion_factor - 1)),
            };
            sum += taxicab_distance(point0, point1);
        }
    }

    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(galaxy_distance_sums(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(galaxy_distance_sums(input, 1000000))
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(82000210));
    }
}
