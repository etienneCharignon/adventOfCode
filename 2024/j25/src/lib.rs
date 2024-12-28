mod inputs;

pub fn read_element(lines: Vec<&str>) -> Vec<usize> {
    let mut element = vec![0,0,0,0,0];
    for line in &lines[1..] {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                element[i] += 1;
            }
        }
    }
    element
}

pub fn matches(key: &Vec<usize>, lock: &Vec<usize>) -> bool {
    key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5)
}

pub fn count_match(input: &str)-> usize {
    let mut keys = vec![];
    let mut locks = vec![];
    for element in input.split("\n\n") {
        let lines: Vec<&str> = element.split('\n').collect();
        if lines[0] == "#####" {
            locks.push(read_element(lines));
        }
        else {
            keys.push(read_element(lines.into_iter().rev().collect()));
        }
    }
    let mut count= 0;
    for key in keys {
        for lock in &locks {
            if matches(&key, &lock) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_element() {
        assert_eq!(read_element(vec!["#####", ".####", ".####", ".####", ".#.#.", ".#...", "....."]), vec![0,5,3,4,3]);
    }

    #[test]
    fn it_matches() {
        assert_eq!(matches(&vec![3,0,2,0,1], &vec![0,5,3,4,3]), true);
        assert_eq!(matches(&vec![5,0,2,1,3], &vec![0,5,3,4,3]), false);
    }

    #[test]
    fn it_works() {
        assert_eq!(count_match(inputs::EXAMPLE), 3);
        assert_eq!(count_match(inputs::INPUT), 2993);
    }
}
