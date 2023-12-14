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

pub fn count_diff(p1: &Vec<char>, p2: &Vec<char>) -> usize {
    let mut count = 0;
    for i in 0..p1.len() {
        if p1[i] != p2[i] {
            count += 1;
        } 
    }
    count
}

pub fn find_symetrie(pattern: &Vec<Vec<char>>) -> usize {
    println!("pattern {}", String::from_iter(&pattern[0]));
    let last_line = pattern.len() - 1;
    for r in 0..last_line {

        if (pattern.len() - r) % 2 != 0 {
            continue;
        }
        let mut diff = count_diff(&pattern[r], &pattern[last_line]);
        let middle = (pattern.len() - r)/2;
        if diff == 0 {
            if (1..middle).map(|i| count_diff(&pattern[r + i], &pattern[last_line - i])).sum::<usize>() == 1 {
                println!("pattern0 {}, {}", String::from_iter(&pattern[0]), middle + r);
                return 100 * (middle + r);
            }
        }
        else if diff == 1 {
            if (1..middle).all(|i| pattern[r + i] == pattern[last_line - i]) {
                println!("pattern1 {}, {}", String::from_iter(&pattern[0]), middle + r);
                return 100 * (middle + r);
            }
        }

        diff = count_diff(&pattern[0], &pattern[last_line - r]);
        if diff == 0 {
            if (1..middle).map(|i| count_diff(&pattern[i], &pattern[last_line - r - i])).sum::<usize>() == 1 {
                println!("pattern0- {}, {}", String::from_iter(&pattern[0]), middle);
                return 100 * (middle);
            }
        }
        else if diff == 1 {
            if (1..middle).all(|i| pattern[i] == pattern[last_line - r - i]) {
                println!("pattern1- {}, {}", String::from_iter(&pattern[0]), middle);
                return 100 * (middle);
            }
        }
    }

    let last_column = pattern[0].len() - 1;
    for c in 0..last_column {
        if (last_column + 1 - c) % 2 != 0 {
            continue;
        }
        let middle = (last_column + 1 - c)/2;
        let mut diff = count_diff(&column(pattern, c), &column(pattern, last_column));
        if diff == 0 {
            if (1..middle).map(|i| count_diff(&column(pattern, c + i), &column(pattern, last_column - i))).sum::<usize>() == 1 {
                println!("patternc0 {}, {}", String::from_iter(&pattern[0]), middle + c);
                return middle + c;
            }
        }
        else if diff == 1 {
            if (1..middle).all(|i| column(pattern, c + i) == column(pattern, last_column - i)) {
                println!("patternc1 {}, {}", String::from_iter(&pattern[0]), middle + c);
                return middle + c;
            }
        }

        diff = count_diff(&column(pattern, 0), &column(pattern, last_column - c));
        if diff == 0 {
            if (1..middle).map(|i| count_diff(&column(pattern, i), &column(pattern, last_column - c - i))).sum::<usize>() == 1 {
                println!("patternc0- {}, {}", String::from_iter(&pattern[0]), middle);
                return middle;
            }
        }
        else if diff == 1 {
            if (1..middle).all(|i| column(pattern, i) == column(pattern, last_column - c - i)) {
                println!("patternc1- {}, {}", String::from_iter(&pattern[0]), middle);
                return middle;
            }
        }
    }
    panic!("aucune tache trouvée");
}

/*
pub fn search_smudge(p: &mut Vec<Vec<char>>) -> usize {
    for r in 0..p.len() {
        for c in 0..p[0].len() {
            if let Some(elem) = p[r].get_mut(c) {
                *elem = if *elem == '.' { '#' } else { '.' };
            }
            match find_symetrie(p) {
                Some(n) => return n,
                None => {
                    if let Some(elem) = p[r].get_mut(c) {
                        *elem = if *elem == '.' { '#' } else { '.' };
                    }
                }
            }
        }
    }
    panic!("Accune tache trouvée");
}*/

pub fn solve(patterns: &Vec<Vec<Vec<char>>>) -> usize {
    patterns.iter().map(|p| find_symetrie(p)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_diff() {
        assert_eq!(count_diff(&("#.".chars().collect()), &("#.".chars().collect())), 0);
        assert_eq!(count_diff(&("#.".chars().collect()), &("##".chars().collect())), 1);
        assert_eq!(count_diff(&("..".chars().collect()), &("##".chars().collect())), 2);
    }

    #[test]
    fn it_works() {
        assert_eq!(solve(&read(inputs::INPUT)), 31974); // too high
        assert_eq!(solve(&read(inputs::EXAMPLE)), 400);
    }
}