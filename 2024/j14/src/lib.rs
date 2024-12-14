mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref RE: Regex = Regex::new(r"([-0-9]+)").unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Vector {
   x: i64,
   y: i64
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Robot {
   p: Vector,
   v: Vector
}

pub fn read(input: &str) -> Vec<Robot> {
    let mut robots = vec![];

    let mut matches = RE.find_iter(input)
        .map(|m| m.as_str().parse::<i64>().unwrap());

    while let (Some(px), Some(py), Some(vx), Some(vy)) = (
        matches.next(),
        matches.next(),
        matches.next(),
        matches.next()
    ) {
        // println!("Position: ({}, {}), Velocity: ({}, {})", px, py, vx, vy);

        robots.push(Robot{
            p: Vector { x: px, y: py },
            v: Vector { x: vx, y: vy }
        })
    }
    println!("nombre de robot : {}", robots.len());
    robots
}

pub fn move_robot(robot: &Robot, width: i64, height: i64, n: i64, slowDown: i64) -> Robot {
    let mut new_robot = Robot {
        p: Vector {
            x: (robot.p.x + n * robot.v.x / slowDown) % width,
            y: (robot.p.y + n * robot.v.y / slowDown) % height,
        },
        v: robot.v
    };
    if new_robot.p.x < 0 {
        new_robot.p.x += new_robot.p.x / width + width;
    }
    if new_robot.p.y < 0 {
        new_robot.p.y += new_robot.p.y / height + height;
    }
    new_robot
}

pub fn move_robots(robots: &Vec<Robot>, width: i64, height: i64, n: i64, slowDown: i64) -> Vec<Robot> {
    let mut new_pos = vec![];
    for robot in robots {
        new_pos.push(move_robot(robot, width, height, n, slowDown));
    }
    new_pos
}

pub fn in_triangle(robots: &Vec<Robot>, width: i64, _height: i64) -> i64 {
    let mut in_triangle = 0;
    for robot in robots {
        if robot.p.x >= width / 2 - robot.p.y && robot.p.x <= width / 2 + robot.p.y {
            in_triangle += 1;
        }
    }
    in_triangle
}

pub fn affiche(robots: &Vec<Robot>, width: i64, height: i64, n: i64) -> Option<i64> {
    let factor = in_triangle(&robots, width, height);
    if factor < 460 {
        return None;
    }
    println!("factor : {factor}");
    println!("iteration {n}");

    let mut world: Vec<Vec<char>> = vec![];
    for _ in 0..height {
        world.push(vec![' '; width as usize])
    }
    for robot in robots {
        world[robot.p.y as usize][robot.p.x as usize] = '*';
    }
    for row in world {
        let str: String = row.iter().collect();
        println!("{}", str);
    }
    Some(n)
}

pub fn safety_factor(robots: &Vec<Robot>, width: i64, height: i64) -> i64 {
    let mut quadrants = [0; 4];
    for robot in robots {
        if robot.p.x < width / 2 {
            if robot.p.y < height / 2 {
                quadrants[0] += 1;
            }
            else if robot.p.y > height / 2{
                quadrants[1] += 1
            }
        }
        else if robot.p.x > width / 2 {
            if robot.p.y < height / 2 {
                quadrants[2] += 1;
            }
            else if robot.p.y > height / 2{
                quadrants[3] += 1
            }
        }
    }
    //println!("{:?}",  quadrants);
    quadrants.iter().product()
}

pub fn affiche_n(robots: Vec<Robot>, width: i64, height: i64, n: i64) -> i64 {
    let mut slowDown = 100;
    for i in 0..n {
        let n_positions = move_robots(&robots, width, height, i, slowDown);
        match affiche(&n_positions, width, height, i) {
            Some(_) => {
                slowDown = 100;
                thread::sleep(Duration::from_secs(1));
            },
            _ => {}
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_move_robot() {
        assert_eq!(move_robot(&Robot { p: Vector { x: 2, y: 6 }, v: Vector { x: 2, y: -3 } }, 11, 7, 1, 1),
                              Robot { p: Vector { x: 4, y: 3 }, v: Vector { x: 2, y: -3 } });
        assert_eq!(move_robot(&Robot { p: Vector { x: 10, y: 6 }, v: Vector { x: 1, y: 1 } }, 11, 7, 1, 1),
                              Robot { p: Vector { x: 0, y: 0 }, v: Vector { x: 1, y: 1 } });
        assert_eq!(move_robot(&Robot { p: Vector { x: 0, y: 0 }, v: Vector { x: -2, y: -2 } }, 11, 7, 1, 1),
                              Robot { p: Vector { x: 9, y: 5 }, v: Vector { x: -2, y: -2 } });
        assert_eq!(move_robot(&Robot { p: Vector { x: 0, y: 0 }, v: Vector { x: -1, y: -1 } }, 11, 7, 1, 1),
                              Robot { p: Vector { x: 10, y: 6 }, v: Vector { x: -1, y: -1 } });
    }

    #[test]
    fn it_works() {
        assert_eq!(affiche_n(read(inputs::INPUT), 101, 103, 1000000), 7584);
    }
}
