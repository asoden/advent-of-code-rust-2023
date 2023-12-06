struct Race {
    time: u64,
    distance: u64,
}

fn parse_races(input: &str) -> Vec<Race> {
    let (time_line, distance_line) = input.trim_end().split_once("\n").expect("two line input");
    let (_, times) = time_line.split_once(":").expect("Colon expected");
    let (_, distances) = distance_line.split_once(":").expect("Colon expected");

    let time_collection = times.split(" ").filter(|x| x.len() > 0).collect::<Vec<_>>();
    let distance_collection = distances
        .split(" ")
        .filter(|x| x.len() > 0)
        .collect::<Vec<_>>();

    let mut races: Vec<Race> = Vec::with_capacity(time_collection.len());

    for i in 0..time_collection.len() {
        races.push(Race {
            time: time_collection[i].parse().unwrap(),
            distance: distance_collection[i].parse().unwrap(),
        });
    }

    races
}

fn parse_race_bad_kerning(input: &str) -> Race {
    let (time_line, distance_line) = input.trim_end().split_once("\n").expect("two line input");
    let (_, times) = time_line.split_once(":").expect("Colon expected");
    let (_, distances) = distance_line.split_once(":").expect("Colon expected");

    let time = times.replace(" ", "").parse().unwrap();
    let distance = distances.replace(" ", "").parse().unwrap();

    Race { time, distance }
}

fn find_quad_roots(race: &Race) -> (f64, f64) {
    let trinomial: f64 = (race.time.pow(2) - 4 * race.distance) as f64;

    let left = (-(race.time as f64) + trinomial.sqrt()) / -2.0;
    let right = (-(race.time as f64) - trinomial.sqrt()) / -2.0;

    (left.min(right), right.max(left))
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_races(input);
    let x: f64 = races
        .iter()
        .map(find_quad_roots)
        .map(|(left, right)| ((right.ceil() - 1.0) - left).ceil())
        .product();
    Some(x as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = parse_race_bad_kerning(input);
    let (left, right) = find_quad_roots(&race);

    Some(((right.ceil() - 1.0) - left).ceil() as u64)
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(71503));
    }
}
