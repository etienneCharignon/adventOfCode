mod inputs;

use std::cmp;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Point(pub i64, pub i64);

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Path {
    pub length: i64,
    pub next: [Option<Box<Path>>; 2],
}

pub fn add(a: Point, b: Point) -> Point {
    Point(a.0 + b.0, a.1 + b.1)
}

pub fn read(input:&str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn right(direction: Point) -> Point {
    match direction {
        Point(1, 0) => Point(0, 1),
        Point(0, 1) => Point(-1, 0),
        Point(0, -1) => Point(1, 0),
        Point(-1, 0) => Point(0, -1),
        _ => panic!("unknown direction")
    }
}

pub fn left(direction: Point) -> Point {
    match direction {
        Point(1, 0) => Point(0, -1),
        Point(0, 1) => Point(1, 0),
        Point(0, -1) => Point(-1, 0),
        Point(-1, 0) => Point(0, 1),
        _ => panic!("unknown direction")
    }
}

pub fn can_walk(pos: Point, direction: Point, field: &Vec<Vec<char>>) -> bool {
    //println!("{pos:?} direction {direction:?}, {}", field[pos.1 as usize][pos.0 as usize]);
    match field[pos.1 as usize][pos.0 as usize] {
        '#' => false,
        '>' => direction == Point(1, 0),
        'v' => direction == Point(0, 1),
        '<' => direction == Point(-1, 0),
        '^' => direction == Point(0, -1),
        _ => true
    }
}

pub fn next_pos(current: Point, current_direction: Point, field: &Vec<Vec<char>>, n: usize) -> Vec<(Point, Point)> {
    let mut positions = vec![];
    let mut new_direction = left(current_direction);
    let mut next = add(current, new_direction);
    if can_walk(next, new_direction, field) {
        positions.push((next, new_direction));
    }

    if positions.len() == n {
        return positions;
    }

    next = add(current, current_direction);
    if can_walk(next, current_direction, field) {
        positions.push((next, current_direction));
    }

    if positions.len() == n {
        return positions;
    }

    new_direction = right(current_direction);
    next = add(current, new_direction);
    if can_walk(next, new_direction, field) {
        positions.push((next, new_direction));
    }

    positions
}

pub fn build_graph(start: Point, start_direction: Point, field: &Vec<Vec<char>>) -> Path {
    // println!("Build graph");
    let mut current = start;
    let mut current_direction = start_direction;
    (current, current_direction) = next_pos(current, current_direction, field, 1)[0];
    let mut length = 1;
    while field[current.1 as usize][current.0 as usize] == '.' && current.1 != field.len() as i64 - 1 {
        // println!("{current:?} : {current_direction:?}");
        (current, current_direction) = next_pos(current, current_direction, field, 1)[0];
        length += 1;
    }
    if current.1 == field.len() as i64 - 1 {
        return Path{length: length + 1, next: [None, None]};
    }

    let crossroad = add(current, current_direction);
    // println!("crossroad {crossroad:?}, {current_direction:?}");
    let exits = next_pos(crossroad, current_direction, field, 2);
    // println!("Exits : {exits:?}");
    let second_next = if exits.len() > 1 {Some(Box::new(build_graph(exits[1].0, exits[1].1, field)))} else { None };
    Path{
        length: length + 2,
        next: [
            Some(Box::new(build_graph(exits[0].0, exits[0].1, field))),
            second_next,
        ]
    }
}

pub fn find_longest(graph: &Path) -> i64 {
    if graph.next == [None, None] {
        return graph.length;
    }
    let left = graph.next[0].clone().unwrap();
    match graph.next[1].clone() {
        None => graph.length + find_longest(&left),
        Some(right) => graph.length + cmp::max(find_longest(&left),find_longest(&right)),
    }
}

pub fn find_longest_path(input: &str) -> i64 {
    let field = read(input);
    println!("{field:?}");
    let start = Point(1, 0);
    let start_direction = Point(0, 1);
    let graph = build_graph(start, start_direction, &field);
    println!("{graph:?}");
    find_longest(&graph) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compute_next_pos() {
        let field = read(inputs::EXAMPLE);
        assert_eq!(next_pos(Point(9, 3), Point(0,-1), &field, 1)[0], (Point(10, 3), Point(1, 0)));
    }

    #[test]
    fn it_solve() {
        assert_eq!(find_longest_path(inputs::EXAMPLE), 94);
        assert_eq!(find_longest_path(inputs::INPUT), 2130);
    }
}
