use regex::Regex;
use std::cell::Cell;
use std::collections::BTreeMap;

const LINE_LENGTH: Cell<usize> = Cell::new(0);

fn set_line_length(text: &str) {
    LINE_LENGTH.set(text.find("\n").expect("input string needs a new line"));
}

fn map_values(input: &str) -> (BTreeMap<(i32, i32), (u32, u32)>, Vec<(i32, i32, char)>) {
    let numbers = Regex::new(r"(\d)+").unwrap();
    let mut number_map: BTreeMap<(i32, i32), (u32, u32)> = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for number_match in numbers.find_iter(line) {
            number_map.insert(
                (number_match.start() as i32, y as i32), // (x, y)
                (
                    number_match.len() as u32,              // length
                    number_match.as_str().parse().unwrap(), // value
                ),
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

fn get_surrounding(
    source_x: i32,
    source_y: i32,
    number_map: &BTreeMap<(i32, i32), (u32, u32)>,
) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();
    // back diag down 3 long
    let mut val = number_map
        .get(&(source_x - 3, source_y - 1))
        .unwrap_or(&(0, 0));

    if val.0 == 3 {
        results.push(val.1);
    }
    // back diag down 2 long
    val = number_map
        .get(&(source_x - 2, source_y - 1))
        .unwrap_or(&(0, 0));
    if val.0 >= 2 {
        results.push(val.1);
    }
    // back diag down 1 long
    results.push(
        number_map
            .get(&(source_x - 1, source_y - 1))
            .unwrap_or(&(0, 0))
            .1,
    );

    // back 3 long
    val = number_map.get(&(source_x - 3, source_y)).unwrap_or(&(0, 0));
    if val.0 == 3 {
        results.push(val.1);
    }

    // back 2 long
    val = number_map.get(&(source_x - 2, source_y)).unwrap_or(&(0, 0));
    if val.0 >= 2 {
        results.push(val.1);
    }

    // back 1 long
    results.push(
        number_map
            .get(&(source_x - 1, source_y))
            .unwrap_or(&(0, 0))
            .1,
    );

    // back diag up 3 long
    val = number_map
        .get(&(source_x - 3, source_y + 1))
        .unwrap_or(&(0, 0));
    if val.0 == 3 {
        results.push(val.1);
    }

    // back diag up 2 long
    val = number_map
        .get(&(source_x - 2, source_y + 1))
        .unwrap_or(&(0, 0));
    if val.0 >= 2 {
        results.push(val.1);
    }

    // back diag up 1 long
    results.push(
        number_map
            .get(&(source_x - 1, source_y + 1))
            .unwrap_or(&(0, 0))
            .1,
    );
    // down
    results.push(
        number_map
            .get(&(source_x, source_y - 1))
            .unwrap_or(&(0, 0))
            .1,
    );
    // up
    results.push(
        number_map
            .get(&(source_x, source_y + 1))
            .unwrap_or(&(0, 0))
            .1,
    );
    // forward diag down
    results.push(
        number_map
            .get(&(source_x + 1, source_y - 1))
            .unwrap_or(&(0, 0))
            .1,
    );
    // forward
    results.push(
        number_map
            .get(&(source_x + 1, source_y))
            .unwrap_or(&(0, 0))
            .1,
    );
    // forward diag up
    results.push(
        number_map
            .get(&(source_x + 1, source_y + 1))
            .unwrap_or(&(0, 0))
            .1,
    );
    results
}

pub fn part_one(input: &str) -> Option<u32> {
    set_line_length(input);
    let (number_map, symbol_map) = map_values(input);
    Some(
        symbol_map
            .iter()
            .map(|(x, y, _symbol)| get_surrounding(*x, *y, &number_map).iter().sum::<u32>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    set_line_length(input);
    let (number_map, symbol_map) = map_values(input);
    Some(
        symbol_map
            .iter()
            .filter(|(_, _, symbol)| symbol.eq(&'*'))
            .map(|(x, y, _symbol)| {
                let res: Vec<u32> = get_surrounding(*x, *y, &number_map)
                    .iter()
                    .filter(|x| x > &&0)
                    .map(|x| *x)
                    .collect();
                if res.len() > 1 {
                    res.iter().product()
                } else {
                    0
                }
            })
            .sum(),
    )
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
        assert_eq!(result, Some(467835));
    }
}
