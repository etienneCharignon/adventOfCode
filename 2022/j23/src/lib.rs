mod inputs;
use std::collections::HashSet;

#[allow(unused_imports)]
use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Elf(i32, i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Dir(i32, i32);

#[allow(dead_code)]
const DIRECTIONS: [Dir; 4] = [
        Dir(0, -1),
        Dir(1, 0),
        Dir(0, 1),
        Dir(-1, 0),
    ];

#[allow(dead_code)]
fn read_input(input: &str) -> HashSet<Elf> {
    let mut result = HashSet::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' { result.insert(Elf(x as i32, y as i32)); }
        }
    }
    result
}

#[allow(dead_code)]
fn add(e: Elf, d: Dir) -> Elf {
    Elf(e.0+d.0, e.1+d.1)
}

fn intention(elf: &Elf, map: &HashSet<Elf>, offset: usize) -> Elf {
    let sequence_directions = [0, 2, 3, 1];
    for d in 0..4 {
        let i = sequence_directions[(d + offset) % 4];
        let intention = add(*elf, DIRECTIONS[i]);
        let right_angle = (i + 1) % 4;
        let left_angle = (i + 3) % 4;
        // print!("{}", print_map(map));
        // println!("map : {:?}", map);
        // println!("{:?}, {:?}, {:?}",
        //          intention,
        //          add(intention, directions[left_angle]),
        //          add(intention, directions[right_angle]));
        if !map.contains(&intention) 
        && !map.contains(&add(intention, DIRECTIONS[left_angle]))
        && !map.contains(&add(intention, DIRECTIONS[right_angle]))
        {
            if d == 0
            && !map.contains(&add(*elf, DIRECTIONS[(i+1)%4]))
            && !map.contains(&add(add(*elf, DIRECTIONS[(i+1)%4]), DIRECTIONS[(i+2)%4]))
            && !map.contains(&add(*elf, DIRECTIONS[(i+2)%4]))
            && !map.contains(&add(add(*elf, DIRECTIONS[(i+2)%4]), DIRECTIONS[(i+3)%4]))
            && !map.contains(&add(*elf, DIRECTIONS[(i+3)%4]))
            && !map.contains(&add(add(*elf, DIRECTIONS[(i+3)%4]), DIRECTIONS[(i+4)%4]))
            {
                return *elf;
            }
            return intention;
        }
    }
    *elf
}

fn compare(a: &HashSet<Elf>, b: &HashSet<Elf>) -> bool {
    let mut a_vec = Vec::from_iter(a);
    let mut b_vec = Vec::from_iter(b);
    a_vec.sort();
    b_vec.sort();
    a_vec.iter().enumerate().all(|(i, elf)| *elf == b_vec[i])
}

#[allow(dead_code)]
fn run_round(n: usize, map: HashSet<Elf>) -> usize {
    let mut new_map: HashSet<Elf> = HashSet::new();
    let mut from_map = map;
    for i in 0..n {
        new_map = run_a_round(i, &from_map);
        if compare(&new_map, &from_map) {
            return i + 1;
        }
        from_map = new_map.clone();
    }
    n
}

#[allow(dead_code)]
fn run_a_round(offset: usize, from_map: &HashSet<Elf>) -> HashSet<Elf> {
    let elfs = Vec::from_iter(from_map);
    let mut conflicts = HashSet::new();
    let mut intentions = Vec::new();
    for elf in elfs.clone() {
        let intention = intention(elf, from_map, offset);
        // print!("{}", print_map(from_map));
        // println!("elf : {:?}, intention : {:?}", *elf, intention);
        if intentions.contains(&intention) {
            conflicts.insert(intention);
        }
        intentions.push(intention);
    }
    let mut new_map: HashSet<Elf> = HashSet::new();
    for (i, intention) in intentions.iter().enumerate() {
        if conflicts.contains(intention) {
            new_map.insert(*elfs[i]);
        }
        else {
            new_map.insert(*intention);
        }
    }

    // println!("{}", print_map(&new_map));
    new_map
}

#[allow(dead_code)]
fn count_empty(map: &HashSet<Elf>) -> usize {
    print_map(map).matches(".").count()
}

fn print_map(map: &HashSet<Elf>) -> String {
    let mut print = String::new();
    let mut minx = i32::MAX;
    let mut miny = i32::MAX;
    let mut maxx = i32::MIN;
    let mut maxy = i32::MIN;
    for elf in map {
        minx = cmp::min(minx, elf.0);
        miny = cmp::min(miny, elf.1);
        maxx = cmp::max(maxx, elf.0);
        maxy = cmp::max(maxy, elf.1);
    }
    for y in miny..=maxy {
        for x in minx..=maxx {
            print += if map.contains(&Elf(x,y)) { "#" } else { "." }
        }
        print += "\n";
    }
    print
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_inputs() {
        assert_eq!(read_input(inputs::EXAMPLE), HashSet::from_iter(vec![
            Elf(4, 0), 
            Elf(3, 1), Elf(6, 1), Elf(2, 1), Elf(4, 1), 
            Elf(0, 2), Elf(4, 2), Elf(6, 2), 
            Elf(1, 3), Elf(5, 3), Elf(6, 3),
            Elf(0, 4), Elf(2, 4), Elf(3, 4), Elf(4, 4),
            Elf(0, 5), Elf(1, 5), Elf(3, 5), Elf(5, 5), Elf(6, 5),
            Elf(1, 6), Elf(4, 6),
        ]));
    }

    #[test]
    fn it_compute_intention() {
        assert_eq!(intention(&Elf(2,2), &HashSet::from([Elf(2, 1), Elf(3,1), Elf(2,2)]), 0), Elf(2,3));
        assert_eq!(intention(&Elf(0,0), &HashSet::from([Elf(0,1)]), 0), Elf(0,-1));
        assert_eq!(intention(&Elf(1,1), &HashSet::from([Elf(2,1)]), 0), Elf(1,0));
        assert_eq!(intention(&Elf(0,0), &HashSet::from([Elf(0,-1)]), 0), Elf(0,1));
        assert_eq!(intention(&Elf(0,0), &HashSet::from([Elf(0,-1), Elf(0,1)]), 0), Elf(-1,0));
        assert_eq!(intention(&Elf(0,0), &HashSet::from([Elf(0,-1), Elf(0,1), Elf(-1, 0), Elf(1,0)]), 0), Elf(0,0));
        assert_eq!(intention(&Elf(0,0), &HashSet::from([Elf(-1,-1)]), 0), Elf(0,1));
        assert_eq!(intention(&Elf(0,0), &HashSet::from([Elf(1,-1)]), 0), Elf(0,1));
        assert_eq!(intention(&Elf(2,4), &HashSet::from([Elf(2,2), Elf(3,4)]), 0), Elf(2,3));
    }

    #[test]
    fn it_run_round() {
        //assert_eq!(run_round(1, &HashSet::from([Elf(0,0)])), HashSet::from([Elf(0,-1)]));
        assert_eq!(run_a_round(0, &read_input(".....\n..##.")), read_input("..##."));
        assert_eq!(run_a_round(0, &read_input(".....
..##.
..#..
.....
..##.
.....")), read_input("..##.
.....
..#..
...#.
..#..
....."));
        assert_eq!(run_round(3, read_input(".....
..##.
..#..
.....
..##.
.....")), 3);
    }

    #[test]
    fn it_score() {
        assert_eq!(run_round(21, read_input(inputs::EXAMPLE)), 20);
        assert_eq!(run_round(1000, read_input(inputs::INPUT)), 923);
    }
}
