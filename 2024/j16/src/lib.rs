mod inputs;

use lazy_static::lazy_static;
use std::collections::HashSet;
use multimap::MultiMap;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Reindeer {
   p: Pos,
   d: Pos
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

pub fn add_spot(c: &Cell, came_from: &MultiMap<Cell, Cell>, spots: &mut HashSet<Cell>) {
    if spots.contains(c) {
        return;
    }
    spots.insert(*c);
    if let Some(values) = came_from.get_vec(&c) {
        let filtered: Vec<&Cell> = values.iter().filter(|&v| {
            if v.r.p != c.r.p { v.c+1 == c.c }
            else { v.c + 1000 == c.c }
        }).collect();
        for child in filtered {
            add_spot(child, came_from, spots);
        }
    }
}

pub fn shortest_path(o: Cell, maze: &Vec<Vec<char>>, visited: &mut HashSet<Reindeer>, _shortest: usize) -> usize {
    let mut open: Vec<Cell> = vec![];
    let mut came_from: MultiMap<Cell, Cell> = MultiMap::new();
    let mut spots_cells: HashSet<Cell> = HashSet::new();
    open.push(o);
    while !open.is_empty() {
        open.sort_by(|cell1, cell2| cell2.c.cmp(&cell1.c));
        let current = open.pop().unwrap();
        visited.insert(current.r);
        // print(maze, visited);

        if cell(current.r.p, maze) == 'E'  {
            // println!("{:?}", came_from.get_vec(&current.r.p));
            add_spot(&current, &came_from, &mut spots_cells);
            continue;
        }

        let next = add(current.r.p, current.r.d);
        if cell(next, maze) == '.' || cell(next, maze) == 'E' {
            let new_r = Reindeer { p: next, d: current.r.d };
            let new_cell = Cell { r: new_r, c: current.c + 1 };
            if ! (visited.contains(&new_r)) {
                println!("inserting {:?} -> {:?}", current, new_r.p);
                came_from.insert(new_cell, current);
                open.push(new_cell);
            }
        }
        for d in [turn_left(current.r.d),turn_right(current.r.d)] {
            let new_r = Reindeer { p: current.r.p, d };
            let new_cell = Cell { r: new_r, c: current.c + 1000 };
            if ! (visited.contains(&new_r)) {
//                 println!("inserting {:?} -> {:?}", current, new_r.p);
                came_from.insert(new_cell, current);
                open.push(new_cell);
            }
        }
    }
    println!("{:?}", came_from);
    let mut spots: HashSet<Pos> = HashSet::new();
    for cell in spots_cells {
        spots.insert(cell.r.p);
    }
    spots.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_turn_right() {
        assert_eq!(turn_right(DIR[3]), DIR[0]);
    }

    #[test]
    fn it_works_example() {
        let maze = read(inputs::EXAMPLE);
        let height = maze.len() as i32;
        assert_eq!(shortest_path(Cell{ r: Reindeer { p: Pos{x:1, y:height - 2}, d: DIR[0] }, c: 0 }, &maze, &mut HashSet::new(), 7036), 45);
    }

    #[test]
    #[ignore]
    fn it_works_example2() {
        let maze = read(inputs::EXAMPLE2);
        let height = maze.len() as i32;
        assert_eq!(shortest_path(Cell{ r: Reindeer { p: Pos{x:1, y:height - 2}, d: DIR[0] }, c: 0 }, &maze, &mut HashSet::new(), 11048), 64);
    }

    #[test]
    #[ignore]
    fn it_works() {
        let maze = read(inputs::INPUT);
        let height = maze.len() as i32;
        assert_eq!(shortest_path(Cell{ r: Reindeer { p: Pos{x:1, y:height - 2}, d: DIR[0] }, c: 0 }, &maze, &mut HashSet::new(), 94436), 572); // 573 is to hight, 449
    }
}
