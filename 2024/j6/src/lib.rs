mod inputs;

use std::collections::HashSet;


pub fn read_world(input: &str) -> ((i32, i32), Vec<Vec<char>>) {
    let mut world = vec![];
    let mut origin = (0, 0);
    for (r, line) in input.split('\n').into_iter().enumerate() {
        let mut row: Vec<char> = line.chars().collect();
        match line.find('^') {
            Some(c) => {
                origin = (c as i32, r as i32);
                row[c] = '.';
            }
            _ => {}
        }
        world.push(row);
    }
    (origin, world)
}

pub fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    ((a.0 + b.0), (a.1 + b.1))
}

pub fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    (-direction.1, direction.0)
}

pub fn next_pos(pos: (i32, i32), world: &Vec<Vec<char>>, direction: (i32, i32)) -> Option<((i32, i32), (i32, i32))> {
    let mut new_direction = direction;
    let mut new_pos = add(pos, new_direction);
    if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= world[0].len() as i32 || new_pos.1 >= world.len() as i32 {
        return None;
    }
    let cell = world[new_pos.1 as usize][new_pos.0 as usize];
    println!("{cell}");
    if cell == '#' {
        new_direction = turn_right(new_direction);
        new_pos = add(pos, new_direction);
    }
    println!("{:?}", new_pos);
    Some((new_pos, new_direction))
}

pub fn count_cells(map: ((i32, i32), Vec<Vec<char>>)) -> usize {
    let (mut pos, world) = map;
    let mut direction = (0, -1);
    let mut visited = HashSet::new();
    visited.insert(pos);
    loop {
        match next_pos(pos, &world, direction) {
            Some((new_pos, new_direction)) => {
                pos = new_pos;
                direction = new_direction;
                visited.insert(new_pos);
            },
            _ => {
                break;
            }
        }

    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_cells() {
        assert_eq!(count_cells(read_world(inputs::EXAMPLE)), 41);
        assert_eq!(count_cells(read_world(inputs::INPUT)), 5318);
    }
}
