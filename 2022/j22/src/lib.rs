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

fn is_free(map: &Vec<&str>, human: Human) -> bool {
    let cell = map[human.1].chars().nth(human.0);
    match cell {
        None => panic!("should not be none"),
        Some('.') => true,
        Some('#') => false,
        _ => panic!("charactère inconnu {}", cell.unwrap())
    }
}

fn moves_one_step(map: &Vec<&str>, human: &Human) -> Human {
    let mut current = *human;
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    loop {
        let mut pos = match current.2 {
            0 => { Pos(current.0 as i32 + 1, current.1 as i32)},
            1 => { Pos(current.0 as i32, current.1 as i32 + 1)},
            2 => { Pos(current.0 as i32 - 1, current.1 as i32)},
            3 => { Pos(current.0 as i32, current.1 as i32 - 1)},
            _ => panic!("direction impossible")
        };
        if pos.1 < 0 { pos.1 = height - 1; }
        else if pos.1 >= height { pos.1 = 0; }
        if pos.0 < 0 { pos.0 = width - 1; }
        else if pos.0 >= width { pos.0 = 0; }
        let cell = map[pos.1 as usize].chars().nth(pos.0 as usize);
        let new_h = Human(pos.0 as usize, pos.1 as usize, human.2);
        match cell {
            None => {
                current = new_h;
                continue;
            },
            Some(' ') => {
                current = new_h;
                continue;
            },
            Some('.') => { return new_h; },
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
fn walk(map: &Vec<&str>, path: &Vec<i32>, human: Human) -> Human {
    let mut h = human;
    for (i, cmd) in path.iter().enumerate() {
        if i % 2 == 0 {
            // moves
            for _ in 0..*cmd {
                h = moves_one_step(map, &h);
            }
        }
        else {
            // turns
            h = turns(h, *cmd)
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_map() {
        assert_eq!(read_map(inputs::MAP).len(), 200);
    }

    #[test]
    fn it_score() {
        assert_eq!(score(Human(7, 5, 0)), 6032);
        assert_eq!(score(Human(115, 42, 2)), 43466);
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
        assert_eq!(walk(&read_map(inputs::EXAMPLE_MAP), &extract_path(inputs::EXAMPLE_PATH), Human(8, 0, 0)), Human(7, 5, 0));
        assert_eq!(walk(&read_map(inputs::MAP), &extract_path(inputs::PATH), Human(50, 0, 0)), Human(115, 42, 2));
    }
}
