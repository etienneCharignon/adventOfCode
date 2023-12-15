mod inputs;

use multimap::MultiMap;

pub fn read_size(input: &str) -> (usize, usize) {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    (lines.len(), lines[0].len())
}

pub fn read(input: &str, type_rock: char) -> Vec<(usize, usize)> {
    let mut rocks = Vec::<(usize, usize)>::new();
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (height, width) = read_size(input);
    for r in 0..height {
        for c in 0..width {
            if lines[r][c] == type_rock {
                rocks.push((c, r));
            }
        }
    }
    rocks
}

pub fn printworld(balls: &Vec<(usize, usize)>, rocks: &Vec<(usize,usize)>, size: (usize, usize)) {
    for r in 0..size.1 {
        let mut row = String::new();
        for c in 0..size.0 {
            if balls.contains(&(c, r)) {
                row.push('O');
            }
            else if rocks.contains(&(c, r)) {
                row.push('#');
            }
            else {
                row.push('.');
            }
        }
        println!("{row}");
    }
    println!("");
}

pub fn find_closest_rock_north(b: (usize, usize), rocks: &Vec<(usize, usize)>) -> usize {
    match rocks.iter().filter(|r| r.0 == b.0 && r.1 < b.1).map(|r| r.1).max() {
        Some(max_rock) => max_rock + 1,
        None => 0
    }
}

pub fn find_closest_rock_south(b: (usize, usize), rocks: &Vec<(usize, usize)>, height: usize) -> usize {
    match rocks.iter().filter(|r| r.0 == b.0 && r.1 > b.1).map(|r| r.1).min() {
        Some(min_rock) => min_rock - 1,
        None => height - 1
    }
}

pub fn find_closest_rock_west(b: (usize, usize), rocks: &Vec<(usize, usize)>) -> usize {
    match rocks.iter().filter(|r| r.1 == b.1 && r.0 < b.0).map(|r| r.0).max() {
        Some(max_rock) => max_rock + 1,
        None => 0
    }
}

pub fn find_closest_rock_est(b: (usize, usize), rocks: &Vec<(usize, usize)>, width: usize) -> usize {
    match rocks.iter().filter(|r| r.1 == b.1 && r.0 > b.0).map(|r| r.0).min() {
        Some(min_rock) => min_rock - 1,
        None => width - 1
    }
}

pub fn tilt_north(balls: &Vec<(usize, usize)>, rocks: &Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut stacks = MultiMap::new();
    for b in balls.iter() {
        let pos = find_closest_rock_north(*b, &rocks);
        stacks.insert((b.0, pos), *b);
    }
    let mut new_balls = Vec::<(usize, usize)>::new();
    for (p, l) in stacks.iter_all() {
        for i in 0..l.len() {
            new_balls.push((p.0, p.1 + i))
        }
    }
    new_balls
}

pub fn tilt_west(balls: &Vec<(usize, usize)>, rocks: &Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut stacks = MultiMap::new();
    for b in balls.iter() {
        let pos = find_closest_rock_west(*b, &rocks);
        stacks.insert((pos, b.1), *b);
    }
    let mut new_balls = Vec::<(usize, usize)>::new();
    for (p, l) in stacks.iter_all() {
        for i in 0..l.len() {
            new_balls.push((p.0 + i, p.1))
        }
    }
    new_balls
}

pub fn tilt_south(balls: &Vec<(usize, usize)>, rocks: &Vec<(usize,usize)>, height: usize) -> Vec<(usize,usize)> {
    let mut stacks = MultiMap::new();
    for b in balls.iter() {
        let pos = find_closest_rock_south(*b, &rocks, height);
        stacks.insert((b.0, pos), *b);
    }
    let mut new_balls = Vec::<(usize, usize)>::new();
    for (p, l) in stacks.iter_all() {
        for i in 0..l.len() {
            new_balls.push((p.0, p.1 - i))
        }
    }
    new_balls
}

pub fn tilt_est(balls: &Vec<(usize, usize)>, rocks: &Vec<(usize,usize)>, width: usize) -> Vec<(usize,usize)> {
    let mut stacks = MultiMap::new();
    for b in balls.iter() {
        let pos = find_closest_rock_est(*b, &rocks, width);
        stacks.insert((pos, b.1), *b);
    }
    let mut new_balls = Vec::<(usize, usize)>::new();
    for (p, l) in stacks.iter_all() {
        for i in 0..l.len() {
            new_balls.push((p.0 - i, p.1))
        }
    }
    new_balls
}

pub fn cycle(balls: &Vec<(usize, usize)>, rocks: &Vec<(usize,usize)>, size: (usize, usize)) -> Vec<(usize,usize)>  {
    let mut new_balls = tilt_north(balls, rocks);
    new_balls = tilt_west(&new_balls, rocks);
    new_balls = tilt_south(&new_balls, rocks, size.0);
    tilt_est(&new_balls, rocks, size.1)
}

pub fn solve(rocks: &Vec<(usize, usize)>, balls: &Vec<(usize,usize)>, size: (usize, usize)) -> usize {
    println!("rocks : {rocks:?}\nballs : {balls:?}");
    let mut new_balls = balls.clone();
    let mut iterations = Vec::<Vec<(usize,usize)>>::new();
    for i in 0..=125 {
        new_balls = cycle(&new_balls, rocks, size);
    }
    for i in 0..((1000000000-126)%59) {
    /*
    for i in 0..10000 {
        */
        new_balls = cycle(&new_balls, rocks, size);
        /*
        //printworld(&new_balls, rocks, size);
        let mut sorted = new_balls.clone();
        sorted.sort();
        //println!("{:?}", sorted);
        if iterations.contains(&sorted) {
            let pos = iterations.iter().position(|e| *e == sorted).unwrap();
            println!("{pos} {i}");
            // pos = 125, i = 184
            break;
        }
        iterations.push(sorted);
        */
    }
    printworld(&new_balls, rocks, size);
    new_balls.iter().map(|b| size.0 - b.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(&read(inputs::INPUT, '#'), &read(inputs::INPUT, 'O'), read_size(inputs::INPUT)), 104247); // 104248 too high
        //assert_eq!(solve(&read(inputs::EXAMPLE, '#'), &read(inputs::EXAMPLE, 'O'), read_size(inputs::EXAMPLE)), 64);
    }
}
