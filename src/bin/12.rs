use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<u32>,
}

impl Record {
    fn new(input: &str) -> Self {
        let (springs, counts) = input.split_once(" ").expect("Space delimited line value.");
        let mut springs = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("Invalid spring character in input stream: {c}"),
            })
            .collect_vec();

        // simplify Damaged recursion case
        springs.push(Spring::Operational);

        let counts = counts.split(",").map(|c| c.parse().unwrap()).collect_vec();

        Self { springs, counts }
    }

    fn newewewewew(input: &str) -> Self {
        let (springs, counts) = input.split_once(" ").expect("Space delimited line value.");
        let mut springs = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("Invalid spring character in input stream: {c}"),
            })
            .collect_vec();

        let mut counts = counts.split(",").map(|c| c.parse().unwrap()).collect_vec();

        // five times as much apparently
        springs = springs
            .iter()
            .copied()
            .chain([Spring::Unknown])
            .cycle()
            .take(springs.len() * 5 + 4)
            .collect();

        // simplify Damaged recursion case
        springs.push(Spring::Operational);

        counts = counts
            .iter()
            .copied()
            .cycle()
            .take(counts.len() * 5)
            .collect();

        Self { springs, counts }
    }

    fn _is_valid(&self) -> bool {
        self.springs
            .iter()
            .group_by(|&item| item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Spring::Damaged {
                    Some(group.count() as u32)
                } else {
                    None
                }
            })
            .eq(self.counts.iter().copied())
    }

    fn find_valid_configurations(&self) -> u64 {
        let mut cache = vec![vec![None; self.springs.len()]; self.counts.len()];
        count_possible_arrangements(&self.springs, &self.counts, &mut cache)
    }
}

fn count_possible_arrangements(
    springs: &[Spring],
    counts: &[u32],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    // quick cases out of the way first
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            // rest of springs are damaged out of count
            0
        } else {
            // remaining springs are good
            1
        };
    }
    if springs.len() < (counts.iter().sum::<u32>() as usize + counts.len()) {
        // not enough space for remaining counts
        return 0;
    }

    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }

    let mut arrangements = 0;
    if springs[0] != Spring::Damaged {
        // assume operational
        arrangements += count_possible_arrangements(&springs[1..], counts, cache);
    }

    let group_size = counts[0] as usize;

    if !springs[..group_size].contains(&Spring::Operational)
        && springs[group_size] != Spring::Damaged
    {
        // assume damaged
        arrangements +=
            count_possible_arrangements(&springs[group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arrangements);
    arrangements
}

pub fn part_one(input: &str) -> Option<u64> {
    let records = input.trim_end().lines().map(Record::new).collect_vec();
    Some(
        records
            .par_iter()
            .map(|record| record.find_valid_configurations())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let records = input
        .trim_end()
        .lines()
        .map(Record::newewewewew)
        .collect_vec();

    Some(
        records
            .par_iter()
            .map(|record| record.find_valid_configurations())
            .sum(),
    )
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(525152));
    }
}
