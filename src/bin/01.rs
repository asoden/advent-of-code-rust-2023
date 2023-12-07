use fxhash::FxHashMap as HashMap;

use regex::Regex;

fn find_num(bad_calibration: &str) -> u32 {
    let tens = bad_calibration.chars().find(|x| x.is_digit(10)).unwrap();

    let ones = bad_calibration
        .chars()
        .rev()
        .find(|x| x.is_numeric())
        .unwrap();

    let string_num = format!("{tens}{ones}");
    string_num.parse().unwrap()
}

fn find_crazy_person_numbers(line: &str, number_words_map: &HashMap<&str, u32>) -> u32 {
    let regex = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let xeger = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let reversed: String = line.chars().rev().collect();
    let tens = number_words_map
        .get(<&str>::from(regex.find(line).unwrap()))
        .unwrap();
    let ones = number_words_map
        .get(<&str>::from(xeger.find(&reversed).unwrap()))
        .unwrap();
    tens * 10 + ones
}

fn wtf_replace(line: &str) -> u32 {
    let wtf = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ee")
        .replace("four", "4")
        .replace("five", "fi5e")
        .replace("six", "6")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "ni9e");

    find_num(&wtf)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end()
            .split("\n")
            .map(|line| find_num(line))
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end()
            .split("\n")
            .map(|line| wtf_replace(line))
            .sum(),
    )
}

// pub fn part_two(input: &str) -> Option<u32> {
//     let mut number_words: HashMap<&str, u32> = HashMap::default();
//     number_words.insert("one", 1);
//     number_words.insert("1", 1);
//     number_words.insert("eno", 1);
//     number_words.insert("two", 2);
//     number_words.insert("2", 2);
//     number_words.insert("owt", 2);
//     number_words.insert("three", 3);
//     number_words.insert("3", 3);
//     number_words.insert("eerht", 3);
//     number_words.insert("four", 4);
//     number_words.insert("4", 4);
//     number_words.insert("ruof", 4);
//     number_words.insert("five", 5);
//     number_words.insert("5", 5);
//     number_words.insert("evif", 5);
//     number_words.insert("six", 6);
//     number_words.insert("6", 6);
//     number_words.insert("xis", 6);
//     number_words.insert("seven", 7);
//     number_words.insert("7", 7);
//     number_words.insert("neves", 7);
//     number_words.insert("eight", 8);
//     number_words.insert("8", 8);
//     number_words.insert("thgie", 8);
//     number_words.insert("nine", 9);
//     number_words.insert("9", 9);
//     number_words.insert("enin", 9);
//     Some(
//         input
//             .trim_end()
//             .split("\n")
//             .map(|crazy_number| find_crazy_person_numbers(crazy_number, &number_words))
//             .sum(),
//     )
// }

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(220));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(281));
    }
}
