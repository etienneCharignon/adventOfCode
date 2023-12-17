mod inputs;

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Point(pub i32, pub i32);

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Node {
    pub p: Point,
    pub direction: Point,
    pub c: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.p.0.cmp(&self.p.0)
    }
}

pub fn read(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|l| l.chars().map(|c| String::from(c).parse::<i32>().unwrap()).collect()).collect()
}

pub fn add(a: Point, b: Point) -> Point {
    Point(a.0+b.0, a.1+b.1)
}

pub fn turn_right(d: Point) -> Point {
    match d {
        Point(1, 0) => Point(0, 1),
        Point(0, 1) => Point(-1, 0),
        Point(-1, 0) => Point(0, -1),
        Point(0, -1) => Point(1, 0),
        _ => panic!("direction inconnue")
    }
}

pub fn turn_left(d: Point) -> Point {
    match d {
        Point(1, 0) => Point(0, -1),
        Point(0, 1) => Point(1, 0),
        Point(-1, 0) => Point(0, 1),
        Point(0, -1) => Point(-1, 0),
        _ => panic!("direction inconnue")
    }
}

pub fn next_possibles(n: Node, height: i32, width: i32) -> Vec<Node> {
    let mut list = vec![];

    if n.c < 10 {
        let new_point = add(n.p, n.direction);
        if new_point.0 >= 0 && new_point.0 < width && new_point.1 >= 0 && new_point.1 < height {
            list.push(Node { p: new_point, direction: n.direction, c: n.c + 1 } );
        }
    }
    if n.c >= 4 {
        let right = turn_right(n.direction);
        let new_point = add(n.p, right);
        if new_point.0 >= 0 && new_point.0 < width && new_point.1 >= 0 && new_point.1 < height {
            list.push(Node { p: new_point, direction: right, c: 1 } );
        }

        let left = turn_left(n.direction);
        let new_pointl = add(n.p, left);
        if new_pointl.0 >= 0 && new_pointl.0 < width && new_pointl.1 >= 0 && new_pointl.1 < height {
            list.push(Node { p: new_pointl, direction: left, c: 1 } );
        }
    }
    list
}

pub fn smallest(open_set: &HashSet<Node>, gscore: &HashMap<Node, i32>) -> Node {
    let node = open_set.iter()
            .map(|item| (item, gscore.get(item).clone().unwrap_or(&i32::MAX)))
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
    *node
}

pub fn reconstruct_path(came_from: &HashMap<Node, Node>, start: Node) -> Vec<Node> {
    let mut current = start;
    let mut total_path = vec![current];
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.insert(0, current);
    }
    total_path
}

pub fn print_path(path: &Vec<Node>, width: i32, height: i32) {
    let points: Vec<_> = path.iter().map(|n| n.p).collect();
    for y in 0..height {
        let mut row = String::new();
        for x in 0..width {
            if points.contains(&Point(x,y)) {
                row.push_str(&path[path.iter().position(|n| n.p == Point(x,y)).unwrap()].c.to_string());
            }
            else {
                row.push('.');
            }
        }
        println!("{row}");
    }
    println!("");
}

pub fn print_set(set: &HashSet<Node>, width: i32, height: i32) {
    let points: Vec<_> = set.iter().map(|n| *n).collect();
    print_path(&points, width, height);
}

pub fn find_path(start: Node, goal: Point, map: &Vec<Vec<i32>>) -> i32
{
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut open_set = BinaryHeap::new(); // HashSet::from([start]);
    open_set.push(Reverse((0, start)));
    let mut closed_set = HashSet::new();
    let mut came_from: HashMap<Node, Node> = HashMap::new();

    let mut gscore: HashMap<Node, i32> =HashMap::new();
    gscore.insert(start, 0);

    let mut ends = Vec::<Node>::new();
    while !open_set.is_empty() {
        let Reverse((score, current)) = open_set.pop().unwrap(); //smallest(&open_set, &gscore);
        //print_set(&open_set, width, height);
        // open_set.remove(&current);
        if current.p == goal && current.c >= 4 {
            // let path = reconstruct_path(&came_from, current);
            // println!("{}", gscore.get(&current).unwrap());
            // print_path(&path, width, height);
            return *gscore.get(&current).unwrap();
            // ends.push(current);
            // continue;
        }
        for neighbor in next_possibles(current, height, width) {
            let tentative_gscore = gscore.get(&current).unwrap() + map[neighbor.p.1 as usize][neighbor.p.0 as usize];
            if ! closed_set.contains(&neighbor) && tentative_gscore < gscore.get(&neighbor).cloned().unwrap_or(i32::MAX) {
                came_from.insert(neighbor, current);
                gscore.insert(neighbor, tentative_gscore);
                open_set.push(Reverse((tentative_gscore, neighbor)));
            }
        }
        closed_set.insert(current);
    }

    *ends.iter().map(|n| gscore.get(n).unwrap()).min().unwrap()
}  

pub fn find_minimum_heat_lost_path(map: &Vec<Vec<i32>>) -> i32 {
    find_path(Node { p: Point(0, 0), direction: Point(1, 0), c: 1 }, 
             Point(map[0].len() as i32 - 1, map.len() as i32 - 1),
             map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(find_minimum_heat_lost_path(&read(inputs::EXAMPLE2)), 71);
        assert_eq!(find_minimum_heat_lost_path(&read(inputs::EXAMPLE)), 94);
        assert_eq!(find_minimum_heat_lost_path(&read(inputs::INPUT)), 1157);
    }
}