use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::*;

fn parse(input: &str) -> (Vec<char>, FxHashMap<&str, (&str, &str)>) {
    let (instruction_string, direction_strings) = input
        .trim_end()
        .split_once("\n\n")
        .expect("Input is delimited by two new lines.");
    let instructions = instruction_string.chars().collect_vec();
    let mut routes = FxHashMap::default();

    direction_strings.lines().for_each(|line| {
        let (key, forks) = line.split_once(" = ").expect("Line entry split by ' = '");
        let (left, right) = forks
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .expect("Delimited by ', '");
        routes.insert(key, (left, right));
    });

    return (instructions, routes);
}

// Euclid was a genius so let's use that
fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

// get least common multiple by relationship of greatest common denominator
fn lcm(a: u64, b: u64) -> u64 {
    return (a * b) / gcd(a, b);
}

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, routes) = parse(input);

    let mut current = "AAA";
    let mut count: u64 = 0;

    loop {
        if current == "ZZZ" {
            break;
        }
        let index = count as usize % instructions.len();
        let direction = instructions[index];

        let route = routes.get(current).expect("All routes closed together.");

        current = if direction == 'L' { route.0 } else { route.1 };

        count += 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, routes) = parse(input);

    let starts: Vec<_> = routes
        .keys()
        .copied()
        .filter(|&label| label.ends_with('A'))
        .collect();

    let cycle = starts
        .par_iter()
        .map(|&start| {
            let mut count: u64 = 0;
            let mut current = start;

            loop {
                if current.ends_with('Z') {
                    break count;
                }
                let index = count as usize % instructions.len();
                let direction = instructions[index];

                let route = routes.get(current).expect("All routes closed together.");

                current = if direction == 'L' { route.0 } else { route.1 };

                count += 1;
            }
        })
        .reduce(|| 1, |acc, e| lcm(acc, e));

    Some(cycle)
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(2));
    }
}
