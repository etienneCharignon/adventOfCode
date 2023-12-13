mod inputs;

pub fn read(world: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns = Vec::<Vec<Vec<char>>>::new(); 
    let mut pattern = Vec::<Vec<char>>::new();
    for l in world.lines() {
        if l.len() == 0 {
            patterns.push(pattern);
            pattern = Vec::<Vec::<char>>::new();
        }
        else {
            pattern.push(l.chars().collect());
        }
    }
    patterns
}

pub fn column(pattern: &Vec<Vec<char>>, i: usize) -> Vec<char> {
    pattern.iter().map(|l| l[i]).collect()
}

pub fn find_symetrie(pattern: &Vec<Vec<char>>) -> usize {
    println!("pattern {}", String::from_iter(&pattern[0]));
    let mut sym = 0;
    let last_line = pattern.len() - 1;
    for r in 0..last_line {
        if pattern[r] == pattern[last_line] {
            let middle = (pattern.len() - r)/2;
            println!("line {r} {last_line} {} {middle} {} {}", last_line + 1 - r, String::from_iter(&pattern[r]), String::from_iter(&pattern[last_line]));
            if (1..middle).all(|i| pattern[r + i] == pattern[last_line - i]) {
                println!("{}", middle + r);
                return 100 * (middle + r);
            }
        }
        if pattern[0] == pattern[last_line - r] {
            let middle = (pattern.len() - r)/2;
            println!("line- {} {last_line} {} {middle} {} {}",  last_line - r, last_line + 1 - r, String::from_iter(&pattern[r]), String::from_iter(&pattern[last_line]));
            if (1..middle).all(|i| pattern[i] == pattern[last_line - r - i]) {
                println!("{}", middle);
                return 100 * (middle);
            }
        }
    }
    let last_column = pattern[0].len() - 1;
    for c in 0..last_column {
        // println!("{}, {} {}", c, String::from_iter(column(pattern, c)), String::from_iter(column(pattern, last_column)));
        if column(pattern, c) == column(pattern, last_column) {
            let middle = (last_column + 1 - c)/2;
            println!("column {c} {last_column} {} {middle} {} {}", last_column + 1 - c, String::from_iter(&column(pattern, c)), String::from_iter(&column(pattern, last_column)));
            if (1..middle).all(|i| column(pattern, c + i) == column(pattern, last_column - i)) {
                println!("{}", middle + c);
                return middle + c;
            }
        }
        if column(pattern, 0) == column(pattern, last_column - c) {
            let middle = (last_column + 1 - c)/2;
            println!("column- {} {last_column} {} {middle} {} {}", last_column - c, last_column + 1 - c, String::from_iter(&column(pattern, c)), String::from_iter(&column(pattern, last_column)));
            if (1..middle).all(|i| column(pattern, i) == column(pattern, last_column - c - i)) {
                println!("{}", middle);
                return middle;
            }
        }
    }
    panic!("aucune symétrie {}", String::from_iter(&pattern[0]));
}

pub fn solve(patterns: &Vec<Vec<Vec<char>>>) -> usize {
    println!("{:?}", patterns.iter().map(|p| find_symetrie(p)).collect::<Vec<_>>());
    patterns.iter().map(|p| find_symetrie(p)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(&read(inputs::EXAMPLE)), 405);
        assert_eq!(solve(&read(inputs::INPUT)), 35210); // 38575 to hight and 26764 is too low 28609 is wrong too, 37585 non.
    }
}
// 29327 non
// 31723 non
// 32783 non