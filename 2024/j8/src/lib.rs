mod inputs;

use multimap::MultiMap;
use std::collections::HashSet;


#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn read_map(input: &str) -> (MultiMap<char, Point>, i32, i32) {
    let mut map = MultiMap::new();
    let lines: Vec<&str> = input.split('\n').collect();
    let height = lines.len();
    let width = lines[0].len();
    for (r, line) in lines.iter().enumerate() {
        for (c, a) in line.chars().enumerate() {
            if a != '.' {
                map.insert(a, Point { x: c as i32, y: r as i32})
            }
        }
    }
    (map, height as i32, width as i32)
}

pub fn substract(p: &Point, distance: &Point) -> Point {
    Point { x: p.x - distance.x, y: p.y - distance.y }
}

pub fn outside_map(p : &Point, height: i32, width: i32) -> bool {
    p.x <0 || p.x >= width || p.y < 0 || p.y >= height
}

pub fn count_antinode(map: (MultiMap<char, Point>, i32, i32)) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (frequency, positions) in map.0.iter_all() {
        println!("Key: {}, Value: {:?}", frequency, positions);
        for p1 in positions {
            for p2 in positions {
                if p1 == p2 { continue; }

                let distance = substract(p2, p1);
                let mut antinode_pos = *p1;
                loop {
                    if outside_map(&antinode_pos, map.1, map.2) { break; }

                    antinodes.insert(antinode_pos);
                    antinode_pos = substract(&antinode_pos, &distance);
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(count_antinode(read_map(inputs::EXAMPLE)), 34);
        assert_eq!(count_antinode(read_map(inputs::INPUT)), 1115);
    }
}
