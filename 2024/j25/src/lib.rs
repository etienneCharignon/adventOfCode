mod inputs;

pub fn read_element(lines: Vec<&str>) -> Vec<usize> {
    lines[1..].iter()
        .flat_map(|line| line.chars().enumerate())
        .filter(|(_column, char)| *char == '#')
        .fold(vec![0,0,0,0,0], |mut element, (column, _)| {
            element[column] += 1;
            element
        })
}

pub fn matches(key: &Vec<usize>, lock: &Vec<usize>) -> bool {
    key.iter()
        .zip(lock.iter())
        .all(|(k, l)| k + l <= 5)
}

pub fn count_match(input: &str)-> usize {
    let mut keys = vec![];
    let mut locks = vec![];
    input.split("\n\n")
        .map(|element| element.split('\n').collect())
        .for_each(|lines: Vec<&str>| {
            if lines[0] == "#####" {
                locks.push(read_element(lines));
            }
            else {
                keys.push(read_element(lines.into_iter().rev().collect()));
            }
        });
    keys.iter()
        .flat_map(|key| locks.iter().map(move |lock| (key, lock)))
        .filter(|(key, lock)| matches(key, lock))
        .count()
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
