mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::thread;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref VIDE: Regex = Regex::new(r"^$").unwrap();
}

pub fn read(almanac: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
    let mut seeds = vec![];
    let mut maps = Vec::<Vec<(u64, u64, u64)>>::new();
    let mut map = Vec::<(u64, u64, u64)>::new();
    for line in almanac.lines() {
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
            map.sort_by(|r1, r2| r1.0.cmp(&r2.0));
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

pub fn intervals_map(map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut intervals = Vec::<(u64, u64)>::new();
    let mut s = 0;
    for row in map {
        if s != row.1 && row.1 > 1 {
            intervals.push((s, row.1 - 1));
        }
        intervals.push((row.1, row.1 + row.2 - 1));
        s = row.1 + row.2;
    }
    intervals.push((s, std::u64::MAX));
    intervals
}

pub fn intersection(i1: (u64, u64), i2: (u64, u64)) -> Option<(u64, u64)> { 
    if i2.0 > i1.1 || i1.0 > i2.1 {
        None
    }
    else {
        Some((cmp::max(i1.0, i2.0), cmp::min(i1.1, i2.1)))
    }
}

pub fn find_intervals(intervals: &Vec<(u64, u64)>, map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_intervals = Vec::<(u64, u64)>::new(); 
    let mut mapsorted = map.clone();
    mapsorted.sort_by(|r1, r2| r1.1.cmp(&r2.1));
    println!("{:?}", intervals); 
    println!("map : {:?}", mapsorted); 
    let map_intervals = intervals_map(&mapsorted);
    println!("{:?}", map_intervals); 
    for mi in map_intervals {
        for i in intervals {
            match intersection(mi, *i) {
                Some(intersection) => new_intervals.push(intersection),
                None => {}
            };
        }
    }

    println!("{:?}", new_intervals); 
    new_intervals.iter().map(|i| (map_value(i.0, map), map_value(i.1, map))).collect()
}

pub fn find_minimum_location(almanac: (Vec<u64>, Vec<Vec<(u64, u64, u64)>>)) -> u64 {
    let mut seeds = almanac.0.chunks(2).map(|c| (c[0], (c[0] + c[1] - 1))).collect::<Vec<_>>();
    seeds.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    println!("{:?}", seeds); 

    /*let mut intervals: Vec<(u64, u64)> = seeds;

    for map in almanac.1 {
        intervals = find_intervals(&mut intervals, &map);
        println!("{:?}\n", intervals); 
    }

    intervals.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    intervals.iter().next().unwrap().0;*/

    let mut handles = Vec::<thread::JoinHandle<u64>>::new();
    for intervals in seeds {
        let maps = almanac.1.clone();
        handles.push(thread::spawn(move || {
            (intervals.0..intervals.1).map(|seed| find_location(seed, &maps[0..])).min().unwrap()
        }));
    }
    /*
    let locations: Vec<_> = seeds.iter().map(|c| {
       (c.0..c.1).map(|seed| find_location(seed, &almanac.1[0..])).min().unwrap()
    }).collect();*/

    let mut locations  = Vec::<u64>::new();
    for handle in handles {
        locations.push(handle.join().unwrap());
        println!(".");
    }
    *locations.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_find_intervals() {
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(5, 5, 2), (3, 3, 2)]),
                   vec![(2, 2), (3, 4), (6, 6), (7, 8)]);
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(9, 9, 2)]),
                   vec![(2, 4), (6, 8)]);
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(0, 0, 2)]),
                   vec![(2, 4), (6, 8)]);
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(8, 8, 2)]),
                   vec![(2, 4), (6, 7), (8, 8)]);
        assert_eq!(find_intervals(&vec![(2, 8)], &vec![(3, 3, 2)]),
                   vec![(2, 2), (3, 4), (5, 8)]);
        assert_eq!(find_intervals(&vec![(2, 8)], &vec![(1, 1, 10)]),
                   vec![(2, 8)]);
        assert_eq!(find_intervals(&vec![(2, 11)], &vec![(1, 1, 10)]),
                   vec![(2, 10), (11, 11)]);
        assert_eq!(find_intervals(&vec![(1, 20)], &vec![(1, 1, 10), (11, 11, 5) ]),
                   vec![(1, 10), (11, 15), (16, 20)]);
    }

    #[test]
    fn it_compute_map_interval() {
        assert_eq!(intervals_map(&vec![(52, 50, 48), (50, 98, 2)]), [(0, 49), (50, 97), (98, 99), (100, 18446744073709551615)]);
    }

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
    fn it_find_minimum_locations_example_part2() {
        assert_eq!(find_minimum_location(read(inputs::EXAMPLE)), 46);
    }

    #[test]
    #[ignore]
    fn it_find_minimum_locations_input_part2() {
        assert_eq!(find_minimum_location(read(inputs::INPUT)), 125742456);
    }

    #[test]
    fn it_find_locations_input() {
        assert_eq!(*find_locations(read(inputs::INPUT)).iter().min().unwrap(), 196167384);
    }
}
