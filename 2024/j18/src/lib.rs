mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
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
        if count == time {
            break;
        }
    }
    println!("memory size: {}", memory.len());
    memory
}

pub fn h(pos: &Pos, size: i32) -> i32 {
    (size - pos.x) + (size - pos.y)
}

pub fn print(memory: &HashSet<Pos>, size: i32, visited: &HashSet<Pos>) {
    let mut output = String::new();
    for r in 0..size {
        for c in 0..size {
            let current = Pos { x: c, y: r };
            if memory.contains(&current) {
                output.push_str("#");
            }
            else if visited.contains(&current) {
                output.push_str("O");
            }
            else {
                output.push_str(".");
            }
        }
        output.push_str("\n");
    }
    println!("{}", output);
}

pub fn find_path(memory: &HashSet<Pos>, size: i32, origin: Pos, visited: &mut HashSet<Pos>) -> i32 {
    let mut open: Vec<Pos> = vec![];
    open.push(origin);
    visited.insert(origin);
    let mut iteration = 1;
    let mut next_generation: Vec<Pos> = vec![];
    while !open.is_empty() {
        let o = open.remove(0);
        println!("\norigin : {o:?}");
        // print(memory, size, visited);
        for d in DIR.iter() {
            let next = add(o, *d);
            println!("next : {next:?}");
            if next.x < 0 || next.y < 0 || next.x >= size || next.y >= size {
                continue;
            }
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            if memory.contains(&next) {
                continue;
            }

            if next.x == size - 1 && next.y == size - 1 {
                return iteration;
            }

            next_generation.push(next);
        }
        if open.is_empty() {
            iteration += 1;
            open.extend(next_generation);
            next_generation = vec![];
        }
    }
    0
}

pub fn find_path_size(memory: HashSet<Pos>, size: i32) -> i32 {
    let origin = Pos { x: 0, y:0 };
    let mut visited: HashSet<Pos> = HashSet::new();

    print(&memory, size, &visited);
    find_path(&memory, size, origin, &mut visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(find_path_size(read(inputs::EXAMPLE, 12), 7), 22);
        assert_eq!(find_path_size(read(inputs::INPUT, 1024), 71), 260);
    }
}
