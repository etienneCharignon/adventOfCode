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

pub fn explore_region(pos: Pos, map: &Vec<Vec<char>>, height: i32, width: i32, visited: &mut HashSet<Pos>, fences: &mut Vec<(Pos, Pos)>) -> usize {
    let plant = map[pos.y as usize][pos.x as usize];
    let mut area = 0;
    while !visited.contains(&pos) {
        area += 1;
        visited.insert(pos);
        let directions = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
        for d in directions {
            let neighboug = add(pos, d);
            if neighboug.x < 0 || neighboug.y < 0 || neighboug.x >=width || neighboug.y >= height {
                fences.push((neighboug, d));
                continue;
            }
            if plant == map[neighboug.y as usize][neighboug.x as usize] {
                let a = explore_region(neighboug, map, height, width, visited, fences);
                area += a;
            }
            else {
                fences.push((neighboug, d));
            }
        }
    }
    area
}

pub fn visite_side(pos: (Pos, Pos), visited: &mut HashSet<(Pos, Pos)>, fences: &Vec<(Pos, Pos)>) {
    let directions = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
    for d in directions {
        let neighboug = add(pos.0, d);
        let neighbour_fence = (neighboug, pos.1);
        if fences.contains(&neighbour_fence) && !visited.contains(&neighbour_fence) {
            visited.insert(neighbour_fence);
            visite_side(neighbour_fence, visited, fences);
        }
    }
}

pub fn count_sides(fences: &Vec<(Pos, Pos)>) -> usize {
    let mut visited = HashSet::<(Pos, Pos)>::new();
    let mut sides = 0;
    println!("{:?}", fences);
    for f in fences {
        if visited.contains(f) {
            continue;
        }
        visited.insert(*f);
        sides += 1;
        visite_side(*f, &mut visited, fences);
    }
    sides
}

pub fn compute_price(world: (Vec<Vec<char>>, i32, i32)) -> usize {
    let (map, height, width) = world;
    let mut visited = HashSet::<Pos>::new();
    let mut price = 0;
    for x in 0..width {
        for y in 0..height {
            let pos = Pos {x: x as i32, y: y as i32};
            if ! visited.contains(&pos) {
                let mut fences = vec![];
                let a = explore_region(pos, &map, height, width, &mut visited, &mut fences);
                let sides = count_sides(&fences);
                println!("{sides}");
                price += a * sides;
            }
        }
    }
    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_explore_region() {
        let (map, height, width) = read(inputs::EXAMPLE_X);
        let mut visited = HashSet::<Pos>::new();
        let mut fences = vec![];
        assert_eq!(explore_region(Pos{x:1, y:1}, &map, height, width, &mut visited, &mut fences), 1);
        let mut visited = HashSet::<Pos>::new();
        let mut fences = vec![];
        assert_eq!(explore_region(Pos{x:0, y:0}, &map, height, width, &mut visited, &mut fences), 21);
        assert_eq!(fences.len(), 36)
    }

    #[test]
    fn it_works() {
        assert_eq!(compute_price(read(inputs::EXAMPLE_E)), 236);
        assert_eq!(compute_price(read(inputs::INPUT)), 787680);
    }
}
