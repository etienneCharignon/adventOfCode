mod inputs;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}

pub fn add(a: Pos, b: &Pos) -> Pos {
    Pos {x: a.x + b.x, y: a.y + b.y }
}

lazy_static! {
    static ref DIR: [Pos;4] = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
}

pub fn read(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let mut maze: Vec<Vec<char>> = vec![];
    let mut start: Pos = Pos { x: 0, y: 0};
    let mut end: Pos = Pos { x: 0, y: 0};
    for (r, line) in input.split('\n').enumerate() {
        let mut row: Vec<char> = vec![];
        for (c, char) in line.chars().enumerate() {
            if char == 'S' {
                start = Pos { x: c as i32, y: r as i32 };
                row.push('.');
            }
            else if char == 'E' {
                end = Pos { x: c as i32, y: r as i32 };
                row.push('.');
            }
            else {
                row.push(char);
            }
        }
        maze.push(row);
    }
    (maze, start, end)
}

pub fn cell(p: Pos, maze: &Vec<Vec<char>>) -> char {
    maze[p.y as usize][p.x as usize]
}

pub fn find_next(current: Pos, maze: &Vec<Vec<char>>, path: &Vec<Pos>) -> Option<Pos> {
    for d in DIR.iter() {
        let next = add(current, d);
        if cell(next, maze) == '.' && !path.contains(&next) {
            return Some(next);
        }
    }
    return None
}

pub fn read_path(input: &str) -> (Vec<Pos>, Vec<Vec<char>>) {
    let (maze, start, end) = read(input);
    let mut current = start;
    let mut path = vec![start];
    while current != end {
        if let Some(next) = find_next(current, &maze, &path) {
            path.push(next);
            current = next;
        }
        else {
            panic!("on devrait toujours trouver un next")
        }
    }
    // print(&maze, &path);
    (path, maze)
}

pub fn print(maze: &Vec<Vec<char>>, visited: &Vec<Pos>) {
    let mut maze_visisted = maze.clone();
    for p in visited {
        maze_visisted[p.y as usize][p.x as usize] = '-';
    }
    let mut output = String::new();
    for line in maze_visisted {
        let str: String = line.iter().collect();
        output.push_str(&str);
        output.push_str("\n");
    }
    println!("{}", output);
}

pub fn manathan_distance(p1: &Pos, p2: &Pos) -> usize {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as usize
}

pub fn find_shortcut(pos: &Pos, path: &[Pos], min_save: i32) -> Vec<usize> {
    let mut shortcuts = vec![];
    for (i, p) in path.iter().enumerate() {
        let d = manathan_distance(p, pos);
        if d > 1 && d <= 20 {
            let save = i as i32 - d as i32 + 1;
            if save >= min_save {
                // println!("S: {pos:?}, E: {p:?}, {d} => {}", save);
                shortcuts.push(save as usize);
            }
        }
    }
    shortcuts
}

pub fn count_cheats(path_maze: (Vec<Pos>, Vec<Vec<char>>), min_save: usize) -> usize {
    let (path, _maze) = path_maze;
    let mut saves: Vec<usize> = vec![];
    for (i, pos) in path.iter().enumerate() {
        let distances = find_shortcut(pos, &path[(i + 1)..], min_save as i32);
        saves.extend(distances);
    }
    // println!("{saves:?}");
    saves.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_path() {
        assert_eq!(read_path(inputs::EXAMPLE).0.len() - 1, 84);
    }

    #[test]
    fn it_compute_manathan_distance() {
        assert_eq!(manathan_distance(&Pos {x: 0, y: 0}, &Pos{x: 1, y:0}), 1);
        assert_eq!(manathan_distance(&Pos {x: 1, y: 0}, &Pos{x: 0, y:0}), 1);
        assert_eq!(manathan_distance(&Pos {x: 0, y: 0}, &Pos{x: 5, y:0}), 5);
        assert_eq!(manathan_distance(&Pos {x: 0, y: 1}, &Pos{x: 0, y:0}), 1);
        assert_eq!(manathan_distance(&Pos {x: 1, y: 1}, &Pos{x: 0, y:0}), 2);
    }

    #[test]
    fn it_works() {
        assert_eq!(count_cheats(read_path(inputs::EXAMPLE), 76), 3);
        assert_eq!(count_cheats(read_path(inputs::INPUT), 100), 1502);
    }
}
