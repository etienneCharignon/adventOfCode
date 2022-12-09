mod inputs;

use assert_approx_eq::assert_approx_eq;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Direction(i32, i32);
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Pos(i32, i32);

#[derive(Debug, PartialEq)]
struct Move {
    dir: Direction,
    dist: i32
}

#[allow(dead_code)]
fn read_input(input: &str) -> Vec<Move> {
    input.lines().map(|cmd| read_cmd(cmd)).collect::<Vec<Move>>()
}

#[allow(dead_code)]
fn read_cmd(cmd: &str) -> Move {
    let parts: Vec<&str> = cmd.split(' ').collect();
    let dir = {
        match parts[0] {
            "R" => Direction(1, 0),
            "U" => Direction(0, 1),
            "L" => Direction(-1, 0),
            "D" => Direction(0, -1),
            _ => panic!("not a direction")
        }
    };
    Move {
        dir: dir,
        dist: parts[1].parse::<i32>().unwrap()
    }
}
#[allow(dead_code)]
fn add(pos: &Pos, dir: &Direction) -> Pos {
    Pos(pos.0 + dir.0, pos.1 + dir.1)
}

#[allow(dead_code)]
fn compute_distance(head: &Pos, tail: &Pos) -> Direction {
    let mut dx = head.0 - tail.0;
    let mut dy = head.1 - tail.1;
    let distance = Direction(dx, dy);
    if length(distance) < 2_f32 {
        return Direction(0, 0);
    }
    if dx > 1 { dx -= 1; } 
    if dx < -1 { dx += 1; } 
    if dy > 1 { dy -= 1; } 
    if dy < -1 { dy += 1; } 
    return Direction(dx, dy)
}

#[allow(dead_code)]
fn length(vector: Direction) -> f32 {
    ((vector.0 * vector.0 + vector.1 * vector.1) as f32).sqrt()
}

#[allow(dead_code)]
fn run_cmd(cmd: Move, head_pos: &mut Vec<Pos>, tail_pos: &mut Vec<Pos>) {
    let direction = &cmd.dir;
    let tail = *tail_pos.last().unwrap();
    for i in 0..cmd.dist {
        &head_pos.push(add(head_pos.last().unwrap(), direction));
        let head = head_pos.last().unwrap();
        let tail_move = compute_distance(head, &tail);
        if length(tail_move) > 0_f32 {
            &tail_pos.push(add(&tail, &tail_move));
        }
    }
}

#[allow(dead_code)]
fn moves(cmds: Vec<Move>) -> (Vec<Pos>, Vec<Pos>) {
    let mut head_pos = vec![Pos(0,0)];
    let mut tail_pos = vec![Pos(0,0)];
    for cmd in cmds {
        run_cmd(cmd, &mut head_pos, &mut tail_pos);
    }
    (head_pos, tail_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_input() {
        assert_eq!(read_input(inputs::EXAMPLE), vec![
            Move { dir: Direction(1, 0), dist: 4 },
            Move { dir: Direction(0, 1), dist: 4 },
            Move { dir: Direction(-1, 0), dist: 3 },
            Move { dir: Direction(0, -1), dist: 1 },
            Move { dir: Direction(1, 0), dist: 4 },
            Move { dir: Direction(0, -1), dist: 1 },
            Move { dir: Direction(-1, 0), dist: 5 },
            Move { dir: Direction(1, 0), dist: 2 }
        ]);
    }

    #[test]
    fn it_compute_distance() {
        assert_eq!(compute_distance(&Pos(1,1), &Pos(0,0)), Direction(0,0));
        assert_eq!(compute_distance(&Pos(-1,-1), &Pos(0,0)), Direction(0,0));
    }

    #[test]
    fn it_compute_length() {
        assert_eq!(length(Direction(1,0)), 1.0_f32);
        assert_eq!(length(Direction(2,0)), 2.0_f32);
        assert_approx_eq!(length(Direction(1,2)), 2.23_f32, 0.01f32);
    }

    #[test]
    fn it_moves() {
        assert_eq!(moves(read_input(inputs::EXAMPLE)).0, vec![
            Pos(0, 0),
            Pos(1, 0), Pos(2, 0), Pos(3, 0), Pos(4, 0),
            Pos(4, 1), Pos(4, 2), Pos(4, 3), Pos(4, 4),
            Pos(3, 4), Pos(2, 4), Pos(1, 4),
            Pos(1, 3),
            Pos(2, 3), Pos(3, 3), Pos(4, 3), Pos(5, 3),
            Pos(5, 2),
            Pos(4, 2), Pos(3, 2), Pos(2, 2), Pos(1, 2), Pos(0, 2),
            Pos(1, 2), Pos(2, 2)
        ]);
        assert_eq!(moves(read_input(inputs::EXAMPLE)).1, vec![
            Pos(0, 0),
            Pos(1, 0), Pos(2, 0), Pos(3, 0),
            Pos(4, 1), Pos(4, 2), Pos(4, 3),

            Pos(3, 4), Pos(2, 4),
            Pos(3, 3), Pos(4, 3),
            Pos(3, 2), Pos(2, 2), Pos(1, 2)
        ]);
    }

    #[test]
    fn it_solve() { 
        let tail_pos = moves(read_input(inputs::INPUT)).1;

        assert_eq!(HashSet::<Pos>::from_iter::<Vec<Pos>>(tail_pos).len(), 13);
    }
}
