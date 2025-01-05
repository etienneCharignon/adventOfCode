mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;

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
        ' ' => Pos {x: 0, y: 3},
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
        ' ' => Pos {x: 0, y: 0},
        _ => panic!("Unknown position")
    }
}

pub fn move_to_key(from: Pos, to:Pos, h_first: bool, blank: Pos) -> String {
    // println!("{from:?}, {to:?}");
    let mut sequence = String::new();
    let mut start_with_h =  h_first;
    if from.y == blank.y && to.x == blank.x {
        start_with_h = false;
    }
    else if from.x == blank.x && to.y == blank.y {
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

pub fn append_char_sequences(sequences: HashSet<String>, char_sequences: HashSet<String>) -> HashSet<String> {
    sequences
        .iter()
        .flat_map(|sequence| char_sequences.iter().map(move |char_sequence| sequence.to_owned() + &char_sequence))
        .collect()
    // let mut new_sequences = HashSet::new();
    // for sequence in sequences {
    //     for char_sequence in &char_sequences {
    //         new_sequences.insert(format!("{sequence}{}", char_sequence));
    //     }
    // }
    // new_sequences
}

pub fn sequence_to_pad(to_pos: fn(char) -> Pos, blank: Pos, codes: HashSet<String>, robot: usize,
                       cache: &mut HashMap<(usize, String), HashSet<String>>) -> HashSet<String> {
    let mut all_sequences: HashSet<String> = HashSet::new();
    let mut current: char = 'A';
    for code in codes {
        let key = (robot, code.clone());
        if cache.contains_key(&key) {
            for sequence in cache.get(&key).unwrap() {
                all_sequences.insert(sequence.clone());
            }
            continue;
        }

        let mut sequences = HashSet::new();
        sequences.insert(String::new());
        for dest in code.chars() {
            let mut all_sequences_for_char = HashSet::new();
            for priority in [true, false] {
                let mut s = String::new();
                s += &move_to_key(to_pos(current), to_pos(dest), priority, blank);
                s.push('A');
                all_sequences_for_char.insert(s);
            }
            sequences = append_char_sequences(sequences, all_sequences_for_char);
            current = dest;
        }
        for sequence in sequences.clone() {
            all_sequences.insert(sequence);
        }
        cache.insert((robot, code), sequences);
    }
    all_sequences
}

pub fn unitary_move(from: char, to: char, to_pos: fn(char) -> Pos, blank: Pos) -> HashSet<String> {
    let mut sequences = HashSet::new();
    for priority in [true, false] {
        let mut s = String::new();
        s += &move_to_key(to_pos(from), to_pos(to), priority, blank);
        s += "A";
        sequences.insert(s);
    }
    sequences
}

pub fn final_sequence(code: &str) -> usize {
    let mut sequences = sequence_to_pad(numeric_pos, Pos {x: 0, y: 3}, to_set(vec![code]), 0, &mut HashMap::new());
    println!("nombre sequence : {}", sequences.len());
    let directional_blank = Pos {x: 0, y: 0};
    let mut cache = HashMap::new();
    for i in 0..2 {
        sequences = sequence_to_pad(directionnal_pos, directional_blank, sequences, i + 1, &mut cache);
        println!("nombre sequence : {}", sequences.len());
    }
    println!("finit");
    // println!("{all_sequences:?}");
    sequences.iter()
        .fold(String::from(""),
              |shortest, sequence|
              if shortest.len() > 0 && shortest.len() <= sequence.len() {
                  shortest
              } else {
                  sequence.clone()
              }).len()
}
 pub fn minimum_len(code: String, irobot:usize, nbr_robot: usize, cache: &mut HashMap<(usize, char, char), usize>) -> usize {
    let mut sequence_len = 0;
    let mut chars: Vec<char> = code.chars().collect();
    chars.insert(0, 'A');
    for i in 0..chars.len()-1 {
        sequence_len += minimum_for_a_char(chars[i], chars[i+1], irobot, nbr_robot, cache);
    }
    sequence_len
 }

pub fn minimum_for_a_char(from: char, to:char, irobot:usize, nbr_robot: usize, cache: &mut HashMap<(usize, char, char), usize>) -> usize {
    let key = (irobot, from, to);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut sequences = HashSet::new();
    if irobot == nbr_robot {
        for priority in [true, false] {
            let mut s = String::new();
            s += &move_to_key(numeric_pos(from), numeric_pos(to), priority, numeric_pos(' '));
            s.push('A');
            sequences.insert(minimum_len(s, irobot - 1, nbr_robot, cache));
        }
    }
    else {
        for priority in [true, false] {
            let mut s = String::new();
            s += &move_to_key(directionnal_pos(from), directionnal_pos(to), priority, directionnal_pos(' '));
            s.push('A');
            if irobot > 0 {
                sequences.insert(minimum_len(s, irobot - 1, nbr_robot, cache));
            }
            else {
                sequences.insert(s.len());
            }
        }
    }
    let min = *sequences.iter().min().unwrap();
    cache.insert(key, min);
    min
}

pub fn complexity(code: &str) -> usize {
    let sequence_len = final_sequence(code);
    println!("{code}: {}, {sequence_len}", sequence_len);

    sequence_len * RE.find(code).unwrap().as_str().parse::<usize>().unwrap()
}

pub fn complexity2(code: &str) -> usize {
    let mut cache = HashMap::new();
    let number_of_robot = 25;
    let sequence_len = minimum_len(code.to_string(), number_of_robot, number_of_robot, &mut cache);
    println!("{code}: {}, {sequence_len}", sequence_len);

    sequence_len * RE.find(code).unwrap().as_str().parse::<usize>().unwrap()
}

pub fn complexities(input: &str) -> usize {
    input.split("\n").map(|code| { complexity(code) }).sum()
}

pub fn complexities2(input: &str) -> usize {
    input.split("\n").map(|code| { complexity2(code) }).sum()
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

// A<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// A  v <<   A >>  ^ A   <   A > A  v  A  <   ^ AA > A   < v  AAA >  ^ A
// A         <       A       ^   A     >        ^^   A        vvv      A
// A                 0           2                   9                 A

pub fn to_set(vec: Vec<&str>) -> HashSet<String> {
    vec.into_iter().map(String::from).collect()
}


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
    fn it_compute_unitary_move() {
        assert_eq!(unitary_move('A', '0', numeric_pos, Pos { x: 0, y: 3 }), to_set(vec!["<A"]));
        assert_eq!(unitary_move('0', '2', numeric_pos, Pos { x: 0, y: 3 }), to_set(vec!["^A"]));
        assert_eq!(unitary_move('9', 'A', numeric_pos, Pos { x: 0, y: 3 }), to_set(vec!["vvvA"]));
        assert_eq!(unitary_move('2', '9', numeric_pos, Pos { x: 0, y: 3 }), to_set(vec![">^^A", "^^>A"]));
    }

    #[test]
    #[ignore]
    fn it_compute_sequence_to_numeric_key_pad() {
        assert_eq!(sequence_to_pad(numeric_pos, Pos { x: 0, y: 3 }, to_set(vec!["029A"]), 0, &mut HashMap::new()),
                   to_set(vec!["<A^A>^^AvvvA", "<A^A^^>AvvvA"]));
        let mut cache = HashMap::new();
        assert_eq!(sequence_to_pad(directionnal_pos, Pos {x: 0, y: 0},
                sequence_to_pad(numeric_pos, Pos { x: 0, y: 3 }, to_set(vec!["029A"]), 0, &mut cache), 1, &mut cache),
                   to_set(vec![
                       "v<<A>>^A<A>A<AAv>A^Av<AAA^>A",
                       "v<<A>>^A<A>AvA<^AA>Av<AAA^>A",
                       "v<<A>>^A<A>A<AAv>A^A<vAAA>^A",
                       "v<<A>>^A<A>A<AA>vA^Av<AAA>^A",
                       "v<<A>>^A<A>AvA^<AA>A<vAAA>^A",
                       "v<<A>>^A<A>AvA^<AA>A<vAAA^>A",
                       "v<<A>>^A<A>A<AA>vA^A<vAAA>^A",
                       "v<<A>>^A<A>A<AA>vA^A<vAAA^>A",
                       "v<<A>>^A<A>AvA^<AA>Av<AAA>^A",
                       "v<<A>>^A<A>A<AAv>A^A<vAAA^>A",
                       "v<<A>>^A<A>AvA<^AA>A<vAAA^>A",
                       "v<<A>>^A<A>AvA<^AA>A<vAAA>^A",
                       "v<<A>>^A<A>AvA<^AA>Av<AAA>^A",
                       "v<<A>>^A<A>AvA^<AA>Av<AAA^>A",
                       "v<<A>>^A<A>A<AA>vA^Av<AAA^>A",
                       "v<<A>>^A<A>A<AAv>A^Av<AAA>^A"]));
    }

    #[test]
    #[ignore]
    fn it_compute_final_sequence() {
        assert_eq!((
                final_sequence("029A"),
                final_sequence("980A"),
                final_sequence("179A"),
                final_sequence("456A"),
                final_sequence("379A"),
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
    #[ignore]
    fn it_works_p1() {
        assert_eq!(complexities(inputs::EXAMPLE), 126384);
        assert_eq!(complexities(inputs::INPUT), 107934);
    }

    #[test]
    fn it_works_p2() {
        assert_eq!(complexities2(inputs::INPUT), 130470079151124);
    }
}
