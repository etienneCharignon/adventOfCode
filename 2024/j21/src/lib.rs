mod inputs;

use lazy_static::lazy_static;
use regex::Regex;

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

pub fn move_to_key(from: Pos, to:Pos) -> String {
    let mut sequence = String::new();

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
    sequence
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

pub fn sequence_to_pad(to_pos: fn(char) -> Pos, code: &str) -> String {
    let mut current: char = 'A';
    let mut s = String::new();
    for dest in code.chars() {
        s += &move_to_key(to_pos(current), to_pos(dest));
        s += "A";
        current = dest;
    }
    s
}

pub fn final_sequence(code: &str) -> String {
    sequence_to_pad(directionnal_pos,
        &sequence_to_pad(directionnal_pos,
            &sequence_to_pad(numeric_pos, code)))
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
    fn it_compute_sequence_to_numeric_key_pad() {
        assert_eq!(sequence_to_pad(numeric_pos, "029A"),"<A^A^^>AvvvA");
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
    }
}
