use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{map, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
use nom::IResult;

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    dice: Vec<Vec<Die>>,
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug, Copy, Clone)]
struct Die {
    count: u32,
    color: Color,
}

fn number(input: &str) -> IResult<&str, u32> {
    map(digit1, |num_str: &str| num_str.parse::<u32>().unwrap())(input)
}

fn parse_game_number(input: &str) -> IResult<&str, u32> {
    delimited(tag("Game "), number, tag(": "))(input)
}

fn parse_die(input: &str) -> IResult<&str, Die> {
    map(
        separated_pair(
            number,
            space1,
            alt((
                value(Color::Red, tag("red")),
                value(Color::Blue, tag("blue")),
                value(Color::Green, tag("green")),
            )),
        ),
        |(count, color)| Die { count, color },
    )(input)
}

fn parse_bag_pull(input: &str) -> IResult<&str, Vec<Die>> {
    separated_list1(tag(", "), parse_die)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        pair(
            parse_game_number,
            separated_list1(tag("; "), parse_bag_pull),
        ),
        |(game_num, dice)| Game { id: game_num, dice },
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| {
                let game = match parse_game(line) {
                    Ok((_, game)) => game,
                    Err(_e) => panic!("Parsing error"),
                };

                for dice_pull in game.dice {
                    for dice in dice_pull {
                        match dice.color {
                            Color::Red => {
                                if dice.count > 12 {
                                    return None;
                                }
                            }
                            Color::Blue => {
                                if dice.count > 14 {
                                    return None;
                                }
                            }
                            Color::Green => {
                                if dice.count > 13 {
                                    return None;
                                }
                            }
                        }
                    }
                }
                Some(game.id)
            })
            .flatten()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .lines()
            .map(|line| {
                let game = match parse_game(line) {
                    Ok((_, game)) => game,
                    Err(_e) => panic!("Parsing error"),
                };

                let mut color_max = (0, 0, 0);
                for dice_pull in game.dice {
                    for dice in dice_pull {
                        match dice.color {
                            Color::Red => {
                                if dice.count > color_max.0 {
                                    color_max.0 = dice.count;
                                }
                            }
                            Color::Green => {
                                if dice.count > color_max.1 {
                                    color_max.1 = dice.count;
                                }
                            }
                            Color::Blue => {
                                if dice.count > color_max.2 {
                                    color_max.2 = dice.count
                                }
                            }
                        }
                    }
                }
                color_max.0 * color_max.1 * color_max.2
            })
            .sum(),
    )
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, None);
    }
}
