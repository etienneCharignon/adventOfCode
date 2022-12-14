mod inputs;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(usize, usize);
#[derive(Debug, Clone, PartialEq)]
pub struct Obstacle {
    points: Vec<Point>
}

#[allow(unused_imports)]
use std::cmp;

fn read_point(input: &str) -> Point {
    let coordinates: Vec<&str> = input.split(",").collect();
    Point(coordinates[0].parse::<usize>().unwrap(), coordinates[1].parse::<usize>().unwrap())
}


#[allow(dead_code)]
fn read_world(input: &str) -> (usize, Vec<Obstacle>) {
    let mut world = vec![];
    let mut maxy = 0;
    for line in input.lines() {
        let points: Vec<&str> = line.split(" -> ").collect();
        for i in 1..points.len() {
            let a = read_point(points[i-1]);
            let b = read_point(points[i]);
            world.push(Obstacle { points: vec![a, b] });
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
                let range = if x0 < x1 { x0..=x1 } else { x1..=x0 };
                if point.1 == y && (range).contains(&point.0) {
                    return true;
                }
            }
            else {
                let x = obstacle.points[0].0;
                let y0 = obstacle.points[0].1;
                let y1 = obstacle.points[1].1;
                let range = if y0 < y1 { y0..=y1 } else { y1..=y0 };
                if point.0 == x && (range).contains(&point.1) {
                    return true;
                }
            }
        }
    }
    false
}

#[allow(dead_code)]
fn drop_sand(world: &mut Vec<Obstacle>, maxy: usize) -> usize {
    let mut x = 500;
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
                    world.push(Obstacle { points: vec![Point(x, y)] });
                    return 1;
                }
            }
        }
    }
    0
}

#[allow(dead_code)]
fn drop_all_sand(world: &mut Vec<Obstacle>, maxy: usize) -> usize {
    let mut count = 0;
    while drop_sand(world, maxy) == 1 {
        count += 1;
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
            Obstacle { points: vec![Point(498,6), Point(496,6)] },
        ]));
    }

    #[test]
    fn it_drop_sand() {
        let (maxy, mut world) = read_world("502,9 -> 494,9");
        drop_sand(&mut world, maxy);
        assert_eq!(world, vec![
            Obstacle { points: vec![Point(502,9), Point(494,9)] },
            Obstacle { points: vec![Point(500,8)] },
        ]);
        drop_sand(&mut world, maxy);
        assert_eq!(world, vec![
            Obstacle { points: vec![Point(502,9), Point(494,9)] },
            Obstacle { points: vec![Point(500,8)] },
            Obstacle { points: vec![Point(499,8)] },
        ]);
        drop_sand(&mut world, maxy);
        assert_eq!(world, vec![
            Obstacle { points: vec![Point(502,9), Point(494,9)] },
            Obstacle { points: vec![Point(500,8)] },
            Obstacle { points: vec![Point(499,8)] },
            Obstacle { points: vec![Point(501,8)] },
        ]);
    }

    #[test]
    fn it_drop_all_sand() {
        let (maxy, mut world) = read_world(inputs::INPUT);
        assert_eq!(drop_all_sand(&mut world, maxy), 674);
    }

}
