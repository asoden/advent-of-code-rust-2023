use regex::Regex;
use std::cell::RefCell;
use std::cell::{Cell, Ref};
use std::collections::BTreeMap;

const LINE_LENGTH: Cell<usize> = Cell::new(0);

fn set_line_length(text: &str) {
    LINE_LENGTH.set(text.find("\n").expect("input string needs a new line"));
}

fn map_values(input: &str) -> (BTreeMap<(i32, i32), u32>, Vec<(i32, i32, char)>) {
    let numbers = Regex::new(r"(\d)+").unwrap();
    let mut number_map: BTreeMap<(i32, i32), u32> = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for number_match in numbers.find_iter(line) {
            number_map.insert(
                (number_match.start() as i32, y as i32),
                number_match.as_str().parse().unwrap(),
            );
        }
    }

    let mut symbol_map: Vec<(i32, i32, char)> = Vec::new();
    let symbols = Regex::new(r"[^\d.]").unwrap();
    for (y, line) in input.lines().enumerate() {
        for symbol_match in symbols.find_iter(line) {
            symbol_map.push((
                symbol_match.start() as i32,
                y as i32,
                symbol_match.as_str().parse().unwrap(),
            ))
        }
    }

    (number_map, symbol_map)
}
pub fn part_one(input: &str) -> Option<u32> {
    set_line_length(input);
    let (mut number_map, symbol_map) = map_values(input);
    Some(
        symbol_map
            .iter()
            .map(|(x, y, _symbol)| {
                // back diag down 3 long
                let val = number_map.remove(&(x - 3, y - 1)).unwrap_or(0) +
            // back diag down 2 long
            number_map.remove(&(x - 2, y - 1)).unwrap_or(0) +
            // back diag down 1 long
            number_map.remove(&(x - 1, y - 1)).unwrap_or(0) +
            // back 3 long
            number_map.remove(&(x - 3, *y)).unwrap_or(0) +
            // back 2 long
            number_map.remove(&(x - 2, *y)).unwrap_or(0) +
            // back 1 long
            number_map.remove(&(x - 1, *y)).unwrap_or(0) +
            // back diag up 3 long
            number_map.remove(&(x - 3, y + 1)).unwrap_or(0) +
            // back diag up 2 long
            number_map.remove(&(x - 2, y + 1)).unwrap_or(0) +
            // back diag up 1 long
            number_map.remove(&(x - 1, y + 1)).unwrap_or(0) +
            // down
            number_map.remove(&(*x, y - 1)).unwrap_or(0) +
            // up
            number_map.remove(&(*x, y + 1)).unwrap_or(0) +
            // forward diag down
            number_map.remove(&(x + 1, y - 1)).unwrap_or(0) +
            // forward
            number_map.remove(&(x + 1, *y)).unwrap_or(0) +
            // forward diag up
            number_map.remove(&(x + 1, y + 1)).unwrap_or(0);
                val
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, None);
    }
}
