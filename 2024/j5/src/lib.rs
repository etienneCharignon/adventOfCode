mod inputs;
use multimap::MultiMap;
use std::cmp::Ordering;

pub fn read_priorities(priorities: &str) -> MultiMap<i32, i32> {
    let mut map = MultiMap::new();
    for line in priorities.split("\n") {
        let pages: Vec<i32> = line.split("|")
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
        map.insert(pages[0], pages[1]);
    }
    map
}

pub fn good_priorities(priorities: &MultiMap<i32, i32>, pages: &Vec<i32>) -> bool {
    for i in 0..pages.len() {
        let previous_pages = &pages[0..i];
        match priorities.get_vec(&pages[i]) {
            Some(after_pages) => {
                if previous_pages.iter().any(|&p| after_pages.contains(&p)) {
                    return false
                }
            }
            _ => {}
        }
    }
    true
}

pub fn solve_p1(priorities: MultiMap<i32, i32>, updates: &str) -> i32 {
    let mut sum = 0;
    for line in updates.split("\n") {
        let pages: Vec<i32> = line.split(',')
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
        if good_priorities(&priorities, &pages){
            sum += pages[pages.len() / 2];
        }
    }
    sum
}

pub fn compare(priorities: &MultiMap<i32, i32>, p1: &i32, p2: &i32) -> Ordering {
    match priorities.get_vec(p1) {
        Some(after_pages) => {
            if after_pages.contains(p2) {
                Ordering::Less
            }
            else {
                Ordering::Greater
            }
        },
        _ => {
            match priorities.get_vec(p2) {
                Some(after_pages) => {
                    if after_pages.contains(p1) {
                        Ordering::Greater
                    }
                    else {
                        Ordering::Less
                    }
                },
                _ => {
                    Ordering::Equal
                }
            }
        }
    }
}

pub fn solve_p2(priorities: MultiMap<i32, i32>, updates: &str) -> i32 {
    let mut sum = 0;
    for line in updates.split("\n") {
        let mut pages: Vec<i32> = line.split(',')
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect();
        if !good_priorities(&priorities, &pages){
            pages.sort_by(|a, b| compare(&priorities, a,b));
            sum += pages[pages.len() / 2];
        }
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_priorities() {
        assert_eq!(read_priorities(inputs::EXAMPLE_PRIORITIES).get_vec(&97).unwrap(), &vec![13, 61, 47, 29, 53, 75]);
    }

    #[test]
    fn it_solve_p1() {
        assert_eq!(solve_p1(read_priorities(inputs::EXAMPLE_PRIORITIES), inputs::EXAMPLE_UPDATES), 143);
        assert_eq!(solve_p1(read_priorities(inputs::INPUT_PRIORITIES), inputs::INPUT_UPDATES), 5064);
    }

    #[test]
    fn it_solve_p2() {
        assert_eq!(solve_p2(read_priorities(inputs::EXAMPLE_PRIORITIES), inputs::EXAMPLE_UPDATES), 123);
        assert_eq!(solve_p2(read_priorities(inputs::INPUT_PRIORITIES), inputs::INPUT_UPDATES), 5152);
    }
}
