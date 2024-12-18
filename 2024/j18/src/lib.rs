mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

lazy_static! {
    static ref RE: Regex = Regex::new(r"([-0-9]+)").unwrap();
    static ref DIR: [Pos;4] = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}

pub fn add(a: Pos, b: Pos) -> Pos {
    Pos {x: a.x + b.x, y: a.y + b.y }
}

pub fn read(input: &str, time: usize) -> HashSet<Pos> {
    let mut memory = HashSet::new();
    let mut matches = RE.find_iter(input)
        .map(|m| m.as_str().parse::<i32>().unwrap());

    let mut count = 0;
    while let (Some(px), Some(py)) = (
        matches.next(),
        matches.next()
    ) {
        memory.insert(Pos{ x: px, y: py });
        count += 1;
        if count > time {
            break;
        }
    }
    println!("memory size: {}", memory.len());
    memory
}

pub fn h(pos: &Pos, size: i32) -> i32 {
    (size - pos.x) + (size - pos.y)
}

// pub fn shortest_path(o: Cell, maze: &Vec<Vec<char>>, visited: &mut HashSet<Reindeer>, _shortest: usize) -> usize {
pub fn find_path_size(memory: HashSet<Pos>, size: i32) -> i32 {
    let origin = Pos { x: 0, y:0 };
    let mut open: Vec<Pos> = vec![];
    open.push(origin);
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut g_score: HashMap<Pos, i32> = HashMap::new();
    g_score.insert(origin, 0);
    let mut f_score: HashMap<Pos, i32> = HashMap::new();
    f_score.insert(origin, h(&origin, size));

    while !open.is_empty() {
        open.sort_by(|cell1, cell2| {
            let f_score1 = f_score.get(cell1).unwrap_or(&i32::MAX);
            let f_score2 = f_score.get(cell2).unwrap_or(&i32::MAX);
            f_score2.cmp(&f_score1)
        });
        let current = open.pop().unwrap();

        if current.x == size - 1 && current.y == size - 1  {
            return 0; // reconstruct_path(cameFrom, current)
        }

        for d in DIR.iter() {
            let next = add(current, *d);
            if memory.contains(&next) {
                continue;
            }
            let tentative_g_score = g_score.get(&current).unwrap_or(&(i32::MAX - 1)) + 1;
            if tentative_g_score < *g_score.get(&next).unwrap_or(&i32::MAX) {
                came_from.insert(next, current);
                g_score.insert(next, tentative_g_score);
                f_score.insert(next, tentative_g_score + h(&next, size));
                if ! open.contains(&next) {
                    open.push(next);
                }
            }
        }
        println!("{:?}", open);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(find_path_size(read(inputs::EXAMPLE, 12), 6), 22);
    }
}
