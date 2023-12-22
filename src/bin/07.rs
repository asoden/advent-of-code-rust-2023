use fxhash::FxHashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct CamelHand {
    outcome: HandType,
    cards: [u8; 5],
    bid: u32,
}

impl CamelHand {
    fn new(input: &str) -> Self {
        let (raw_cards, bid) = input.split_once(" ").expect("space delimited");
        let bid: u32 = bid.parse().expect("Number value");

        let mut cards = [0; 5];

        for (i, &card) in raw_cards.as_bytes().iter().enumerate() {
            match card {
                b'A' => cards[i] = 14,
                b'K' => cards[i] = 13,
                b'Q' => cards[i] = 12,
                b'J' => cards[i] = 11,
                b'T' => cards[i] = 10,
                digit @ b'0'..=b'9' => cards[i] = digit - b'0',
                _ => (),
            }
        }

        let mut card_count = FxHashMap::default();
        for card in raw_cards.chars() {
            *card_count.entry(card).or_insert(0) += 1;
        }

        let outcome: HandType = match card_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_count.values().any(|&count| count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&count| count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Impossible Hand sent: {:?}", raw_cards),
        };

        Self {
            cards,
            outcome,
            bid,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct JokerHand {
    outcome: HandType,
    cards: [u8; 5],
    bid: u32,
}

impl JokerHand {
    fn new(input: &str) -> Self {
        let (raw_cards, bid) = input.split_once(" ").expect("space delimited");
        let bid: u32 = bid.parse().expect("Number value");

        let mut cards = [0; 5];

        for (i, &card) in raw_cards.as_bytes().iter().enumerate() {
            match card {
                b'A' => cards[i] = 14,
                b'K' => cards[i] = 13,
                b'Q' => cards[i] = 12,
                b'J' => cards[i] = 0,
                b'T' => cards[i] = 10,
                digit @ b'0'..=b'9' => cards[i] = digit - b'0',
                _ => (),
            }
        }

        let mut card_count = FxHashMap::default();
        for card in raw_cards.chars() {
            *card_count.entry(card).or_insert(0) += 1;
        }

        let outcome: HandType = match card_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_count.values().any(|&count| count == 4) {
                    if card_count.contains_key(&'J') {
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                } else if card_count.contains_key(&'J') {
                    HandType::FiveOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&count| count == 3) {
                    if card_count.contains_key(&'J') {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else {
                    card_count.get(&'J').map_or(HandType::TwoPair, |&count| {
                        if count == 2 {
                            HandType::FourOfAKind
                        } else {
                            HandType::FullHouse
                        }
                    })
                }
            }
            4 => {
                if card_count.contains_key(&'J') {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            5 => {
                if card_count.contains_key(&'J') {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
            _ => panic!("Impossible Hand sent: {:?}", raw_cards),
        };

        Self {
            cards,
            outcome,
            bid,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cards = input
        .trim_end()
        .lines()
        .map(CamelHand::new)
        .collect::<Vec<_>>();
    cards.sort_unstable();

    Some(
        (1..)
            .zip(cards.into_iter())
            .map(|(rank, hand)| rank * hand.bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = input
        .trim_end()
        .lines()
        .map(JokerHand::new)
        .collect::<Vec<_>>();
    cards.sort_unstable();

    Some(
        (1..)
            .zip(cards.into_iter())
            .map(|(rank, hand)| rank * hand.bid)
            .sum(),
    )
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(5905));
    }
}
