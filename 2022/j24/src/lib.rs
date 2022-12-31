mod inputs;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Direction(i32, i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Obstacle(i32, i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Position(pub i32, pub i32, pub i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Blizzard(i32, i32, Direction);

#[allow(dead_code)]
const DIRECTIONS: [Direction; 5] = [
        Direction(1, 0),
        Direction(0, 1),
        Direction(0, -1),
        Direction(-1, 0),
        Direction(0, 0),
    ];

#[allow(dead_code)]
pub fn read_blizzards(input: &str) -> (usize, usize, Vec<Blizzard>) {
    let mut blizzards: Vec<Blizzard> = vec![];
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let d = match c {
                '>' => 0,
                'v' => 1,
                '^' => 2,
                '<' => 3,
                '#' => 4,
                _ => continue
            };
            blizzards.push(Blizzard(x as i32, y as i32, DIRECTIONS[d]));
        }
    }
    (input.lines().nth(0).unwrap().len(), input.lines().count(), blizzards)
}

fn loop_neg(p: i32, size: i32) -> i32 {
    if p <= 0 { p + size } else { p }
}

fn obstacle(time: i32, w: i32, h: i32, blizzard: Blizzard) -> Obstacle {
    if blizzard.2 == DIRECTIONS[4] {
        return Obstacle(blizzard.0, blizzard.1);
    }
    let o = Obstacle(blizzard.0 + (time * blizzard.2.0), blizzard.1 + (time * blizzard.2.1));
    Obstacle(loop_neg((o.0 - 1) % (w  - 2) + 1, w - 2), loop_neg((o.1 - 1) % (h - 2) + 1, h - 2))
}

#[allow(dead_code)]
fn get_map(time: i32, blizzards: (usize, usize, Vec<Blizzard>)) -> HashSet<Obstacle> {
    let mut obstacles = HashSet::new();
    for blizzard in blizzards.2 {
        obstacles.insert(obstacle(time, blizzards.0 as i32, blizzards.1 as i32, blizzard));
    }
    obstacles
}

#[allow(dead_code)]
fn print_map(w: usize, h: usize, map: &HashSet<Obstacle>, current: Position) -> String {
    let mut print = String::new();
    for y in 0..h {
        for x in 0..w {
            print += if map.contains(&Obstacle(x as i32,y as i32)) {
                " "
            } else if current.0 == x as i32 && current.1 == y as i32 {
                "E"
            }
            else {
                "."
            }
        }
        print += "\n";
    }
    print
}

#[allow(dead_code)]
fn reconstruct_path(came_from: &HashMap<Position, Position>, start: Position) -> Vec<Position> {
    let mut current = start;
    let mut total_path = vec![current];
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.insert(0, current);
    }
    total_path
}

fn length(a: Position, b: Position) -> usize {
    (b.0-a.0).abs() as usize + (b.1 - a.1).abs() as usize
}

fn smallest(open_set: &HashSet<Position>, fscore: &HashMap<Position, usize>) -> Position {
    *open_set.iter()
            .map(|position| (position, fscore.get(position).clone().unwrap_or(&usize::MAX)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0
}

fn neighbor(a: Position, b: Direction) -> Position {
    Position(a.0+b.0, a.1+b.1, a.2 + 1)
}

fn neighbors(current: Position, obstacles: &HashSet<Obstacle>, max: i32) -> Vec<Position> {
    let mut result: Vec<Position> = vec![];
    for d in 0..5 {
        let p = neighbor(current, DIRECTIONS[d]);
        if p.1 >= 0 && p.1 < max && ! obstacles.contains(&Obstacle(p.0, p.1)) {
            result.push(p);
        }
    }
    result
}

#[allow(dead_code)]
pub fn find_path(start: Position, goal: Position, blizzards: (usize, usize, Vec<Blizzard>)) -> Position {
    let mut open_set = HashSet::from([start]);
    let mut gscore: HashMap<Position, usize> =HashMap::new();
    gscore.insert(start, 0);
    let mut fscore: HashMap<Position, usize> =HashMap::new();
    fscore.insert(start, length(start, goal));
    let mut world: HashMap<i32, HashSet<Obstacle>> =HashMap::new();
    let mut winner = Position(0, 0, i32::MAX);
    while !open_set.is_empty() {
        let current = smallest(&open_set, &fscore).to_owned();

        //let map_str = to_string(inputs::SCREEN, reconstruct_path(&came_from, current));
        //println!("{}", map_str);
        if current.0 == goal.0 && current.1 == goal.1 {
            winner = if winner.2 < current.2 { winner } else { current };
            println!("{:?}", winner);
            break;
        }
        let next_tick = current.2 + 1;
        if ! world.contains_key(&next_tick) {
            world.insert(next_tick, get_map(next_tick, (blizzards.0, blizzards.1, blizzards.2.clone())));
        }
        let obstacles = world.get(&next_tick).unwrap();
        // println!("minute {} :\n{}", current.2, print_map(blizzards.0, blizzards.1, &obstacles, current));
        for neighbor in neighbors(current, &obstacles, blizzards.1 as i32) {
            let tentative_gscore = neighbor.2 as usize;
            if tentative_gscore < gscore.get(&neighbor).cloned().unwrap_or(usize::MAX) {
                gscore.insert(neighbor, tentative_gscore);
                fscore.insert(neighbor, tentative_gscore + length(neighbor, goal));
                open_set.insert(neighbor);
            }
        }
        open_set.remove(&current);
    }
    winner
}

#[allow(dead_code)]
pub fn solve(start: Position, goal: Position, blizzards: (usize, usize, Vec<Blizzard>)) -> Position {
    let mut winner = find_path(start, goal, (blizzards.0, blizzards.1, blizzards.2.clone()));
    // let mut winner = Position(120, 26, 314);
    println!("arrived but…");
    winner = find_path(winner, Position(1, 0, i32::MAX), (blizzards.0, blizzards.1, blizzards.2.clone()));
    // let mut winner = Position(1, 0, 574);
    println!("went back !");
    find_path(winner, goal, (blizzards.0, blizzards.1, blizzards.2.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_find_smallest() {
        let p1 = Position(1,0,0);
        let p2 = Position(1,1,0);
        assert_eq!(smallest(&HashSet::from([p1, p2]), &HashMap::from([(p1, 1), (p2, 2)])), p1);
        assert_eq!(smallest(&HashSet::from([p1, p2]), &HashMap::from([(p1, 2), (p2, 1)])), p2);
        assert_eq!(smallest(&HashSet::from([p1, p2]), &HashMap::from([(p1, 2)])), p1);
    }

    #[test]
    fn it_read_blizzards() {
        assert_eq!(read_blizzards(inputs::SIMPLE_EXAMPLE), (7, 7, vec![
            Blizzard(0, 0, Direction(0, 0)), Blizzard(2, 0, Direction(0, 0)), Blizzard(3, 0, Direction(0, 0)),
            Blizzard(4, 0, Direction(0, 0)), Blizzard(5, 0, Direction(0, 0)), Blizzard(6, 0, Direction(0, 0)),

            Blizzard(0, 1, Direction(0, 0)), Blizzard(6, 1, Direction(0, 0)),
            Blizzard(0, 2, Direction(0, 0)), Blizzard(1, 2, Direction(1, 0)), Blizzard(6, 2, Direction(0, 0)),
            Blizzard(0, 3, Direction(0, 0)), Blizzard(6, 3, Direction(0, 0)),
            Blizzard(0, 4, Direction(0, 0)), Blizzard(4, 4, Direction(0, 1)), Blizzard(6, 4, Direction(0, 0)),
            Blizzard(0, 5, Direction(0, 0)), Blizzard(6, 5, Direction(0, 0)),
            Blizzard(0, 6, Direction(0, 0)), Blizzard(1, 6, Direction(0, 0)),
            Blizzard(2, 6, Direction(0, 0)), Blizzard(3, 6, Direction(0, 0)),
            Blizzard(4, 6, Direction(0, 0)), Blizzard(6, 6, Direction(0, 0)),
        ]));
    }

    #[test]
    fn it_move_blizzards_in_time() {
        assert_eq!(obstacle(1, 8, 9, Blizzard(1, 1, Direction(-1, 0))), Obstacle(6, 1));
        assert_eq!(obstacle(1, 8, 9, Blizzard(1, 1, Direction(0, -1))), Obstacle(1, 7));
    }

    #[test]
    fn it_get_map() {
        assert_eq!(get_map(0, read_blizzards(inputs::SIMPLE_EXAMPLE)), HashSet::from_iter(vec![
            Obstacle(1, 2), Obstacle(4, 4), 
            Obstacle(0, 0), Obstacle(2, 0), Obstacle(3, 0), Obstacle(4, 0), Obstacle(5, 0), Obstacle(6, 0),
            Obstacle(0, 1), Obstacle(6, 1),
            Obstacle(0, 2), Obstacle(6, 2),
            Obstacle(0, 3), Obstacle(6, 3),
            Obstacle(0, 4), Obstacle(6, 4),
            Obstacle(0, 5), Obstacle(6, 5),
            Obstacle(0, 6), Obstacle(6, 6),
            Obstacle(1, 6), Obstacle(2, 6), Obstacle(3, 6), Obstacle(4, 6),
        ]));
        assert_eq!(get_map(1, read_blizzards(inputs::SIMPLE_EXAMPLE)), HashSet::from_iter(vec![
            Obstacle(2, 2),
            Obstacle(4, 5),
            Obstacle(0, 0), Obstacle(2, 0), Obstacle(3, 0), Obstacle(4, 0), Obstacle(5, 0), Obstacle(6, 0),
            Obstacle(0, 1), Obstacle(6, 1),
            Obstacle(0, 2), Obstacle(6, 2),
            Obstacle(0, 3), Obstacle(6, 3),
            Obstacle(0, 4), Obstacle(6, 4),
            Obstacle(0, 5), Obstacle(6, 5),
            Obstacle(0, 6), Obstacle(6, 6),
            Obstacle(1, 6), Obstacle(2, 6), Obstacle(3, 6), Obstacle(4, 6),
        ]));
        assert_eq!(get_map(2, read_blizzards(inputs::SIMPLE_EXAMPLE)), HashSet::from_iter(vec![
            Obstacle(3, 2),
            Obstacle(4, 1),
            Obstacle(0, 0), Obstacle(2, 0), Obstacle(3, 0), Obstacle(4, 0), Obstacle(5, 0), Obstacle(6, 0),
            Obstacle(0, 1), Obstacle(6, 1),
            Obstacle(0, 2), Obstacle(6, 2),
            Obstacle(0, 3), Obstacle(6, 3),
            Obstacle(0, 4), Obstacle(6, 4),
            Obstacle(0, 5), Obstacle(6, 5),
            Obstacle(0, 6), Obstacle(6, 6),
            Obstacle(1, 6), Obstacle(2, 6), Obstacle(3, 6), Obstacle(4, 6),
        ]));
    }

    #[test]
    fn it_find_neighbor() {
        assert_eq!(neighbors(Position(1, 0, 0), &HashSet::from_iter(vec![
            Obstacle(0, 0),
            Obstacle(2, 0),
            Obstacle(1, 1),
        ]), 5), vec![
            Position(1, 0, 1),
        ]);
    }

    #[test]
    fn it_find_path() {
        assert_eq!(solve(Position(1, 0, 0), Position(6, 5, i32::MAX), read_blizzards(inputs::EXAMPLE)).2, 54);
        assert_eq!(find_path(Position(1, 0, 0), Position(120, 26, i32::MAX), read_blizzards(inputs::INPUT)).2, 314);
        assert_eq!(find_path(Position(120, 26, 314), Position(1, 0, i32::MAX), read_blizzards(inputs::INPUT)).2, 574);
        assert_eq!(find_path(Position(1, 0, 574), Position(120, 26, i32::MAX), read_blizzards(inputs::INPUT)).2, 896);
    }

    #[test]
    fn it_solve_part_2() {
        assert_eq!(solve(Position(1, 0, 0), Position(120, 26, i32::MAX), read_blizzards(inputs::INPUT)).2, 896);
    }
}
