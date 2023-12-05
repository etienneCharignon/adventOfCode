mod inputs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref VIDE: Regex = Regex::new(r"^$").unwrap();
}

pub fn read(alamnac: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let mut seeds = vec![];
    let mut maps = Vec::<Vec<(u64, u64, u64)>>::new();
    let mut map = Vec::<(u64, u64, u64)>::new();
    for line in alamnac.lines() {
        if line.starts_with("seeds") {
            seeds =  NUMBER.captures_iter(line).map(|cap| cap[0].parse::<u64>().unwrap()).collect::<Vec<_>>();
        }
        else if line.ends_with("map:") {
            map = Vec::<(u64, u64, u64)>::new();
        }
        else if !VIDE.is_match(line) {
            let numbers = NUMBER.captures_iter(line).map(|cap| cap[0].parse::<u64>().unwrap()).collect::<Vec<_>>();
            map.push((numbers[0], numbers[1], numbers[2]));
        }
        else if !map.is_empty() {
            maps.push(map.clone());
        }
    }
    (seeds, maps)
}

pub fn map_value(value: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for row in map {
        if (row.1..(row.1+row.2)).contains(&value) {
            return row.0 + (value - row.1)
        }
    }
    value
}

pub fn find_location(value: u64, maps: &[Vec<(u64, u64, u64)>]) -> u64 {
    if maps.is_empty() {
        value
    }
    else {
        find_location(map_value(value, &maps[0]), &maps[1..])
    }
}

pub fn find_locations(almanac: (Vec<u64>, Vec<Vec<(u64, u64, u64)>>)) -> Vec<u64> {
    almanac.0.iter().map(|seed| find_location(*seed, &almanac.1[0..])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_test_regex() {
        assert_eq!(VIDE.is_match(""), true);
    }

    #[test]
    fn it_read_input() {
        assert_eq!(read(inputs::EXAMPLE).1[0], vec![(50, 98, 2), (52, 50, 48)]);
        assert_eq!(read(inputs::EXAMPLE).1.iter().count(), 7);
        assert_eq!(read(inputs::EXAMPLE).0, vec![79, 14, 55, 13]);
    }

    #[test]
    fn it_map_value() {
        assert_eq!(map_value(78, &vec![(60, 56, 37), (56, 93, 4)]), 82); // location
        assert_eq!(map_value(78, &vec![(0, 69, 1), (1, 0, 69)]), 78); // humidity
        assert_eq!(map_value(78, &vec![(0, 69, 1), (1, 0, 69)]), 78); // humidity
        assert_eq!(map_value(74, &vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]), 78); // temperature
        assert_eq!(map_value(81, &vec![(88, 18, 7), (18, 25, 70)]), 74); // light
        assert_eq!(map_value(81, &vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]), 81); // water
        assert_eq!(map_value(81, &vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]), 81); // fertilize
        assert_eq!(map_value(79, &vec![(50, 98, 2), (52, 50, 48)]), 81); // soil
    }

    #[test]
    fn it_find_locations_example() {
        assert_eq!(find_locations(read(inputs::EXAMPLE)), vec![82, 43, 86, 35]);
    }

    #[test]
    fn it_find_locations_input() {
        assert_eq!(*find_locations(read(inputs::INPUT)).iter().min().unwrap(), 196167384);
    }
}
