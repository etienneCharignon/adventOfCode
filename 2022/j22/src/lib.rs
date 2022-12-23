mod inputs;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Human(usize, usize, usize);
struct Pos(i32, i32);

#[allow(dead_code)]
fn read_map(map: &str) -> Vec<&str> {
    map.lines().collect()
}

#[allow(dead_code)]
fn score(human: Human) -> usize {
    1000 * (human.1 + 1) + 4 * (human.0 + 1) + human.2
}

#[allow(dead_code)]
fn extract_path(path: &str) -> Vec<i32> {
    let regex : Regex = Regex::new(r"[0-9]+|[LR]").unwrap();
    regex.find_iter(path)
         .map(|mat| {
            let item = mat.as_str();
            match item {
                "L" => -1,
                "R" => 1,
                _ => item.parse::<i32>().unwrap()
            }
         }).collect::<Vec<i32>>()
}

fn moves_one_step(map: &Vec<&str>, human: &Human, size: usize) -> Human {
    let mut current = *human;
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    loop {
        let mut pos = match current.2 {
            0 => { Pos(current.0 as i32 + 1, current.1 as i32) },
            1 => { Pos(current.0 as i32, current.1 as i32 + 1) },
            2 => { Pos(current.0 as i32 - 1, current.1 as i32) },
            3 => { Pos(current.0 as i32, current.1 as i32 - 1) },
            _ => panic!("direction impossible")
        };
        if pos.1 < 0 { pos.1 = height - 1; }
        else if pos.1 >= height { pos.1 = 0; }
        if pos.0 < 0 { pos.0 = width - 1; }
        else if pos.0 >= width { pos.0 = 0; }

        let cell = map[pos.1 as usize].chars().nth(pos.0 as usize);
        if cell == Some(' ') {
            current = change_face(pos, current.2 as i32,  size as i32);
        }
        else {
            current = Human(pos.0 as usize, pos.1 as usize, current.2);
        }
        let cell = map[current.1 as usize].chars().nth(current.0 as usize);
        match cell {
            Some('.') => { return current },
            Some('#') => { return *human; },
            _ => panic!("charactère inconnu {}", cell.unwrap())
        }
    }
}

fn turns(h: Human, cmd: i32) -> Human {
    let direction: usize = if h.2 == 0 && cmd < 0 { 3 } else { ((h.2 as i32 + cmd) as usize) % 4 };
    Human(h.0, h.1, direction)
}

#[allow(dead_code)]
fn walk(map: &Vec<&str>, path: &Vec<i32>, human: Human, size: usize) -> Human {
    let mut h = human;
    for (i, cmd) in path.iter().enumerate() {
        if i % 2 == 0 {
            // moves
            for _ in 0..*cmd {
                h = moves_one_step(map, &h, size);
            }
        }
        else {
            // turns
            h = turns(h, *cmd)
        }
    }
    h
}

fn change_face(pos: Pos, direction: i32, size: i32) -> Human {
    // < 1
    if pos.0 == size - 1 && (0..size).contains(&pos.1) && direction == 2 {
        Human(0, (3 * size - 1 - pos.1) as usize, 0)
    }
    // < 3
    else if pos.0 == size - 1 && (size..(2*size)).contains(&pos.1) && direction == 2 {
        Human((pos.1 - size) as usize, (2 * size) as usize, 1)
    }
    // ^ 5
    else if pos.1 == 2 * size - 1 && (0..size).contains(&pos.0) {
        Human(size as usize, (pos.0 + size) as usize, 0)
    }
    // < 5
    else if pos.0 == 3 * size - 1 && (2*size..3*size).contains(&pos.1) {
        Human(size as usize, (size - (pos.1 - 2*size) - 1) as usize, 0)
    }
    // < 6
    else if pos.0 == 3 * size - 1 && (3*size..4*size).contains(&pos.1) && direction == 2 {
        Human((size + (pos.1 - 3*size)) as usize, 0, 1)
    }
    // v 6
    else if pos.1 == 0 && (0..size).contains(&pos.0) && direction == 1 {
        Human((pos.0 + 2*size) as usize, 0, 1)
    }
    // > 6
    else if pos.0 == size && (3*size..4*size).contains(&pos.1) && direction == 0 {
        Human(((pos.1 - 3*size) + size) as usize, (3*size - 1) as usize, 3)
    }
    // v 4
    else if pos.1 == 3*size && (size..2*size).contains(&pos.0) {
        Human((size - 1) as usize, (pos.0 - size + 3*size) as usize, 2)
    }
    // > 4
    else if pos.0 == 2*size && (2*size..3*size).contains(&pos.1) {
        Human((3*size - 1) as usize, (size - (pos.1 - 2*size) - 1) as usize, 2)
    }
    // > 3
    else if pos.0 == 2*size && (size..2*size).contains(&pos.1) && direction == 0 {
        Human((2*size + (pos.1 - size)) as usize, (size - 1) as usize, 3)
    }
    // v 2
    else if pos.1 == size && (2*size..3*size).contains(&pos.0) {
        Human((2*size - 1) as usize, (size + (pos.0 - 2*size)) as usize, 2)
    }
    // > 2
    else if pos.0 == 0 && (0..size).contains(&pos.1) {
        Human((2*size - 1) as usize, (3*size - pos.1 - 1) as usize, 2)
    }
    // ^ 2
    else if pos.1 == 4*size - 1 && (2*size..3*size).contains(&pos.0) {
        Human((pos.0 - 2*size) as usize, pos.1 as usize, 3)
    }
    // ^ 1
    else if pos.1 == 4*size - 1 && (size..2*size).contains(&pos.0) {
        Human(0, (pos.0 - size + 3*size) as usize, 0)
    }
    else {
        panic!("work in progress")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_map() {
        assert_eq!(read_map(inputs::MAP).len(), 200);
    }

    #[test]
    fn it_change_face() {
        // < 1
        assert_eq!(change_face(Pos(49, 0), 2, 50), Human(0,149,0));
        assert_eq!(change_face(Pos(49, 1), 2, 50), Human(0,148,0));
        // < 3
        assert_eq!(change_face(Pos(49, 50), 2, 50), Human(0,100,1));
        assert_eq!(change_face(Pos(49, 51), 2, 50), Human(1,100,1));
        assert_eq!(change_face(Pos(49, 99), 2, 50), Human(49,100,1));
        // ^ 5
        assert_eq!(change_face(Pos(1, 99), 3, 50), Human(50,51,0));
        assert_eq!(change_face(Pos(49, 99), 3, 50), Human(50,99,0));
        // < 5
        assert_eq!(change_face(Pos(149, 100), 2, 50), Human(50,49,0));
        assert_eq!(change_face(Pos(149, 101), 2, 50), Human(50,48,0));
        // < 6
        assert_eq!(change_face(Pos(149, 150), 2, 50), Human(50,0,1));
        assert_eq!(change_face(Pos(149, 199), 2, 50), Human(99,0,1));
        // v 6
        assert_eq!(change_face(Pos(0, 0), 1, 50), Human(100,0,1));
        assert_eq!(change_face(Pos(49, 0), 1, 50), Human(149,0,1));
        // > 6
        assert_eq!(change_face(Pos(50, 150), 0, 50), Human(50,149,3));
        assert_eq!(change_face(Pos(50, 151), 0, 50), Human(51,149,3));
        // v 4
        assert_eq!(change_face(Pos(50, 150), 1, 50), Human(49,150,2));
        assert_eq!(change_face(Pos(99, 150), 1, 50), Human(49,199,2));
        // > 4
        assert_eq!(change_face(Pos(100, 100), 0, 50), Human(149,49,2));
        assert_eq!(change_face(Pos(100, 149), 0, 50), Human(149,0,2));
        // > 3
        assert_eq!(change_face(Pos(100, 50), 0, 50), Human(100,49,3));
        assert_eq!(change_face(Pos(100, 99), 0, 50), Human(149,49,3));
        // v 2
        assert_eq!(change_face(Pos(100, 50), 1, 50), Human(99,50,2));
        assert_eq!(change_face(Pos(149, 50), 1, 50), Human(99,99,2));
        // > 2
        assert_eq!(change_face(Pos(0, 0), 0, 50), Human(99,149,2));
        assert_eq!(change_face(Pos(0, 49), 0, 50), Human(99,100,2));
        // ^ 2
        assert_eq!(change_face(Pos(100, 199), 3, 50), Human(0,199,3));
        assert_eq!(change_face(Pos(149, 199), 3, 50), Human(49,199,3));
        // ^ 1
        assert_eq!(change_face(Pos(50, 199), 3, 50), Human(0,150,0));
        assert_eq!(change_face(Pos(99, 199), 3, 50), Human(0,199,0));
    }

    #[test]
    fn it_score() {
        assert_eq!(score(Human(7, 5, 0)), 6032);
        assert_eq!(score(Human(115, 42, 2)), 43466);
        assert_eq!(score(Human(37, 161, 3)), 162155);
    }

    #[test]
    fn it_turns() {
        assert_eq!(turns(Human(0, 0, 0), 1), Human(0, 0, 1));
        assert_eq!(turns(Human(0, 0, 3), 1), Human(0, 0, 0));
        assert_eq!(turns(Human(0, 0, 1), -1), Human(0, 0, 0));
        assert_eq!(turns(Human(0, 0, 0), -1), Human(0, 0, 3));
    }

    #[test]
    fn it_extract_path() {
        assert_eq!(extract_path(inputs::EXAMPLE_PATH), vec![ 10, 1, 5, -1, 5, 1, 10, -1, 4, 1, 5, -1, 5 ]);
    }

    #[test]
    fn it_walk() {
    //    assert_eq!(walk(&read_map(inputs::EXAMPLE_MAP), &extract_path(inputs::EXAMPLE_PATH), Human(8, 0, 0), 4), Human(6, 4, 3));
        assert_eq!(walk(&read_map(inputs::MAP), &extract_path(inputs::PATH), Human(50, 0, 0), 50), Human(37, 161, 3));
    }
}
