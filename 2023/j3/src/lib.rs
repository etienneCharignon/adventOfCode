mod inputs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
    static ref STAR: Regex = Regex::new(r"[*]").unwrap();
}

pub fn is_symbol(c: char) -> bool {
    let is: bool = match c {
        '.' => false,
        '\n' => false,
        '0'..='9' => false,
        _ => true,
    };
    if c != '.' && is == false {
        println!("{:?}", (c, is));
    }
    is
}

pub fn is_surounded(row_index: usize, from: usize, to: usize, world: &str) -> bool {
    let bytes: Vec<char> = world.chars().collect();
    let row_len = world.chars().position(|c| c == '\n').unwrap() + 1;
    let row_number = world.lines().count();
    let start = if from > 0 { from - 1 } else { from };
    for x in start..=to {
        if row_index + 1 < row_number {
            if is_symbol(bytes[x + (row_index + 1) * row_len]) {
                return true;
            }
        }
        if row_index > 0 && is_symbol(bytes[x + (row_index - 1) * row_len]) {
            return true;
        }
    }
    (from > 0 && is_symbol(bytes[from - 1 + row_index * row_len])) || 
    is_symbol(bytes[to + row_index * row_len])
}

pub fn find_row_part(row_index: usize, row: &str) -> Vec<(usize, usize, usize, u32)> {
    RE.captures_iter(row).map(|c| {
        let m = c.get(0).unwrap();
        (row_index, m.start(), m.end(), c[0].parse::<u32>().unwrap())
    }).collect()
}

pub fn find_parts(input: &str) -> Vec<(usize, usize, usize, u32)> {
    let mut parts = Vec::<(usize, usize, usize, u32)>::new();
    let mut row_index = 0usize;
    for row in input.lines() {
        parts.extend(find_row_part(row_index, row));
        row_index = row_index + 1;
    }
    parts
}

pub fn find_adjacent(parts: &Vec<(usize, usize, usize, u32)>, x: usize, y: usize) -> Vec<u32> {
    println!("{:?}", (x, y));
    let adjacent: Vec<u32> = parts.iter().filter(|p| is_close_to(**p, x, y)).map(|p| p.3).collect();
    println!("{:?}", adjacent);
    if adjacent.iter().count() == 2 {
        adjacent
    }
    else {
        Vec::<u32>::from([0])
    }
}

pub fn solve(input: &str) -> u32 {
    let parts = find_parts(input);
    let row_len = input.chars().position(|c| c == '\n').unwrap() + 1;
    STAR.captures_iter(input).map(|c| {
        let m = c.get(0).unwrap();
        find_adjacent(&parts, m.start()/row_len, m.start()%row_len).iter().product::<u32>()
    }).sum()
}

pub fn is_close_to(part: (usize, usize, usize, u32), r: usize, c: usize) -> bool {
    let fx = if c > 0 { c - 1 } else { c };
    let fy = if r > 0 { r - 1 } else { r };
    for x in fx..=(c + 1) {
        for y in fy..=(r + 1) {
            if x >= part.1 && x < part.2 && y == part.0 {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_close_to() {
        assert_eq!(is_close_to((2, 2, 4, 123), 1, 3), true);
        assert_eq!(is_close_to((0, 0, 3, 123), 1, 3), true);
        assert_eq!(is_close_to((0, 0, 3, 123), 2, 3), false);
    }


    #[test]
    fn it_is_symbol() {
        for c in "*#+$/@".chars() {
            assert_eq!(is_symbol(c), true);
        }
        for c in ".1234567890".chars() {
            assert_eq!(is_symbol(c), false);
        }
    }

    #[test]
    fn it_is_surounded() {
        assert_eq!(is_surounded(1, 1, 4, ".....\n.123."), false);
        assert_eq!(is_surounded(1, 1, 4, "*....\n.123.\n....."), true);
        assert_eq!(is_surounded(1, 1, 4, ".....\n*123.\n....."), true);
        assert_eq!(is_surounded(0, 1, 4, "*123.\n....."), true);
        assert_eq!(is_surounded(6, 2, 5, inputs::EXAMPLE), true);
        assert_eq!(is_surounded(4, 0, 3, inputs::EXAMPLE), true);
        assert_eq!(is_surounded(0, 0, 3, inputs::EXAMPLE), true);
        assert_eq!(is_surounded(0, 5, 8, inputs::EXAMPLE), false);
    }

    #[test]
    fn it_find_parts() {
        assert_eq!(find_parts(".....\n.123."), vec![(1, 1, 4, 123)]);
    }

    #[test]
    fn it_solve_example() {
        assert_eq!(solve(inputs::EXAMPLE), 467835);
    }

    #[test]
    fn it_solve() {
        // assert_eq!(find_parts(inputs::INPUT).iter().sum::<u32>(), 521601);
        assert_eq!(solve(inputs::INPUT), 80694070);
    }
}
