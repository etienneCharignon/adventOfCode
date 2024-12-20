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

pub fn print_spots(maze: &Vec<Vec<char>>, spots: &HashSet<Pos>) {
    let mut maze_visisted = maze.clone();
    for r in spots {
        maze_visisted[r.y as usize][r.x as usize] = 'X';
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

pub fn add_spot(c: &Cell, came_from: &MultiMap<Reindeer, Cell>, spots: &mut HashSet<Pos>, maze: &Vec<Vec<char>>) {
    spots.insert(c.r.p);
    if let Some(values) = came_from.get_vec(&c.r) {
        for from in values {
            // print_spots(maze, spots);
            add_spot(&from, came_from, spots, maze);
        }
    }
    else {
        // println!("Arrivé : {c:?}");
    }
    // println!("{spots:?}");
}

pub fn add_to_came_from(came_from: &mut MultiMap<Reindeer, Cell>, to: Cell, from: Cell) {
    if let Some(value) = came_from.get(&to.r) {
        if value.c > to.c {
            came_from.remove(&to.r);
        }
    }
    let from_with_to_cost = Cell { r: from.r, c: to.c };
    if let Some(values) = came_from.get_vec(&to.r) {
        if values.contains(&from_with_to_cost) {
            return;
        }
    }
    came_from.insert(to.r, from_with_to_cost);
}

pub fn shortest_path(o: Cell, maze: &Vec<Vec<char>>, visited: &mut HashSet<Reindeer>, _shortest: usize) -> usize {
    let mut open: Vec<Cell> = vec![];
    let mut came_from: MultiMap<Reindeer, Cell> = MultiMap::new();
    let mut spots: HashSet<Pos> = HashSet::new();
    open.push(o);
    while !open.is_empty() {
        open.sort_by(|cell1, cell2| cell2.c.cmp(&cell1.c));
        let current = open.pop().unwrap();
        visited.insert(current.r);
        // print(maze, visited);

        if cell(current.r.p, maze) == 'E'  {
            println!("{:?}", came_from.keys().len());
            add_spot(&current, &came_from, &mut spots, maze);
            break;
        }

        let next = add(current.r.p, current.r.d);
        if cell(next, maze) == '.' || cell(next, maze) == 'E' {
            let new_r = Reindeer { p: next, d: current.r.d };
            let new_cell = Cell { r: new_r, c: current.c + 1 };
            if ! (visited.contains(&new_r)) {
                // println!("inserting {:?} -> {:?} with cost {}", new_r, current.r, new_cell.c);
                add_to_came_from(&mut came_from, new_cell, current);
                open.push(new_cell);
            }
        }
        for d in [turn_left(current.r.d),turn_right(current.r.d)] {
            let new_r = Reindeer { p: current.r.p, d };
            let new_cell = Cell { r: new_r, c: current.c + 1000 };
            if ! (visited.contains(&new_r)) {
                // println!("inserting {:?} -> {:?}", current, new_r.p);
                add_to_came_from(&mut came_from, new_cell, current);
                open.push(new_cell);
            }
        }
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
    fn it_works_example2() {
        let maze = read(inputs::EXAMPLE2);
        let height = maze.len() as i32;
        assert_eq!(shortest_path(Cell{ r: Reindeer { p: Pos{x:1, y:height - 2}, d: DIR[0] }, c: 0 }, &maze, &mut HashSet::new(), 11048), 64);
    }

    #[test]
    fn it_works() {
        let maze = read(inputs::INPUT);
        let height = maze.len() as i32;
        assert_eq!(shortest_path(Cell{ r: Reindeer { p: Pos{x:1, y:height - 2}, d: DIR[0] }, c: 0 }, &maze, &mut HashSet::new(), 94436), 481); // 573 is to hight, 449
    }
}
