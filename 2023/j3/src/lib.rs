mod inputs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

pub fn is_symbol(c: u8) -> bool {
    let is: bool = match c {
        0x21 ..= 0x2D => true,
        0x3A ..= 0x40 => true,
        0x2F => true,
        _ => false,
    };
    if char::from(c) != '.' && is == false {
        println!("{:?}", (char::from(c), is));
    }
    is
}

pub fn is_surounded(row_index: usize, from: usize, to: usize, world: &str) -> bool {
    let bytes = world.as_bytes();
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
    if from > 0 && is_symbol(bytes[from - 1 + (row_index) * row_len]) {
        return true;
    }
    if is_symbol(bytes[to + (row_index) * row_len]) {
        return true;
    }
    false
}

pub fn find_row_part(row_index: usize, row: &str, world: &str) -> Vec<u32> {
    RE.captures_iter(row).map(|c| {
        let m = c.get(0).unwrap();
        if is_surounded(row_index, m.start(), m.end(), world) {
            c[0].parse::<u32>().unwrap()
        }
        else {
            0
        }
    }).collect()
}

pub fn find_parts(input: &str) -> Vec<u32> {
    let mut parts: Vec<u32> = Vec::new();
    let mut row_index: usize = 0;
    for row in input.lines() {
        parts.extend(find_row_part(row_index, row, input));
        row_index = row_index + 1;
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_symbol() {
        for c in "*#+$/@".as_bytes() {
            assert_eq!(is_symbol(*c), true);
        }
        for c in ".1234567890".as_bytes() {
            assert_eq!(is_symbol(*c), false);
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
    fn it_solve_example() {
        assert_eq!(find_parts(inputs::EXAMPLE).iter().sum::<u32>(), 4361);
    }

    #[test]
    fn it_solve() {
        assert_eq!(find_parts(inputs::INPUT).iter().sum::<u32>(), 521601);
    }
}
