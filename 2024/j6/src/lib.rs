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

pub fn next_pos(pos: (i32, i32), world: &Vec<Vec<char>>, direction: (i32, i32), obstacle_meetings: &mut HashSet<((i32, i32),(i32,i32))>) -> Option<((i32, i32), (i32, i32))> {
    let mut new_direction = direction;
    let mut new_pos = add(pos, new_direction);
    if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= world[0].len() as i32 || new_pos.1 >= world.len() as i32 {
        return None;
    }
    let cell = world[new_pos.1 as usize][new_pos.0 as usize];
    if cell == '#' {
        let collision = ((new_pos),(new_direction));
        if obstacle_meetings.contains(&collision) {
            return Some(((-1, -1),(-1, -1)))
        }
        obstacle_meetings.insert(collision);
        new_direction = turn_right(new_direction);
        return Some((pos, new_direction));
    }
    Some((new_pos, new_direction))
}

pub fn find_visited(map: ((i32, i32), Vec<Vec<char>>)) -> Option<HashSet<(i32,i32)>> {
    let (mut pos, world) = map;
    let mut direction = (0, -1);
    let mut visited = HashSet::new();
    visited.insert(pos);
    let mut obstacle_meetings = HashSet::new();
    loop {
        match next_pos(pos, &world, direction, &mut obstacle_meetings) {
            Some((new_pos, new_direction)) => {
                if new_pos == (-1, -1) { // boucle
                    return None;
                }
                pos = new_pos;
                direction = new_direction;
                visited.insert(new_pos);
            },
            _ => {
                break;
            }
        }
    }
    Some(visited)
}

pub fn count_obstacle_position(map: ((i32, i32), Vec<Vec<char>>)) -> usize {
    let (origin, world) = map;
    let mut possible_pos = find_visited((origin, world.clone())).unwrap();
    possible_pos.remove(&origin);
    let mut count = 0;
    for pos in possible_pos {
        let  mut world_with_obstacle = world.clone();
        world_with_obstacle[pos.1 as usize][pos.0 as usize] = '#';
        if find_visited((origin, world_with_obstacle)) == None {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_cells() {
        assert_eq!(find_visited(read_world(inputs::EXAMPLE)).unwrap().len(), 41);
        assert_eq!(find_visited(read_world(inputs::INPUT)).unwrap().len(), 5318);
    }

    #[test]
    fn it_count_obstacle_positions() {
        assert_eq!(count_obstacle_position(read_world(inputs::EXAMPLE)), 6);
        assert_eq!(count_obstacle_position(read_world(inputs::INPUT)), 1831); // too high
    }
}
