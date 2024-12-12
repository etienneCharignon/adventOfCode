mod inputs;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}

pub fn add(a: Pos, b: Pos) -> Pos {
    Pos {x: a.x + b.x, y: a.y + b.y }
}

pub fn read(input: &str) -> (Vec<Vec<char>>, i32, i32) {
    let mut world = vec![];
    let lines: Vec<&str> = input.split('\n').collect();
    let height = lines.len();
    let width = lines[0].len();
    for line in lines {
        world.push(line.chars().collect());
    }
    (world, height as i32, width as i32)
}

pub fn explore_region(pos: Pos, map: &Vec<Vec<char>>, height: i32, width: i32, visited: &mut HashSet<Pos>) -> (usize, usize) {
    let plant = map[pos.y as usize][pos.x as usize];
    let mut area = 0;
    let mut fences = 0;
    while !visited.contains(&pos) {
        area += 1;
        visited.insert(pos);
        let directions = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
        for d in directions {
            let neighboug = add(pos, d);
            if neighboug.x < 0 || neighboug.y < 0 || neighboug.x >=width || neighboug.y >= height {
                fences += 1;
                continue;
            }
            if plant == map[neighboug.y as usize][neighboug.x as usize] {
                let (a, f)= explore_region(neighboug, map, height, width, visited);
                area += a;
                fences += f;
            }
            else {
                fences +=1;
            }
        }
    }
    (area, fences)
}

pub fn compute_price(world: (Vec<Vec<char>>, i32, i32)) -> usize {
    let (map, height, width) = world;
    let mut visited = HashSet::<Pos>::new();
    let mut price = 0;
    for x in 0..width {
        for y in 0..height {
            let pos = Pos {x: x as i32, y: y as i32};
            if ! visited.contains(&pos) {
                let (a, f) = explore_region(pos, &map, height, width, &mut visited);
                price += a * f;
            }
        }
    }
    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_exploire_region() {
        let (map, height, width) = read(inputs::EXAMPLE_X);
        let mut visited = HashSet::<Pos>::new();
        assert_eq!(explore_region(Pos{x:1, y:1}, &map, height, width, &mut visited), (1,4));
        let mut visited = HashSet::<Pos>::new();
        assert_eq!(explore_region(Pos{x:0, y:0}, &map, height, width, &mut visited), (21, 36));
    }

    #[test]
    fn it_works() {
        assert_eq!(compute_price(read(inputs::EXAMPLE)), 1930);
        assert_eq!(compute_price(read(inputs::INPUT)), 1363682);
    }
}
