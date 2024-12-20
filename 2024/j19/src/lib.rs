mod inputs;
use multimap::MultiMap;
use std::collections::HashMap;

pub fn read_towels(input: &str) -> MultiMap<char, &str> {
    let mut stripes_map = MultiMap::new();
    for towel in input.split(", ") {
        stripes_map.insert(towel.chars().next().unwrap(), towel);
    }
    stripes_map
}

pub fn read_designs(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

pub fn is_possible(design: &str, towels_map: &MultiMap<char, &str>, cache: &mut HashMap<String, bool>) -> bool {
    if let Some(is_possible) = cache.get(design) {
        return *is_possible;
    }
    if design.len() == 0 {
        return true;
    }
    if let Some(towels) = towels_map.get_vec(&design.chars().next().unwrap()) {
        for towel in towels {
            if let Some(stripped) = design.strip_prefix(towel) {
                if is_possible(stripped, towels_map, cache) {
                    cache.insert(String::from(stripped), true);
                    return true;
                }
            }
        }
    }
    cache.insert(String::from(design), false);
    false
}

pub fn count_arrangements(design: &str, towels_map: &MultiMap<char, &str>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(count) = cache.get(design) {
        return *count;
    }
    if design.len() == 0 {
        return 1;
    }
    let mut count = 0;
    if let Some(towels) = towels_map.get_vec(&design.chars().next().unwrap()) {
        for towel in towels {
            if let Some(stripped) = design.strip_prefix(towel) {
                let sub_count = count_arrangements(stripped, towels_map, cache);
                if sub_count > 0 {
                    cache.insert(String::from(stripped), sub_count);
                    count += sub_count;
                }
            }
        }
    }
    cache.insert(String::from(design), count);
    count
}

pub fn count_possible_designs(stripes: MultiMap<char, &str>, designs: Vec<&str>) -> usize {
    let mut cache: HashMap<String, bool> = HashMap::new();
    designs.iter().filter(|design| {
        println!("{design}");
        is_possible(design, &stripes, &mut cache)
    }).count()
}

pub fn count_possible_arrangement(stripes: MultiMap<char, &str>, designs: Vec<&str>) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    designs.iter().map(|design| {
        count_arrangements(design, &stripes, &mut cache)
    }).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_possible_design() {
        assert_eq!(count_possible_designs(read_towels(inputs::EXAMPLE_TOWELS), read_designs(inputs::EXAMPLE_DESIGNS)), 6);
        assert_eq!(count_possible_designs(read_towels(inputs::INPUT_TOWELS), read_designs(inputs::INPUT_DESIGNS)), 300);
    }

    #[test]
    fn it_count_arrangenemts() {
        assert_eq!(count_possible_arrangement(read_towels(inputs::EXAMPLE_TOWELS), read_designs(inputs::EXAMPLE_DESIGNS)), 16);
        assert_eq!(count_possible_arrangement(read_towels(inputs::INPUT_TOWELS), read_designs(inputs::INPUT_DESIGNS)), 624802218898092);
    }
}
