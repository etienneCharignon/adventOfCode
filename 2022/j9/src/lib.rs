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

#[derive(Debug, PartialEq, Copy, Clone)]
struct Knot {
    positions: &mut Vec<Pos>
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
fn compute_move(head: &Pos, tail: &Pos) -> Direction {
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

fn move_knot(head: &Pos, tail: &Pos, knot_pos: &mut Vec<Pos>){
    let kmove = compute_move(head, tail);
    if length(kmove) > 0_f32 {
        &knot_pos.push(add(tail, &kmove));
    }
}

#[allow(dead_code)]
fn xrun_cmd(cmd: Move, head_pos: &mut Vec<Pos>,
    knot1_pos: &mut Vec<Pos>,
    knot2_pos: &mut Vec<Pos>,
    knot3_pos: &mut Vec<Pos>,
    knot4_pos: &mut Vec<Pos>,
    knot5_pos: &mut Vec<Pos>,
    knot6_pos: &mut Vec<Pos>,
    knot7_pos: &mut Vec<Pos>,
    knot8_pos: &mut Vec<Pos>,
    knot9_pos: &mut Vec<Pos>)
{
    let direction = &cmd.dir;
    for _i in 0..cmd.dist {
        let new_head = add(head_pos.last().unwrap(), direction);
        &head_pos.push(new_head);
        let knot1 = *knot1_pos.last().unwrap();
        move_knot(&new_head, &knot1, knot1_pos);
        let new_knot1 = *knot1_pos.last().unwrap();
        let knot2 = *knot2_pos.last().unwrap();
        move_knot(&new_knot1, &knot2, knot2_pos);
        let new_knot2 = *knot2_pos.last().unwrap();
        let knot3 = *knot3_pos.last().unwrap();
        move_knot(&new_knot2, &knot3, knot3_pos);
        let new_knot3 = *knot3_pos.last().unwrap();
        let knot4 = *knot4_pos.last().unwrap();
        move_knot(&new_knot3, &knot4, knot4_pos);
        let new_knot4 = *knot4_pos.last().unwrap();
        let knot5 = *knot5_pos.last().unwrap();
        move_knot(&new_knot4, &knot5, knot5_pos);
        let new_knot5 = *knot5_pos.last().unwrap();
        let knot6 = *knot6_pos.last().unwrap();
        move_knot(&new_knot5, &knot6, knot6_pos);
        let new_knot6 = *knot6_pos.last().unwrap();
        let knot7 = *knot7_pos.last().unwrap();
        move_knot(&new_knot6, &knot7, knot7_pos);
        let new_knot7 = *knot7_pos.last().unwrap();
        let knot8 = *knot8_pos.last().unwrap();
        move_knot(&new_knot7, &knot8, knot8_pos);
        let new_knot8 = *knot8_pos.last().unwrap();
        let knot9 = *knot9_pos.last().unwrap();
        move_knot(&new_knot8, &knot9, knot9_pos);
    }
}

#[allow(dead_code)]
fn run_cmd(cmd: Move, rope: &Vec<Knot>) {
    let direction = &cmd.dir;
    let  head = rope[0];
    for _i in 0..cmd.dist {
        let new_head = add(head.positions.last().unwrap(), direction);
        &head.positions.push(new_head);
        let knot1 = *rope[1].positions.last().unwrap();
        move_knot(&new_head, &knot1, rope[1].positions);
    }
}

#[allow(dead_code)]
fn moves(cmds: Vec<Move>) -> (Vec<Pos>, Vec<Pos>) {
    let mut head_pos = vec![Pos(0,0)];
    let mut knot1_pos = vec![Pos(0,0)];
    for cmd in cmds {
        run_cmd(cmd, &vec![ Knot { positions: &head_pos }, Knot { positions: &knot1_pos } ]);
    }
    (head_pos, knot1_pos)
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
        assert_eq!(compute_move(&Pos(1,1), &Pos(0,0)), Direction(0,0));
        assert_eq!(compute_move(&Pos(-1,-1), &Pos(0,0)), Direction(0,0));
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
        ]);
    }

    #[test]
    fn it_solve() { 
        let tail_pos = moves(read_input(inputs::INPUT)).1;

        assert_eq!(HashSet::<Pos>::from_iter::<Vec<Pos>>(tail_pos).len(), 2724);
    }
}
