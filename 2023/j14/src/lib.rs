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

pub fn find_closest_rock(b: (usize, usize), rocks: &Vec<(usize, usize)>, size: (usize, usize)) -> usize {
    let (height, width) = size;
    match rocks.iter().filter(|r| r.0 == b.0 && r.1 < b.1).map(|r| r.1).max() {
        Some(max_rock_y) => max_rock_y + 1,
        None => 0
    }
}

pub fn solve(rocks: &Vec<(usize, usize)>, balls: &Vec<(usize,usize)>, size: (usize, usize)) -> usize {
    println!("rocks : {rocks:?}\nballs : {balls:?}");
    let mut stacks = MultiMap::new();
    for b in balls.iter() {
        let pos = find_closest_rock(*b, &rocks, size);
        stacks.insert((b.0, pos), *b);
        println!("{b:?} {pos}");
    }
    println!("{stacks:?}");
    let mut new_balls = Vec::<(usize, usize)>::new();
    for (p, l) in stacks.iter_all() {
        for i in 0..l.len() {
            new_balls.push((p.0, p.1 + i))
        }
    }
    println!("{new_balls:?}");
    new_balls.iter().map(|b| size.0 - b.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(solve(&read(inputs::EXAMPLE, '#'), &read(inputs::EXAMPLE, 'O'), read_size(inputs::EXAMPLE)), 136);
        assert_eq!(solve(&read(inputs::INPUT, '#'), &read(inputs::INPUT, 'O'), read_size(inputs::INPUT)), 108840);
    }
}
