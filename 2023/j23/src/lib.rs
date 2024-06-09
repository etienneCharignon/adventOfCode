mod inputs;

use multimap::MultiMap;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Point(pub i64, pub i64);

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

pub fn can_walk(pos: Point, _direction: Point, field: &Vec<Vec<char>>) -> bool {
    //println!("{pos:?} direction {direction:?}, {}", field[pos.1 as usize][pos.0 as usize]);
    if pos.1 < 0 || pos.0 < 0 || pos.1 >= field.len() as i64 || pos.0 >= field[0].len() as i64 {
        return false;
    }
    match field[pos.1 as usize][pos.0 as usize] {
        '#' => false,
        /*
        '>' => direction == Point(1, 0),
        'v' => direction == Point(0, 1),
        '<' => direction == Point(-1, 0),
        '^' => direction == Point(0, -1),
        */
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

pub fn compute_id(p1: Point, p2:Point) -> (Point, Point) {
    if p1.0 < p2.0 {
        (p1, p2)
    }
    else if p1.0 > p2.0 {
        (p2, p1)
    }
    else if p1.1 < p2.1 {
        (p1, p2)
    }
    else {
        (p2, p1)
    }
}

pub fn build_graph(start: Point, start_direction: Point, field: &Vec<Vec<char>>,
                   nexts_path: &mut MultiMap<Point, Point>, lengths: &mut HashMap<Point, usize>,
                   path_ids: &mut HashMap<Point, (Point, Point)>) {
    // println!("Build graph");
    let mut current = start;
    let mut end = current;
    let mut current_direction = start_direction;
    (current, current_direction) = next_pos(current, current_direction, field, 1)[0];
    let mut length = 1;
    let mut next_positions = vec![];
    loop {
        // println!("{current:?} : {current_direction:?}");
        length += 1;
        next_positions = next_pos(current, current_direction, field, 3);
        if next_positions.len() == 0 {
            lengths.insert(start, length);
            let id = compute_id(start, current);
            path_ids.insert(start, id);
            path_ids.insert(current, id);
            return;
        }
        if next_positions.len() > 1 {
            break;
        }
        end = current;
        (current, current_direction) = next_positions[0]
    }

    // println!("crossroad {crossroad:?}, {current_direction:?}");
    // println!("Exits : {next_positions:?}");
    lengths.insert(start, length);
    lengths.insert(end, length);
    let id = compute_id(start, end);
    path_ids.insert(start, id);
    path_ids.insert(end, id);
    for exit in next_positions.clone() {
        nexts_path.insert(start, exit.0);
    }

    for exit in next_positions {
        if ! nexts_path.contains_key(&exit.0) {
            build_graph(exit.0, exit.1, field, nexts_path, lengths, path_ids);
        }
    }
}

pub fn print_visited(start: Point, visited: &mut Vec<(Point, Point)>, lengths: &HashMap<Point, usize>) {
    let mut printout = String::new();
    println!("{start:?}");
    for path in visited.clone() {
        printout += &format!("{path:?}, ");
    }
    println!("{printout}");
    printout = String::new();
    let mut count = 0;
    for path in visited {
        let length = match lengths.get(&path.0) { None => 0, Some(l) => *l};
        count += length;
        printout += &format!("{:?}, ", length );
    }
    println!("{count}: {printout}");
}

pub fn find_longest(start: Point, nexts_path: &MultiMap<Point, Point>, lengths: &HashMap<Point, usize>,
                    path_ids: &HashMap<Point, (Point, Point)>, visited: &mut Vec<(Point, Point)>, nexts: &Vec<Point>) -> Option<usize> {
    let id = path_ids.get(&start).unwrap();
    if visited.contains(&id) {
        return None;
    }
    for next in nexts {
        let id_next = path_ids.get(&next).unwrap();
        visited.push(*id_next);
    }

    let start_length = lengths.get(&start).unwrap();
    if ! nexts_path.contains_key(&start) {
        // print_visited(start, visited, lengths);
        return Some(*start_length as usize);
    }
 
    let nexts = nexts_path.get_vec(&start).unwrap();
    nexts.iter()
         .map(|next| find_longest(*next, nexts_path, lengths, path_ids, &mut visited.clone(), &nexts))
         .reduce(Option::max)
         .flatten()
         .and_then(|next_longest_length| {
            // println!("{} : {start:?}\n{next_longest:?}\n{visited:?}", *next_longest_length);
            Some(start_length + next_longest_length)
         })
}

pub fn print_graph(nexts_path: &MultiMap<Point, Point>, lengths: &HashMap<Point, usize>, path_ids: &HashMap<Point, (Point, Point)>) {
    for (key, value) in nexts_path {
        println!("length {:02}: {key:?}, {value:?}", lengths.get(&key).unwrap());
    }
    println!("Path Ids:");
    for (key, value) in path_ids {
        println!("{key:?}: {value:?}");
    }
}

pub fn find_longest_path(input: &str) -> usize {
    let field = read(input);
    println!("{field:?}");
    let start = Point(1, 0);
    let start_direction = Point(0, 1);
    let mut nexts_path = MultiMap::<Point, Point>::new();
    let mut lengths = HashMap::<Point, usize>::new();
    let mut path_ids = HashMap::<Point, (Point, Point)>::new();
    build_graph(start, start_direction, &field, &mut nexts_path, &mut lengths, &mut path_ids);
    print_graph(&nexts_path, &lengths, &path_ids);
    find_longest(start, &nexts_path, &lengths, &path_ids, &mut Vec::<(Point, Point)>::new(), &vec![]).unwrap() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compute_next_pos() {
        let field = read(inputs::EXAMPLE);
        assert_eq!(next_pos(Point(9, 3), Point(0,-1), &field, 1)[0], (Point(10, 3), Point(1, 0)));
        assert_eq!(next_pos(Point(19, 18), Point(0, 1), &field, 3), vec![(Point(19, 19), Point(0, 1))]);
        assert_eq!(next_pos(Point(19, 19), Point(0, 1), &field, 3), vec![(Point(19, 20), Point(0, 1)), (Point(18, 19), Point(-1, 0))]);
    }

    #[test]
    fn find_max_in_list_of_optional() {
        assert_eq!(
            [Some(1), Some(2), None]
                .into_iter()
                .reduce(Option::max)
                .and_then(|o| o),
            Some(2)
        );
        let list: Vec<Option<u32>> = vec![];
        assert_eq!(
            list
                .into_iter()
                .reduce(Option::max)
                .and_then(|o| o),
            None
        );
        let list: Vec<Option<u32>> = vec![None];
        assert_eq!(
            list
                .into_iter()
                .reduce(Option::max)
                .and_then(|o| o),
            None
        );
    }

    #[test]
    fn it_solve() {
        assert_eq!(find_longest_path(inputs::EXAMPLE), 154);
        assert_eq!(find_longest_path(inputs::INPUT), 6710);
    }
}
