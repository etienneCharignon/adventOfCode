mod inputs;
use std::collections::HashSet;

pub fn read(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn expension_from<X>(galaxies: &Vec<(usize, usize)>, size: usize, x: X) -> Vec<usize> 
where
    X: Fn(&(usize, usize)) -> usize,
{
    let presences_v: HashSet<usize> = HashSet::from_iter(galaxies.iter().map(|g| x(g)));
    let mut expension_v = Vec::<usize>::new();
    let mut expension = 0;
    for v in 0..size {
        if ! presences_v.contains(&v) {
            expension += 1;
        }
        expension_v.push(expension);
    }
    expension_v
}

pub fn diff(a: usize, b: usize) -> usize {
    if a > b { a - b } else { b - a }
}

pub fn distance(g1: (usize, usize), g2: (usize, usize), expension_c: &[usize], expension_r: &[usize]) -> usize {
    diff(g2.0 + expension_c[g2.0], g1.0 + expension_c[g1.0]) + diff(g2.1 + expension_r[g2.1], g1.1 + expension_r[g1.1])
}

pub fn solve_p1(world: Vec<Vec<char>>) -> usize {
    let height: usize = world.len();
    let width: usize = world.iter().next().unwrap().len();
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for r in 0..height {
        for c in 0..width {
            if world[r][c] == '#' {
                galaxies.push((c, r));
            }
        }
    }
    let mut expension_c = expension_from(&galaxies, width, |g| g.0);
    let mut expension_r = expension_from(&galaxies, height, |g| g.1);
    println!("{:?}", expension_c);
    println!("{:?}", expension_r);
    println!("{:?}", galaxies);
    let mut paths = Vec::<usize>::new();
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            paths.push(distance(galaxies[i], galaxies[j], &expension_c, &expension_r));
        }
    }
    paths.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compute_distances() {
        assert_eq!(distance((3,0), (7, 8), &vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 3], &vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 2]), 15);
        assert_eq!(distance((7,8), (3, 0), &vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 3], &vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 2]), 15);
    }

    #[test]
    fn it_works() {
        assert_eq!(solve_p1(read(inputs::EXAMPLE)), 374);
        assert_eq!(solve_p1(read(inputs::INPUT)), 10231178);
    }
}
