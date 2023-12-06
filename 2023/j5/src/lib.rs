mod inputs;

use lazy_static::lazy_static;
use regex::Regex;

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

pub fn map_value_r(value: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for row in map {
        if (row.0..(row.0+row.2)).contains(&value) {
            return row.1 + (value - row.0)
        }
    }
    value
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

pub fn intervals(map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut intervals = Vec::<(u64, u64)>::new();
    let mut s = 0;
    for row in map {
        if s != row.0 {
            intervals.push((s, row.0));
        }
        intervals.push((row.0, row.0 + row.2));
        s = row.0 + row.2;
    }
    intervals
}

pub fn intersections(intervals: Vec<(u64, u64)>, map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> { 
    let mut new_intervals = Vec::<(u64, u64)>::new();
    for interval in intervals {
        
    }
    new_intervals
}

pub fn find_intervals(intervals: &Vec<(u64, u64)>, map: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_intervals = Vec::<(u64, u64)>::new(); 
    let mut mapsorted = map.clone();
    mapsorted.sort_by(|r1, r2| r1.1.cmp(&r2.1));
    println!("{:?}", intervals); 
    println!("{:?}", mapsorted); 
    let mut itermap = mapsorted.iter();
    let mut orull = itermap.next();
    for interval in intervals {
        if orull == None {
            new_intervals.push(*interval);
            continue;
        }
        let mut rull = orull.unwrap();
        while interval.0 >= rull.1 + rull.2 {
            orull = itermap.next();
            if orull == None {
                break;
            }
            rull = orull.unwrap();
        }
        if orull == None {
            new_intervals.push(*interval);
            continue;
        }
        while true {
            if interval.0 < rull.1 && interval.1 >= rull.1 {
                new_intervals.push((interval.0, rull.1));
                if interval.1 > rull.1 {
                    if rull.1 + rull.2 < interval.1 {
                        new_intervals.push((rull.1, rull.1 + rull.2));
                        new_intervals.push((rull.1 + rull.2, interval.1));
                    }
                    else {
                        new_intervals.push((rull.1, interval.1));
                    }
                }
            }
            else if interval.0 < rull.1 + rull.2 && interval.1 >= rull.1 + rull.2 {
                new_intervals.push((interval.0, rull.1 + rull.2));
                if  interval.1 > rull.1 + rull.2 {
                    new_intervals.push((rull.1 + rull.2, interval.1));
                }
            }
            else if interval.0 > rull.1 {
                new_intervals.push(*interval);
            }
            else if interval.1 < rull.1 {
                new_intervals.push(*interval);
                break;
            }
            orull = itermap.next();
            if orull == None { break; }
            rull = orull.unwrap();
            if interval.1 < rull.1 { break; }
        }
    }
    new_intervals
}

pub fn find_minimum_location(almanac: (Vec<u64>, Vec<Vec<(u64, u64, u64)>>)) -> u64 {
    let mut seeds = almanac.0.chunks(2).map(|c| (c[0], (c[0]+c[1]))).collect::<Vec<_>>();
    seeds.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    println!("{:?}", seeds); 

    let mut intervals: Vec<(u64, u64)> = seeds;

    for map in almanac.1 {
        intervals = find_intervals(&mut intervals, &map)
    }

    intervals.sort_by(|r1, r2| r1.0.cmp(&r2.0));
    intervals.iter().next().unwrap().0

/*    let locations: Vec<_> = seeds.iter().map(|c| {
       (c.0..c.1).map(|seed| find_location(seed, &almanac.1[0..])).min().unwrap()
    }).collect();
    *locations.iter().min().unwrap()*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_find_intervals() {
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(5, 5, 2), (3, 3, 2)]),
                   vec![(2, 3), (3, 4), (6, 7), (7, 8)]);
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(9, 9, 2)]),
                   vec![(2, 4), (6, 8)]);
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(0, 0, 2)]),
                   vec![(2, 4), (6, 8)]);
        assert_eq!(find_intervals(&vec![(2, 4), (6, 8)], &vec![(8, 8, 2)]),
                   vec![(2, 4), (6, 8)]);
        assert_eq!(find_intervals(&vec![(2, 8)], &vec![(3, 3, 2)]),
                   vec![(2, 3), (3, 5), (5, 8)]);
        assert_eq!(find_intervals(&vec![(2, 8)], &vec![(1, 1, 10)]),
                   vec![(2, 8)]);
        assert_eq!(find_intervals(&vec![(2, 11)], &vec![(1, 1, 10)]),
                   vec![(2, 11)]);
        assert_eq!(find_intervals(&vec![(1, 20)], &vec![(1, 1, 10), (11, 11, 5) ]),
                   vec![(1, 11), (11, 16), (16, 20)]);
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
    #[ignore]
    fn it_find_minimum_locations_example_part2() {
        assert_eq!(find_minimum_location(read(inputs::EXAMPLE)), 46);
    }

    #[test]
    #[ignore]
    fn it_find_minimum_locations_input_part2() {
        assert_eq!(find_minimum_location(read(inputs::INPUT)), 46);
    }

    #[test]
    fn it_find_locations_input() {
        assert_eq!(*find_locations(read(inputs::INPUT)).iter().min().unwrap(), 196167384);
    }
}
