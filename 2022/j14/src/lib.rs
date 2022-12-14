mod inputs;

use std::io::Write;
use std::{thread, time};
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(usize, usize);
#[derive(Debug, Clone, PartialEq)]
pub struct Obstacle {
    points: Vec<Point>
}

const L:usize = 600;

#[allow(unused_imports)]
use std::cmp;

fn read_point(input: &str) -> Point {
    let coordinates: Vec<&str> = input.split(",").collect();
    Point(coordinates[0].parse::<usize>().unwrap(), coordinates[1].parse::<usize>().unwrap())
}

#[allow(dead_code)]
pub fn read_world_2(input: &str) -> [[char; L]; 164] {
    let mut world: [[char; L]; 164] = [[' '; L]; 164];
    for line in input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        for i in 1..points.len() {
            let a = read_point(points[i-1]);
            let b = read_point(points[i]);
            if a.1 == b.1 {
                let y = a.1;
                let range = if a.0 < b.0 { a.0..=b.0 } else { b.0..=a.0 };
                for x in range {
                    world[y][x] = '#';
                }
            }
            else {
                let x = a.0;
                let range = if a.1 < b.1 { a.1..=b.1 } else { b.1..=a.1 };
                for y in range {
                    world[y][x] = '#';
                }
            }
        }
    }
    world

    // let mut result: Vec<String> = vec![];
    // for r in 0..164 {
    //     result.push(String::from_iter(world[r]));
    // }
    // result
}

#[allow(dead_code)]
pub fn read_world(input: &str) -> (usize, Vec<Obstacle>) {
    let mut world = vec![];
    let mut maxy = 0;
    for line in input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        for i in 1..points.len() {
            let a = read_point(points[i-1]);
            let b = read_point(points[i]);
            if a.1 == b.1 {
                if a.0 < b.0 {
                    world.push(Obstacle { points: vec![a, b] });
                }
                else {
                    world.push(Obstacle { points: vec![b, a] });
                }
            }
            else {
                if a.1 < b.1 {
                    world.push(Obstacle { points: vec![a, b] });
                }
                else {
                    world.push(Obstacle { points: vec![b, a] });
                }
            }
            maxy = cmp::max(maxy, a.1);
            maxy = cmp::max(maxy, b.1);
        }
    }
    (maxy, world)
}

fn contains(world: &Vec<Obstacle>, point: Point) -> bool {
    for obstacle in world {
        if obstacle.points.len() == 1 {
            if obstacle.points[0] == point {
                return true;
            }
        }
        else {
            if obstacle.points[0].1 == obstacle.points[1].1 {
                let y = obstacle.points[0].1;
                let x0 = obstacle.points[0].0;
                let x1 = obstacle.points[1].0;
                if point.1 == y && point.0 >= x0 && point.0 <= x1 {
                    return true;
                }
            }
            else {
                let x = obstacle.points[0].0;
                let y0 = obstacle.points[0].1;
                let y1 = obstacle.points[1].1;
                if point.0 == x && point.1 >= y0 && point.1 <= y1 {
                    return true;
                }
            }
        }
    }
    false
}

#[allow(dead_code)]
fn drop_sand_2(world: &mut [[char; L]; 164]) -> usize {
    let mut x = 500;
    if world[0][500] != ' ' {
        return 0;
    }
    for y in 0..world.len() {
        //println!("{}, {}", y, contains(world, Point(x, y + 1)));
        if world[y+1][x] != ' ' {
            if world[y + 1][x - 1] == ' ' {
                x = x - 1;
            }
            else {
                if world[y + 1][x + 1] == ' ' {
                    x = x + 1;
                }
                else {
                    world[y][x] = '*';
                    return 1;
                }
            }
        }
    }
    0
}

#[allow(dead_code)]
fn drop_sand(world: &mut Vec<Obstacle>, maxy: usize) -> usize {
    let mut x = 500;
    if contains(world, Point(500, 0)){
        return 0;
    }
    for y in 0..maxy {
        //println!("{}, {}", y, contains(world, Point(x, y + 1)));
        if contains(world, Point(x, y + 1)) {
            if !contains(world, Point(x - 1, y + 1)) {
                x = x - 1;
            }
            else {
                if !contains(world, Point(x + 1, y + 1)) {
                    x = x + 1;
                }
                else {
                    world.insert(0, Obstacle { points: vec![Point(x, y)] });
                    return 1;
                }
            }
        }
    }
    0
}

#[allow(dead_code)]
pub fn drop_all_sand(world: &mut [[char; L]; 164]) -> usize {
    let mut count = 0;
    while drop_sand_2(world) == 1 {
        count += 1;
        for r in 0..164 {
            println!("{}", String::from_iter(world[r][300..599]));
        }
        std::io::stdout().flush();
        thread::sleep(time::Duration::from_millis(1));
    }
    count
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_contains() {
        assert!(contains(&read_world("494,9 -> 502,9").1, Point(500, 9)));
        assert!(contains(&read_world("500,5 -> 500,10").1, Point(500, 5)));
        assert!(contains(&read_world("500,5 -> 500,10").1, Point(500, 7)));
        assert!(contains(&read_world("500,5 -> 500,10").1, Point(500, 10)));
        assert!(contains(&vec![Obstacle { points: vec![Point(500,4)] }], Point(500, 4)));
        assert!(!contains(&vec![Obstacle { points: vec![Point(500,4)] }], Point(500, 5)));
        assert!(contains(&read_world("494,9 -> 502,9\n494,10 -> 502,10").1, Point(500, 10)));
    }

    #[test]
    fn it_read_world() {
        assert_eq!(read_world("498,4 -> 498,6 -> 496,6"), (6, vec![
            Obstacle { points: vec![Point(498,4), Point(498,6)] },
            Obstacle { points: vec![Point(496,6), Point(498,6)] },
        ]));
    }

    #[test]
    fn it_read_world_2() {
        assert_eq!(&String::from_iter(read_world_2("0,0 -> 0,1 -> 3,1")[0])[0..10], "#         ");
        assert_eq!(&String::from_iter(read_world_2("0,0 -> 0,1 -> 3,1")[1])[0..10], "####      ");
    }

    #[test]
    fn it_drop_sand() {
        let (maxy, mut world) = read_world("502,9 -> 494,9");
        drop_sand(&mut world, maxy);
        assert_eq!(world, vec![
            Obstacle { points: vec![Point(500,8)] },
            Obstacle { points: vec![Point(494,9), Point(502,9)] },
        ]);
        drop_sand(&mut world, maxy);
        assert_eq!(world, vec![
            Obstacle { points: vec![Point(499,8)] },
            Obstacle { points: vec![Point(500,8)] },
            Obstacle { points: vec![Point(494,9), Point(502,9)] },
        ]);
        drop_sand(&mut world, maxy);
        assert_eq!(world, vec![
            Obstacle { points: vec![Point(501,8)] },
            Obstacle { points: vec![Point(499,8)] },
            Obstacle { points: vec![Point(500,8)] },
            Obstacle { points: vec![Point(494,9), Point(502,9)] },
        ]);
    }

    #[test]
    fn it_drop_all_sand() {
        let mut world = read_world_2(inputs::INPUT);
        assert_eq!(drop_all_sand(&mut world), 24958);
    }

}
