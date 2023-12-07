use std::collections::HashSet;

struct Card {
    matches: Vec<u32>,
}

impl Card {
    pub fn new(input: &str) -> Self {
        let (_, numbers) = input
            .split_once(":")
            .expect("Colon between numbers and card");

        let (winners, entries) = numbers
            .split_once(" | ")
            .expect("Pipe between winning and entries");

        let wins: HashSet<u32> = winners
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();

        let attempts: HashSet<u32> = entries
            .split_whitespace()
            .map(|str| str.parse().unwrap())
            .collect();

        Self {
            matches: wins.intersection(&attempts).cloned().collect(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(Card::new)
            .map(|card| {
                return if card.matches.len() < 1 {
                    0
                } else {
                    2_u32.pow((card.matches.len() - 1) as u32)
                };
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input.trim().lines().map(Card::new).collect();
    let mut counts: Vec<u32> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let update_to = if card.matches.len() > counts.len() {
            counts.len()
        } else {
            card.matches.len() + i
        };

        for j in i + 1..=update_to {
            counts[j] += counts[i];
        }
    }
    Some(counts.iter().sum())
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(30));
    }
}
