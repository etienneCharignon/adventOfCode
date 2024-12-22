mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::min;
use std::collections::HashSet;

lazy_static! {
    static ref RE: Regex = Regex::new(r"([0-9]+)").unwrap();
}


#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}

pub fn add(a: Pos, b: &Pos) -> Pos {
    Pos {x: a.x + b.x, y: a.y + b.y }
}

pub fn numeric_pos(key: char) -> Pos {
    match key {
        '7' => Pos {x: 0, y: 0},
        '8' => Pos {x: 1, y: 0},
        '9' => Pos {x: 2, y: 0},
        '4' => Pos {x: 0, y: 1},
        '5' => Pos {x: 1, y: 1},
        '6' => Pos {x: 2, y: 1},
        '1' => Pos {x: 0, y: 2},
        '2' => Pos {x: 1, y: 2},
        '3' => Pos {x: 2, y: 2},
        '0' => Pos {x: 1, y: 3},
        'A' => Pos {x: 2, y: 3},
        _ => panic!("Unknown position")
    }
}

pub fn directionnal_pos(key: char) -> Pos {
    match key {
        '^' => Pos {x: 1, y: 0},
        'A' => Pos {x: 2, y: 0},
        '<' => Pos {x: 0, y: 1},
        'v' => Pos {x: 1, y: 1},
        '>' => Pos {x: 2, y: 1},
        _ => panic!("Unknown position")
    }
}

pub fn move_to_key(from: Pos, to:Pos, h_first: bool, blank: Pos) -> String {
    // println!("{from:?}, {to:?}");
    let mut sequence = String::new();
    let mut start_with_h =  h_first;
    if from.y == blank.y {
        start_with_h = false;
    }
    if from.x == blank.x {
        start_with_h = true;
    }
    if start_with_h {
        let delta_x = to.x - from.x;
        if delta_x > 0 {
            sequence += &">".repeat( delta_x as usize);
        }
        else {
            sequence += &"<".repeat( (-delta_x) as usize);
        }

        let delta_y = to.y - from.y;
        if delta_y > 0 {
            sequence += &"v".repeat( delta_y as usize);
        }
        else {
            sequence += &"^".repeat( (-delta_y) as usize);
        }
    }
    else {
        let delta_y = to.y - from.y;
        if delta_y > 0 {
            sequence += &"v".repeat( delta_y as usize);
        }
        else {
            sequence += &"^".repeat( (-delta_y) as usize);
        }

        let delta_x = to.x - from.x;
        if delta_x > 0 {
            sequence += &">".repeat( delta_x as usize);
        }
        else {
            sequence += &"<".repeat( (-delta_x) as usize);
        }
    }
    sequence
}

pub fn add_shortests(sequences: Vec<String>, shortests: HashSet<String>) -> Vec<String> {
    let mut new_sequences = vec![];
    for sequence in sequences {
        for shortest in &shortests {
            new_sequences.push(sequence.clone() + &shortest);
        }
    }
    new_sequences
}

pub fn sequence_to_pad(to_pos: fn(char) -> Pos, blank: Pos, received_shortests: Vec<String>) -> Vec<String> {
    let mut shortest = HashSet::new();
    let mut current: char = 'A';
    let mut min_len = usize::MAX;
    for code in received_shortests {
        let mut sequences = vec![];
        sequences.push(String::new());
        for dest in code.chars() {
            let mut shortest_for_char = HashSet::new();
            let mut min_for_char = usize::MAX;
            for priority in [true, false] {
                let mut s = String::new();
                s += &move_to_key(to_pos(current), to_pos(dest), priority, blank);
                s += "A";
                min_for_char = min(min_for_char, s.len());
                shortest_for_char.insert(s);
            }
            sequences = add_shortests(sequences, shortest_for_char);
            current = dest;
        }
        for sequence in sequences {
            min_len = min(min_len, sequence.len());
            shortest.insert(sequence);
        }
    }
    shortest.into_iter().filter(|sequence| sequence.len() == min_len).collect()
}

pub fn final_sequence(code: &str) -> String {
    sequence_to_pad(directionnal_pos, Pos {x: 0, y: 0},
        sequence_to_pad(directionnal_pos, Pos {x: 0, y: 0},
            sequence_to_pad(numeric_pos, Pos {x: 0, y: 3},
                vec![String::from(code)]))).iter().next().unwrap().to_string()
}

pub fn complexity(code: &str) -> usize {
    let sequence = final_sequence(code);
    println!("{code}: {}, {sequence}", sequence.len());

    sequence.len() * RE.find(code).unwrap().as_str().parse::<usize>().unwrap()
}

pub fn complexities(input: &str) -> usize {
    input.split("\n").map(|code| { complexity(code) }).sum()
}

// example 029A: <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// my            <<vAA>A>^AvAA<^A>A<<vA>>^AvA^A<vA>^A<<vA>^A>AAvA^A<<vA>A>^AAAvA<^A>A
// example 980A: <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A
// my      980A: <<vA>>^AAAvA^A<<vAA>A>^AvAA<^A>A<<vA>A>^AAAvA<^A>A<vA>^A<A>A
// example 179A: <v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
// my      179A: <<vAA>A>^AAvA<^A>AvA^A<<vA>>^AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A
// example 456A: <v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A
// my      456A: <<vAA>A>^AAvA<^A>AAvA^A<vA>^A<A>A<vA>^A<A>A<<vA>A>^AAvA<^A>A
// example 379A: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
// my      379A: <<vA>>^AvA^A<<vAA>A>^AAvA<^A>AAvA^A<vA>^AA<A>A<<vA>A>^AAAvA<^A>A

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_move_to_key() {
        assert_eq!(move_to_key(numeric_pos('A'), numeric_pos('1'), false, Pos { x: -1, y: -1 }),
                   String::from("^<<"));
        assert_eq!(move_to_key(numeric_pos('A'), numeric_pos('1'), true, Pos { x: -1, y: -1 }),
                   String::from("<<^"));
        assert_eq!(move_to_key(numeric_pos('A'), numeric_pos('1'), true, Pos { x: 0, y: 3 }),
                   String::from("^<<"));
        assert_eq!(move_to_key(numeric_pos('7'), numeric_pos('A'), false, Pos { x: 0, y: 3 }),
                   String::from(">>vvv"));
    }

    #[test]
    fn it_compute_sequence_to_numeric_key_pad() {
        assert_eq!(sequence_to_pad(numeric_pos, Pos { x: 0, y: 3 }, vec![String::from("029A")]),
                   vec!["<A^A>^^AvvvA", "<A^A^^>AvvvA"]);
    }

    #[test]
    fn it_compute_final_sequence() {
        assert_eq!((
                final_sequence("029A").len(),
                final_sequence("980A").len(),
                final_sequence("179A").len(),
                final_sequence("456A").len(),
                final_sequence("379A").len(),
                ),
                (
                    "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
                    "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len(),
                    "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len(),
                    "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len(),
                    "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
                )
        );
    }

    #[test]
    fn it_works() {
        assert_eq!(complexities(inputs::EXAMPLE), 126384);
        assert_eq!(complexities(inputs::INPUT), 112689);
    }
}
