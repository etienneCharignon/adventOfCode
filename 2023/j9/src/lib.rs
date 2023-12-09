mod inputs;

pub fn read(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|l| l.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()).collect()
}

pub fn derive(mesure: &Vec<i64>) -> Vec<i64> {
    let mut diffs = Vec::<i64>::new();
    for i in 0..(mesure.iter().count() - 1) {
        diffs.push(mesure[i+1] - mesure[i])
    }
    diffs
}

pub fn find_next(mesure: &Vec<i64>) -> i64 {
    let mut iterations = Vec::<Vec<i64>>::new();
    let mut current_mesure: Vec<i64> = mesure.to_vec();
    println!("{:?}", current_mesure);
    println!("{:?}", current_mesure.iter().all(|m| { println!("{}", m); *m == 0 }) );
    while ! current_mesure.iter().all(|m| *m == 0) {
        iterations.push(current_mesure.clone());
        current_mesure = derive(&current_mesure);
    }

    let mut next = 0;
    for iteration in iterations.iter().rev() {
        println!("{:?}", iteration);
        next = iteration[0] - next;
    }
    next
}

pub fn solve_p2(input: &str) -> i64 {
    let mesures: Vec<Vec<i64>> = read(input);
    println!("{:?}", mesures);
    mesures.iter().map(|mesure| find_next(&mesure)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_find_next() {
        assert_eq!(find_next(&vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(find_next(&vec![3, 3, 3, 3, 3]), 3);
        assert_eq!(find_next(&vec![0, 0, 0, 0]), 0);
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
