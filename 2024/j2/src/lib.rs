mod inputs;

pub fn safe(levels: Vec<i32>) -> bool {
    let direction = (levels[0] - levels[1]) < 0;
    for i in 1..levels.len() {
        let increase = levels[i-1] - levels[i];
        if (increase < 0) != direction { return false; }
        if increase.abs() < 1 || increase.abs() > 3 { return false; }
    }
    true
}

pub fn count_safe(input: &str) -> i32 {
    let lines = input.split('\n');
    let mut count = 0;
    for line in  lines {
        let levels: Vec<i32> = line.split_whitespace().map(|s| s.parse().expect("Failed to parse number")).collect();
        if safe(levels.clone()) { count += 1 }
        else {
            for i in 0..levels.len() {
                let mut levels_but_one = levels.clone();
                levels_but_one.remove(i);
                if safe(levels_but_one) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_say_safe() {
        assert_eq!(safe(vec![1, 2, 3]), true);
        assert_eq!(safe(vec![3, 2, 1]), true);
        assert_eq!(safe(vec![1, 2, 1]), false);
        assert_eq!(safe(vec![1, 1, 2]), false);
        assert_eq!(safe(vec![3, 1, 1]), false);
    }

    #[test]
    fn it_works() {
        assert_eq!(count_safe(inputs::EXAMPLE), 4);
        assert_eq!(count_safe(inputs::INPUT), 589);
    }
}
