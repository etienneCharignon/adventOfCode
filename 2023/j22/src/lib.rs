mod inputs;

use std::cmp;
use multimap::MultiMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Position(pub i64, pub i64, pub i64);

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Brick(Position, Position);

impl Brick {
    fn new(positions: Vec<&str>) -> Brick {
        let p1: Vec<i64> = positions[0].split(",").map(|c| c.parse().unwrap()).collect();
        let p2: Vec<i64> = positions[1].split(",").map(|c| c.parse().unwrap()).collect();
        Brick(Position(p1[0], p1[1], p1[2]), Position(p2[0], p2[1], p2[2]))
    }

    fn from(b: Brick, dz: i64) -> Brick {
        Brick(Position(b.0.0, b.0.1, b.0.2 + dz), Position(b.1.0, b.1.1, b.1.2 + dz))
    }

    fn max_x(&self) -> i64 {
        cmp::max(self.0.0, self.1.0)
    }

    fn max_y(&self) -> i64 {
        cmp::max(self.0.1, self.1.1)
    }

    fn max_z(&self) -> i64 {
        cmp::max(self.0.2, self.1.2)
    }

    fn min_x(&self) -> i64 {
        cmp::min(self.0.0, self.1.0)
    }

    fn min_y(&self) -> i64 {
        cmp::min(self.0.1, self.1.1)
    }

    fn min_z(&self) -> i64 {
        cmp::min(self.0.2, self.1.2)
    }
}

pub fn read(input: &str) -> Vec<Brick> {
    input.lines()
         .map(|line| line.split("~").collect())
         .map(|positions| Brick::new(positions) )
         .collect()
}

pub fn find_max_z(obrick: Option<Brick>, max_z: i64) -> i64 {
    match obrick {
        None => max_z,
        Some(brick) => cmp::max(max_z, brick.max_z()),
    }
}

pub fn add_supporters(supporters: &mut MultiMap<Brick, Brick>, brick: Brick, supporter: Brick) {
    match supporters.get_vec(&brick) {
        None => supporters.insert(brick, supporter),
        Some(existing_supporters) => {
            if ! existing_supporters.contains(&supporter) {
                supporters.insert(brick, supporter);
            }
        }
    }
}

pub fn move_brick(brick: Brick, surface: &mut Vec<Vec<Option<Brick>>>, supporters: &mut MultiMap<Brick, Brick>) -> Brick {
    let minx = brick.0.0 as usize;
    let maxx = brick.1.0 as usize;
    let miny = brick.0.1 as usize;
    let maxy = brick.1.1 as usize;
    if miny != maxy {
        let mut max_surface_z = 0;
        for y in miny..=maxy {
            max_surface_z = find_max_z(surface[minx][y], max_surface_z);
        }
        let new_brick = Brick::from(brick, -(brick.0.2 - max_surface_z) + 1);
        for y in miny..=maxy {
            match surface[minx][y] {
                None => {},
                Some(brick) => {
                    if brick.1.2 == max_surface_z {
                        add_supporters(supporters, new_brick, brick);
                    }
                }
            }
        }
        for y in miny..=maxy {
            surface[minx][y] = Some(new_brick);
        }
        return new_brick;
    }
    else if minx != maxx {
        let mut max_surface_z = 0;
        for x in minx..=maxx {
            max_surface_z = find_max_z(surface[x][miny], max_surface_z);
        }
        let new_brick = Brick::from(brick, -(brick.0.2 - max_surface_z) + 1);
        for x in minx..=maxx {
            match surface[x][miny] {
                None => {},
                Some(brick) => {
                    if brick.1.2 == max_surface_z {
                        add_supporters(supporters, new_brick, brick);
                    }
                }
            }
        }
        for x in minx..=maxx {
            surface[x][miny] = Some(new_brick);
        }
        return new_brick;
    }
    else {
        let max_surface_z = find_max_z(surface[minx][miny], 0);
        let new_brick = Brick::from(brick, -(brick.0.2 - max_surface_z) + 1);
        match surface[minx][miny] {
            None => {},
            Some(brick) => {
                if brick.1.2 == max_surface_z {
                    add_supporters(supporters, new_brick, brick);
                }
            }
        }
        surface[minx][miny] = Some(new_brick);
        return new_brick;
    }
}

pub fn new_surface(width: i64, depth: i64) -> Vec<Vec<Option<Brick>>> {
    let mut surface = Vec::<Vec<Option<Brick>>>::new();
    for _x in 0..width {
        let mut column = Vec::<Option<Brick>>::new();
        for _y in 0..depth {
            column.push(None);
        }
        surface.push(column);
    }
    surface
}

pub fn letter(position: usize) -> char {
    ('A' as u8 + TryInto::<u8>::try_into(position).unwrap()) as char
}

pub fn print_stack(bricks: &HashSet<Brick>) {
    let mut stack = Vec::from_iter(bricks.iter());
    stack.sort_by(|b1, b2| b1.min_x().cmp(&b2.min_x()));
    stack.sort_by(|b1, b2| b1.min_y().cmp(&b2.min_y()));
    stack.sort_by(|b1, b2| b1.min_z().cmp(&b2.min_z()));
    println!("{}", stack.len());
    for (p, b) in stack.iter().enumerate() {
        println!("{} : {:?}", letter(p), b);
    }

    let width = stack.iter().map(|b| b.max_x()).max().unwrap() + 1;
    let depth = stack.iter().map(|b| b.max_y()).max().unwrap() + 1;
    let height = stack.iter().map(|b| b.max_z()).max().unwrap() + 1;
    let mut x_rows = vec![];
    for z in 0..height {
        let mut row = String::new();
        for x in 0..width {
            let p = stack.iter().position(|b| x >= b.0.0 && x<= b.1.0 && z >= b.0.2 && z <= b.1.2);
            if p != None {
                row += &format!("{}", letter(p.unwrap()));
            }
            else {
                row.push('.');
            }
        }
        x_rows.push(row);
    }
    println!("");
    for z in (0..height).rev() {
        let mut row = String::new();
        for y in 0..depth {
            let p = stack.iter().position(|b| y >= b.0.1 && y<= b.1.1 && z >= b.0.2 && z <= b.1.2);
            if p != None {
                row += &format!("{}", letter(p.unwrap()));
            }
            else {
                row.push('.');
            }
        }
        println!("{}\t{row}", x_rows[z as usize]);
    }
    println!("");
}


pub fn count_candidate(stack: Vec<Brick>) -> usize {
    let mut sorted_z = stack;
    sorted_z.sort_by(|b1, b2| b1.min_z().cmp(&b2.min_z()));
    // println!("{sorted_z:?}");

    let width = sorted_z.iter().map(|b| b.max_x()).max().unwrap() + 1;
    let depth = sorted_z.iter().map(|b| b.max_y()).max().unwrap() + 1;

    let mut surface = new_surface(width, depth);

    let mut supporters = MultiMap::<Brick, Brick>::new();
    let mut desintegrable = HashSet::<Brick>::new();
    // let mut dropping = HashSet::<Brick>::new();
    for brick in sorted_z {
        // dropping.insert(brick);
        // if dropping.len() < 26 {
        //     println!("Dropping");
        //     print_stack(&dropping);
        // }
        let moved_brick = move_brick(brick, &mut surface, &mut supporters);
        desintegrable.insert(moved_brick);
        // if desintegrable.len() < 26 {
        //     println!("SUPPORTERS");
        //     for (brick, bricks) in supporters.iter_all() {
        //         println!("{brick:?} : {} {bricks:?}", bricks.len());
        //     }
        //     println!("Dropped");
        //     print_stack(&desintegrable);
        // }
    }
    for (_brick, bricks) in supporters.iter_all() {
        if bricks.len() == 1 {
            desintegrable.remove(&bricks[0]);
        }
    }
    desintegrable.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_move_brick_on_z() {
        let mut supporters = MultiMap::<Brick, Brick>::new();
        let mut surface = new_surface(3, 3);
        move_brick(Brick::new(vec!["0,0,1", "0,0,3"]), &mut surface, &mut supporters);
        assert_eq!(supporters.len(), 0);
        assert_eq!(surface[0][0].unwrap(), Brick::new(vec!["0,0,1", "0,0,3"]));
        move_brick(Brick::new(vec!["0,0,6", "0,0,7"]), &mut surface, &mut supporters);
        assert_eq!(surface[0][0].unwrap(), Brick::new(vec!["0,0,4", "0,0,5"]));
        assert_eq!(supporters.len(), 1);
        assert_eq!(supporters.get(&Brick::new(vec!["0,0,4", "0,0,5"])), Some(&Brick::new(vec!["0,0,1", "0,0,3"])));
    }

    #[test]
    fn it_move_brick_on_x() {
        let mut supporters = MultiMap::<Brick, Brick>::new();
        let mut surface = new_surface(3, 3);
        move_brick(Brick::new(vec!["0,0,1", "1,0,1"]), &mut surface, &mut supporters);
        assert_eq!(supporters.len(), 0);
        assert_eq!(surface[0][0].unwrap(), Brick::new(vec!["0,0,1", "1,0,1"]));
        assert_eq!(surface[1][0].unwrap(), Brick::new(vec!["0,0,1", "1,0,1"]));
        move_brick(Brick::new(vec!["1,0,3", "1,2,3"]), &mut surface, &mut supporters);
        assert_eq!(surface[1][0].unwrap(), Brick::new(vec!["1,0,2", "1,2,2"]));
        assert_eq!(surface[1][1].unwrap(), Brick::new(vec!["1,0,2", "1,2,2"]));
        assert_eq!(surface[1][2].unwrap(), Brick::new(vec!["1,0,2", "1,2,2"]));
        assert_eq!(supporters.len(), 1);
        assert_eq!(supporters.get(&Brick::new(vec!["1,0,2", "1,2,2"])), Some(&Brick::new(vec!["0,0,1", "1,0,1"])));
    }

    #[test]
    fn it_move_brick_on_y() {
        let mut supporters = MultiMap::<Brick, Brick>::new();
        let mut surface = new_surface(3, 3);
        move_brick(Brick::new(vec!["1,0,1", "1,2,1"]), &mut surface, &mut supporters);
        assert_eq!(supporters.len(), 0);
        assert_eq!(surface[1][0].unwrap(), Brick::new(vec!["1,0,1", "1,2,1"]));
        assert_eq!(surface[1][1].unwrap(), Brick::new(vec!["1,0,1", "1,2,1"]));
        assert_eq!(surface[1][2].unwrap(), Brick::new(vec!["1,0,1", "1,2,1"]));
        move_brick(Brick::new(vec!["1,0,3", "1,2,3"]), &mut surface, &mut supporters);
        assert_eq!(surface[1][0].unwrap(), Brick::new(vec!["1,0,2", "1,2,2"]));
        assert_eq!(surface[1][1].unwrap(), Brick::new(vec!["1,0,2", "1,2,2"]));
        assert_eq!(surface[1][2].unwrap(), Brick::new(vec!["1,0,2", "1,2,2"]));
        assert_eq!(supporters.len(), 1);
        assert_eq!(supporters.get(&Brick::new(vec!["1,0,2", "1,2,2"])), Some(&Brick::new(vec!["1,0,1", "1,2,1"])));
    }

    #[test]
    fn it_count_candidate() {
        assert_eq!(count_candidate(read(inputs::EXAMPLE)), 5);
        assert_eq!(count_candidate(read(inputs::INPUT)), 405); // 418 to high, 283 to low
    }
}
