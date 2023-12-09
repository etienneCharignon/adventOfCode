mod inputs;

pub fn read(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|l| l.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()).collect()
}

pub fn diffs(mesure: &[i64]) -> Vec<i64> {
    let mut diffs = Vec::<i64>::new();
    for i in 0..(mesure.len() - 1) {
        diffs.push(mesure[i+1] - mesure[i])
    }
    diffs
}

pub fn find_previous(mesure: &[i64]) -> i64 {
    if mesure.iter().all(|m| *m == 0) {
        0
    } else {
        mesure[0] - find_previous(&diffs(&mesure))
    }
}

pub fn solve_p2(input: &str) -> i64 {
    let mesures: Vec<Vec<i64>> = read(input);
    println!("{:?}", mesures);
    mesures.iter().map(|mesure| find_previous(&mesure)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_find_next() {
        assert_eq!(find_previous(&vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(find_previous(&vec![3, 3, 3, 3, 3]), 3);
        assert_eq!(find_previous(&vec![0, 0, 0, 0]), 0);
    }

    #[test]
    fn it_solve_example() {
        assert_eq!(solve_p2(inputs::EXAMPLE), 2);
    }

    #[test]
    fn it_solve_input() {
        assert_eq!(solve_p2(inputs::INPUT), 919);
    }
}
