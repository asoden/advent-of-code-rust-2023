use rayon::prelude::*;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[derive(Debug, Copy, Clone)]
struct Mapping {
    source: u64,
    destination: u64,
    range: u64,
}

fn map_value(source_value: u64, mapping: &Mapping) -> Option<u64> {
    let min_source_range = mapping.source;
    let max_source_range = mapping.source + mapping.range;

    if source_value < min_source_range || source_value > max_source_range {
        return None;
    }

    let offset = source_value - min_source_range;

    Some(mapping.destination + offset)
}

fn traverse_maps(source_value: u64, mappings: &[Mapping]) -> u64 {
    for map in mappings {
        match map_value(source_value, map) {
            Some(val) => return val,
            None => (),
        }
    }
    source_value
}

fn parse_mapping(input: &str) -> Mapping {
    let split: Vec<_> = input.split(" ").map(|x| x.parse().unwrap()).collect();
    Mapping {
        source: split[1],
        destination: split[0],
        range: split[2],
    }
}

fn parse_almanac(input: &str) -> Almanac {
    let sections: Vec<_> = input.split("\n\n").collect();

    //seeds
    let seeds: Vec<u64> = sections[0]
        .split_once(": ")
        .expect("colon seperation")
        .1
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    //seed to soil
    let seed_to_soil: Vec<Mapping> = sections[1]
        .split_once(":\n")
        .expect("colon delimited label")
        .1
        .lines()
        .map(parse_mapping)
        .collect();

    //soil to fertilizer
    let soil_to_fertilizer: Vec<Mapping> = sections[2]
        .split_once(":\n")
        .expect("colon delimited label")
        .1
        .lines()
        .map(parse_mapping)
        .collect();

    //fertilizer to water
    let fertilizer_to_water: Vec<Mapping> = sections[3]
        .split_once(":\n")
        .expect("colon delimited label")
        .1
        .lines()
        .map(parse_mapping)
        .collect();

    //water to light
    let water_to_light: Vec<Mapping> = sections[4]
        .split_once(":\n")
        .expect("colon delimited label")
        .1
        .lines()
        .map(parse_mapping)
        .collect();

    //light to temp
    let light_to_temperature: Vec<Mapping> = sections[5]
        .split_once(":\n")
        .expect("colon delimited label")
        .1
        .lines()
        .map(parse_mapping)
        .collect();

    //temperature to humidity
    let temperature_to_humidity: Vec<Mapping> = sections[6]
        .split_once(":\n")
        .expect("colon delimited label")
        .1
        .lines()
        .map(parse_mapping)
        .collect();

    //humidity to location
    let humidity_to_location: Vec<Mapping> = sections[7]
        .split_once(":\n")
        .expect("colon delimited label")
        .1
        .lines()
        .map(parse_mapping)
        .collect();

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let val = parse_almanac(input);

    val.seeds
        .into_iter()
        .map(|seed| traverse_maps(seed, &val.seed_to_soil))
        .map(|soil| traverse_maps(soil, &val.soil_to_fertilizer))
        .map(|fertilizer| traverse_maps(fertilizer, &val.fertilizer_to_water))
        .map(|water| traverse_maps(water, &val.water_to_light))
        .map(|light| traverse_maps(light, &val.light_to_temperature))
        .map(|temperature| traverse_maps(temperature, &val.temperature_to_humidity))
        .map(|humidity| traverse_maps(humidity, &val.humidity_to_location))
        .reduce(u64::min)
}

pub fn part_two(input: &str) -> Option<u64> {
    let val = parse_almanac(input);

    let ranges: Vec<RangeInclusive<u64>> =
        val.seeds.chunks(2).map(|x| x[0]..=(x[0] + x[1])).collect();

    ranges
        .into_iter()
        .map(|range| {
            range
                .into_par_iter()
                .map(|seed| traverse_maps(seed, &val.seed_to_soil))
                .map(|soil| traverse_maps(soil, &val.soil_to_fertilizer))
                .map(|fertilizer| traverse_maps(fertilizer, &val.fertilizer_to_water))
                .map(|water| traverse_maps(water, &val.water_to_light))
                .map(|light| traverse_maps(light, &val.light_to_temperature))
                .map(|temperature| traverse_maps(temperature, &val.temperature_to_humidity))
                .map(|humidity| traverse_maps(humidity, &val.humidity_to_location))
                .reduce(|| u64::MAX, u64::min)
        })
        .reduce(u64::min)
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(46));
    }
}
