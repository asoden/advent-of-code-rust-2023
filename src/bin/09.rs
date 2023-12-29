use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i64> {
    let readings: Vec<Vec<i64>> = input
        .trim_end()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| {
                    num.parse::<i64>()
                        .expect("Space seperated numbers expected.")
                })
                .collect()
        })
        .collect();

    let val = readings
        .iter()
        .map(|reading| {
            let mut prediction = 0;
            let mut sub_level = reading.clone();
            while !sub_level.iter().all(|&value| value == 0) {
                prediction += sub_level.last().unwrap();
                sub_level = sub_level
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
            }
            prediction
        })
        .sum();
    Some(val)
}

pub fn part_two(input: &str) -> Option<i64> {
    let readings: Vec<Vec<i64>> = input
        .trim_end()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| {
                    num.parse::<i64>()
                        .expect("Space seperated numbers expected.")
                })
                .collect()
        })
        .collect();

    let val = readings
        .iter()
        .map(|reading| {
            let mut prediction = 0;
            let mut sub_level = reading.clone();
            sub_level.reverse(); // what is the front but the reversed last
            while !sub_level.iter().all(|&value| value == 0) {
                prediction += sub_level.last().unwrap();
                sub_level = sub_level
                    .into_iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect();
            }
            prediction
        })
        .sum();
    Some(val)
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(2));
    }
}
