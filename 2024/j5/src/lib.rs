mod inputs;
use multimap::MultiMap;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_priorities() {
        assert_eq!(read_priorities(inputs::EXAMPLE_PRIORITIES).get_vec(&97).unwrap(), &vec![13, 61, 47, 29, 53, 75]);
    }

    #[test]
    fn it_solve() {
        assert_eq!(solve_p1(read_priorities(inputs::EXAMPLE_PRIORITIES), inputs::EXAMPLE_UPDATES), 143);
        assert_eq!(solve_p1(read_priorities(inputs::INPUT_PRIORITIES), inputs::INPUT_UPDATES), 143);
    }
}
