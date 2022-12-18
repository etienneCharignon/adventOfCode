mod inputs;
use std::io::Write;
use std::collections::HashSet;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(i32, i32);

const BARE: [Point; 4] = [Point(0,0), Point(1,0), Point(2,0), Point(3,0)];
const CROSS: [Point; 5] = [Point(1,0), Point(0,1), Point(1,1), Point(2,1), Point(1, 2)];
const L: [Point; 5] = [Point(0,0), Point(1,0), Point(2,0), Point(2,1), Point(2, 2)];
const V_BARE: [Point; 4] = [Point(0,0), Point(0,1), Point(0,2), Point(0,3)];
const SQARE: [Point; 4] = [Point(0,0), Point(0,1), Point(1,0), Point(1,1)];

fn add(a: Point, b: Point) -> Point {
    Point(a.0+b.0, a.1+b.1)
}

fn move_shape(shape: &Vec<Point>, p: Point) -> Vec<Point> {
    shape.iter().map(|sp| add(*sp, p)).collect()
}


fn start_falling(world: &Vec<HashSet<i32>>, shape: Vec<Point>) -> Vec<Point> {
    let shape_y: i32 = (3 + world.len()) as i32;
    move_shape(&shape, Point(2, shape_y))
}

fn blow(shape: &Vec<Point>, jet: char) -> Vec<Point>{
    let new_shape = match jet {
        '>' => move_shape(shape, Point(1, 0)),
        '<' => move_shape(shape, Point(-1, 0)),
        _ => panic!("{} not managed", jet)
    };
    if new_shape.iter().any(|p| p.0 < 0 || p.0 > 6) { shape.to_vec() } else { new_shape }
}

fn collision(shape: &Vec<Point>, world: &Vec<HashSet<i32>>) -> bool {
    shape.iter()
         .any(|p| {
            if p.1 < 0 { return true; }
            let y = p.1 as usize;
            world.len() > y && world[y].contains(&p.0)
         })
}

fn add_to_world(world: &mut Vec<HashSet<i32>>, shape: &Vec<Point>) {
    for p in shape.iter() {
        let y = p.1 as usize;
        while world.len() <= y {
            world.push(HashSet::new());
        }
        world[y].insert(p.0);
    }
}

#[allow(dead_code)]
pub fn fall_to(number_of_shapes: usize, world: &mut Vec<HashSet<i32>>, jets: Vec<char>) -> usize {
    let shapes = vec![
        Vec::from(BARE),
        Vec::from(CROSS),
        Vec::from(L),
        Vec::from(V_BARE),
        Vec::from(SQARE),
    ];
    let mut ji = 0;
    let mut size_of_last_147 = 0;
    let mut number_of_remaining_drops = 147;
    let mut size_first_iteration = 0;
    let mut dropped_first_iteration = 0;
    let mut dropped_second_iteration = 0;
    let mut size_second_iteration = 0;
    for n in 0..number_of_shapes {
        let mut shape = start_falling(world, shapes[n % shapes.len()].to_vec());
        //println!("New SHAPE");
        loop {
            //println!("{}, {:?}", jets[ji % jets.len()], shape);
            let mut new_shape = blow(&shape, jets[ji % jets.len()]);
            if n == dropped_first_iteration + number_of_remaining_drops {
                println!("n : {}, {}", n, world.len());
                size_of_last_147 = world.len() - size_first_iteration;
            }
            if ji%jets.len() == 0 {
                let jet_iteration = ji/jets.len();
                if jet_iteration == 1 {
                    size_first_iteration = world.len();
                    dropped_first_iteration = n;
                }
                if jet_iteration == 2 {
                    size_second_iteration = world.len() - size_first_iteration;
                    dropped_second_iteration = n - dropped_first_iteration;
                    number_of_remaining_drops = (1000000000000 - dropped_first_iteration) % dropped_second_iteration;
                    println!("remaining drops {}", number_of_remaining_drops);
                }
                println!("{}, {}, {}", n, jet_iteration, world.len());
                //println!("{}, {:?}", jets[ji % jets.len()], shape);
                std::io::stdout().flush();
                if jet_iteration == 10 {
                    let number_of_repetition = (1000000000000 - dropped_first_iteration) / dropped_second_iteration;
                    println!("{}", number_of_repetition);
                    return size_first_iteration + size_second_iteration * number_of_repetition + size_of_last_147;
                }
            }
            ji += 1;
            if !collision(&new_shape, world) {
               shape = new_shape;
            }
            new_shape = move_shape(&shape, Point(0, -1));
            if collision(&new_shape, world) {
                add_to_world(world, &shape);
                break;
            }
            else { shape = new_shape; }
        }
    }
    world.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_start_falling() {
        let world: Vec<HashSet<i32>> = vec![];
        assert_eq!(start_falling(&world, Vec::from(BARE)), vec![Point(2, 3), Point(3,3), Point(4,3), Point(5,3)]);

        let world: Vec<HashSet<i32>> = vec![HashSet::from([2, 3, 4, 5])];
        assert_eq!(start_falling(&world, Vec::from(BARE)), vec![Point(2, 4), Point(3,4), Point(4,4), Point(5,4)]);
    }

    #[test]
    fn it_falls_one() {
        let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".chars().collect();
        let mut world: Vec<HashSet<i32>> = vec![];
        assert_eq!(fall_to(1, &mut world, jets), 1);
        assert_eq!(world, vec![HashSet::from([2, 3, 4, 5])]);
    }

    #[test]
    fn it_falls_two() {
        let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".chars().collect();
        let mut world: Vec<HashSet<i32>> = vec![];
        assert_eq!(fall_to(2, &mut world, jets), 4);
        assert_eq!(world, vec![
            HashSet::from([2, 3, 4, 5]),
            HashSet::from([3]),
            HashSet::from([2, 3, 4]),
            HashSet::from([3]),
        ]);
    }

    #[test]
    fn it_falls_three() {
        let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".chars().collect();
        let mut world: Vec<HashSet<i32>> = vec![];
        assert_eq!(fall_to(3, &mut world, jets), 6);
        assert_eq!(world, vec![
            HashSet::from([2, 3, 4, 5]),
            HashSet::from([3]),
            HashSet::from([2, 3, 4]),
            HashSet::from([0, 1, 2, 3]),
            HashSet::from([2]),
            HashSet::from([2]),
        ]);
    }

    #[test]
    fn it_falls_four() {
        let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".chars().collect();
        let mut world: Vec<HashSet<i32>> = vec![];
        assert_eq!(fall_to(4, &mut world, jets), 7);
        assert_eq!(world, vec![
            HashSet::from([2, 3, 4, 5]),
            HashSet::from([3]),
            HashSet::from([2, 3, 4]),
            HashSet::from([0, 1, 2, 3, 4]),
            HashSet::from([2, 4]),
            HashSet::from([2, 4]),
            HashSet::from([4]),
        ]);
    }

    #[test]
    #[ignore]
    fn it_falls_2022_example() {
        let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".chars().collect();
        let mut world: Vec<HashSet<i32>> = vec![];
        assert_eq!(fall_to(1000000000000, &mut world, jets), 1514285714288);
    }

    #[test]
    fn it_falls_2022() {
        let jets = inputs::INPUT.chars().collect();
        let mut world: Vec<HashSet<i32>> = vec![];
        assert_eq!(fall_to(1000000000000, &mut world, jets), 1567723342929);
    }
}
