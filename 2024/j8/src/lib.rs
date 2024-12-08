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

pub fn compute_distance(p1: &Point, p2: &Point) -> Point {
    Point { x: p2.x - p1.x , y:  p2.y - p1.y }
}

pub fn substract(p: &Point, distance: Point) -> Point {
    Point { x: p.x - distance.x, y: p.y - distance.y }
}

pub fn inside_map(p : Point, height: i32, width: i32) -> bool {
    p.x >=0 && p.x < width && p.y >= 0 && p.y < height
}

pub fn count_antinode(map: (MultiMap<char, Point>, i32, i32)) -> usize {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (frequency, positions) in map.0.iter_all() {
        println!("Key: {}, Value: {:?}", frequency, positions);
        for p1 in positions {
            for p2 in positions {
                if *p1 == *p2 { continue; }
                let distance = compute_distance(p1, p2);
                let position = substract(p1, distance);
                if inside_map(position, map.1, map.2) {
                    antinodes.insert(position);
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
        assert_eq!(count_antinode(read_map(inputs::EXAMPLE)), 14);
        assert_eq!(count_antinode(read_map(inputs::INPUT)), 311);
    }
}
