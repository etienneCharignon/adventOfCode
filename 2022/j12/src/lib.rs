pub mod inputs;

#[allow(unused_imports)]
use assert_approx_eq::assert_approx_eq;
use inputs::Point;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::Write;
use std::{thread, time};

const DIRECTIONS: [Point; 4] = [
        Point(0, 1),
        Point(1, 0),
        Point(0, -1),
        Point(-1, 0),
    ];

#[allow(dead_code)]
pub fn read_map(input: &str) -> (Vec<Vec<i32>>, Point, Point) {
    let mut result = vec![];
    let mut start: Point= inputs::START_EXAMPLE;
    let mut end: Point= inputs::END_EXAMPLE;
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, h) in line.chars().enumerate() {
            if h == 'S' {
                start = Point(x as i32, y as i32);
                row.push(0 as i32);
            }
            else if h == 'E' {
                end = Point(x as i32, y as i32);
                row.push(25 as i32);
            }
            else {
                row.push(('a'..='z').position(|c| c == h).unwrap() as i32);
            }
        }
        result.push(row)
    }
    (result, start, end)
}

#[allow(dead_code)]
fn add(a: Point, b: Point) -> Point {
    Point(a.0+b.0, a.1+b.1)
}

#[allow(dead_code)]
pub fn length(a: Point, b: Point) -> f32 {
    let distance = Point(b.0 - a.0, b.1 - a.1);
    ((distance.0 * distance.0 + distance.1 * distance.1) as f32).sqrt()
}

#[allow(dead_code)]
fn neighbors(p: Point) -> Vec<Point> {
    let mut neighbors = vec![];
    for d in DIRECTIONS {
        neighbors.push(add(p, d));
    }
    neighbors
}

#[allow(dead_code)]
fn reconstruct_path(came_from: &HashMap<Point, Point>, start: Point) -> Vec<Point> {
    let mut current = start;
    let mut total_path = vec![current];
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.insert(0, current);
    }
    total_path
}

#[allow(dead_code)]
fn smallest(open_set: &HashSet<Point>, fscore: &HashMap<Point, f32>) -> Point {
    let point = open_set.iter()
            .map(|item| (item, fscore.get(item).clone().unwrap_or(&f32::MAX)))
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
    *point
}

#[allow(dead_code)]
pub fn find_path<H, D>(start: Point, goal: Point, h: H, d: D) -> Vec<inputs::Point>
where
    H: Fn(Point) -> f32,
    D: Fn(Point, Point) -> f32,
{
    let mut open_set = HashSet::from([start]);
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut gscore: HashMap<Point, f32> =HashMap::new();
    gscore.insert(start, 0f32);
    let mut fscore: HashMap<Point, f32> =HashMap::new();
    fscore.insert(start, h(start));
    while !open_set.is_empty() {
        let current = smallest(&open_set, &fscore).to_owned();
        let map_str = to_string(inputs::SCREEN, reconstruct_path(&came_from, current));
        println!("{}", map_str);
        std::io::stdout().flush();
        thread::sleep(time::Duration::from_millis(10));
        if current == goal {
            return reconstruct_path(&came_from, current);
        }
        for neighbor in neighbors(current) {
            let tentative_gscore = gscore[&current] + d(current, neighbor);
            if tentative_gscore < gscore.get(&neighbor).cloned().unwrap_or(f32::MAX) {
                came_from.insert(neighbor, current);
                gscore.insert(neighbor, tentative_gscore);
                fscore.insert(neighbor, tentative_gscore + h(neighbor));
                open_set.insert(neighbor);
            }
        }
        open_set.remove(&current);
    }
    panic!("path not found");
}  

#[allow(dead_code)]
fn to_string(input: &str, map: Vec<Point>) -> String {
    let mut result = String::from(input);
    for point in map {
        let position: usize =  (point.0 + point.1 * 104) as usize;
        result.replace_range(position..position+1,"*");
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_the_map() {
        assert_eq!(read_map(inputs::EXAMPLE).0, vec![
            vec![25, 25, 25, 25, 25, 25, 25, 25, 25, 25],
            vec![25, 0, 0, 1, 16, 15, 14, 13, 12, 25],
            vec![25, 0, 1, 2, 17, 24, 23, 23, 11, 25],
            vec![25, 0, 2, 2, 18, 25, 25, 23, 10, 25],
            vec![25, 0, 2, 2, 19, 20, 21, 22, 9, 25],
            vec![25, 0, 1, 3, 4, 5, 6, 7, 8, 25],
            vec![25, 25, 25, 25, 25, 25, 25, 25, 25, 25],
        ]);
    }

    #[test]
    fn it_find_path() {
        let map = read_map(inputs::EXAMPLE).0;
        let h = |p: Point| -> f32 {
            length(p, inputs::END_EXAMPLE)
        };
        let d = |a: Point, b: Point| -> f32 {
            let cost = ((map[a.1 as usize][a.0 as usize] - map[b.1 as usize][b.0 as usize]).abs() + 1) as f32;
            if cost <= 2_f32 { cost } else { f32::MAX }
        };
        assert_approx_eq!(h(Point(1, 1)), 29_f32.sqrt(), 0.01f32);
        assert_approx_eq!(d(Point(1, 1), Point(2, 1)), 1_f32, 0.01f32);
        assert_approx_eq!(d(Point(1, 2), Point(2, 2)), 2_f32, 0.01f32);
        assert_approx_eq!(d(Point(1, 1), Point(1, 0)), f32::MAX, 0.01f32);
        assert_approx_eq!(d(Point(2, 5), Point(3, 5)), f32::MAX, 0.01f32);
        assert_eq!(find_path(inputs::START_EXAMPLE, inputs::END_EXAMPLE, h, d).iter().count() - 1, 31);
    }

    #[test]
    fn it_solve() {
        let data = read_map(inputs::INPUT);
        let map = data.0;
        let start = data.1;
        let end = data.2;
        assert_eq!(start, Point(1,21));
        assert_eq!(end, Point(78,21));
        let h = |p: Point| -> f32 {
            length(p, end)
        };
        let d = |c: Point, n: Point| -> f32 {
            let cost = ((map[n.1 as usize][n.0 as usize] - map[c.1 as usize][c.0 as usize]) + 1) as f32;
            if cost <= 2_f32 { 1_f32 } else { f32::MAX }
        };
        assert_eq!(find_path(start, end, h, d).iter().count() -1, 361);
    }

}
