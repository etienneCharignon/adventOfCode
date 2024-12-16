mod inputs;

use lazy_static::lazy_static;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Reindeer {
   p: Pos,
   d: Pos,
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Cell {
   r: Reindeer,
   c: usize
}

pub fn add(a: Pos, b: Pos) -> Pos {
    Pos {x: a.x + b.x, y: a.y + b.y }
}

pub fn turn_right(direction: Pos) -> Pos {
    Pos { x: -direction.y, y: direction.x }
}

pub fn turn_left(direction: Pos) -> Pos {
    Pos { x: direction.y, y: -direction.x }
}

pub fn cell(p: Pos, maze: &Vec<Vec<char>>) -> char {
    maze[p.y as usize][p.x as usize]
}

lazy_static! {
    static ref DIR: [Pos;4] = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
}

pub fn read(input: &str) -> Vec<Vec<char>> {
    input.split('\n').map(|line| line.chars().collect()).collect()
}

pub fn print(maze: &Vec<Vec<char>>, visited: &HashSet<Reindeer>) {
    let mut maze_visisted = maze.clone();
    for r in visited {
        maze_visisted[r.p.y as usize][r.p.x as usize] = 'X';
    }
    let mut output = String::new();
    for line in maze_visisted {
        let str: String = line.iter().collect();
        output.push_str(&str);
        output.push_str("\n");
    }
    println!("{}", output);
}

pub fn find_shortest(c: &Cell, open: &Vec<Cell>) -> bool {
    for cell in open {
        if cell.r == c.r && cell.c < c.c {
            return true;
        }
    }
    false
}

pub fn shortest_path(o: Cell, maze: &Vec<Vec<char>>, visited: &mut HashSet<Reindeer>) -> usize {
    let mut open: Vec<Cell> = vec![];
    open.push(o);
    while !open.is_empty() {
        open.sort_by(|cell1, cell2| cell2.c.cmp(&cell1.c));
        let current = open.pop().unwrap();
        visited.insert(current.r);
        // print(maze, visited);

        if cell(current.r.p, maze) == 'E' {
            return current.c;
        }

        let next = add(current.r.p, current.r.d);
        if cell(next, maze) == '.' || cell(next, maze) == 'E' {
            let new_r = Reindeer { p: next, d: current.r.d };
            let new_cell = Cell { r: new_r, c: current.c + 1 };
            if ! (visited.contains(&new_r) || find_shortest(&new_cell, &open)) {
                open.push(new_cell);
            }
        }
        for d in [turn_left(current.r.d),turn_right(current.r.d)] {
            let new_r = Reindeer { p: current.r.p, d };
            let new_cell = Cell { r: new_r, c: current.c + 1000 };
            if ! (visited.contains(&new_r) || find_shortest(&new_cell, &open)) {
                open.push(new_cell);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_turn_right() {
        assert_eq!(turn_right(DIR[3]), DIR[0]);
    }

    #[test]
    fn it_works() {
        let mut maze = read(inputs::EXAMPLE);
        let mut height = maze.len() as i32;
        assert_eq!(shortest_path(Cell{ r: Reindeer { p: Pos{x:1, y:height - 2}, d: DIR[0] }, c: 0 }, &maze, &mut HashSet::new()), 7036);
        maze = read(inputs::INPUT);
        height = maze.len() as i32;
        assert_eq!(shortest_path(Cell{ r: Reindeer { p: Pos{x:1, y:height - 2}, d: DIR[0] }, c: 0 }, &maze, &mut HashSet::new()), 94436);
    }
}
