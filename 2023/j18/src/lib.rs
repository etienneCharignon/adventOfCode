mod inputs;

use std::collections::HashSet;
use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use std::hash::{Hash, Hasher};

lazy_static! {
    static ref COLORRE: Regex = Regex::new(r"\(#(.{2})(.{2})(.{2})\)").unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Point(pub i64, pub i64);

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub p: Point,
    pub color: (u8, u8, u8),
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.p.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Eq for Node { }

pub fn print_digs(digs: &HashSet<Node>) {
    let x_from = digs.iter().map(|d| d.p.0).min().unwrap();
    let x_to = digs.iter().map(|d| d.p.0).max().unwrap();
    let y_from = digs.iter().map(|d| d.p.1).min().unwrap();
    let y_to = digs.iter().map(|d| d.p.1).max().unwrap();
    for y in y_from..=y_to {
        let mut row = String::new();
        for x in x_from..=x_to {
            let cell = digs.get(&Node { p: Point(x, y), color: (0,0,0) });
            match cell {
                Some(node) => {
                    row = row + &"#".truecolor(node.color.0, node.color.1, node.color.2);
                }
                None => {
                    row.push('.');
                }
            };
        }
        println!("{row}");
    }
    println!("");
}

pub fn find_neighbors(node: Node) -> Vec<Node> {
    let mut neightbors = Vec::<Node>::new();
    neightbors.push(Node { p: Point(node.p.0 - 1, node.p.1), color: (0,0,0)});
    neightbors.push(Node { p: Point(node.p.0 + 1, node.p.1), color: (0,0,0)});
    neightbors.push(Node { p: Point(node.p.0, node.p.1 - 1), color: (0,0,0)});
    neightbors.push(Node { p: Point(node.p.0, node.p.1 + 1), color: (0,0,0)});

    neightbors.push(Node { p: Point(node.p.0 + 1, node.p.1 + 1), color: (0,0,0)});
    neightbors.push(Node { p: Point(node.p.0 - 1, node.p.1 + 1), color: (0,0,0)});
    neightbors.push(Node { p: Point(node.p.0 + 1, node.p.1 - 1), color: (0,0,0)});
    neightbors.push(Node { p: Point(node.p.0 - 1, node.p.1 - 1), color: (0,0,0)});
    neightbors
}

pub fn fill_digs(digs: &HashSet<Node>) -> HashSet<Node> {
    let mut all_digs: HashSet<Node> = HashSet::new();
    for node in digs.iter() {
        all_digs.insert(*node);
    }

    let x_from = digs.iter().map(|d| d.p.0).min().unwrap();
    let x_to = digs.iter().map(|d| d.p.0).max().unwrap();
    let y_from = digs.iter().map(|d| d.p.1).min().unwrap();
    let y_to = digs.iter().map(|d| d.p.1).max().unwrap();
    let mut start_fill = Node { p: Point(0,0), color: (0,0,0)};
    for x in x_from..=x_to {
        if digs.contains(&Node { p: Point(x, y_from), color: (0,0,0) }) {
            start_fill = Node { p: Point( x + 1, y_from + 1), color: (0,0,0)};
            break;
        }
    }
    println!("{start_fill:?}");
    all_digs.insert(start_fill);
    print_digs(&all_digs);
    let mut opens = vec![start_fill];
    while !opens.is_empty() {
        let current = opens.pop().unwrap();
        for neightbor in find_neighbors(current) {
            if !all_digs.contains(&neightbor) {
                all_digs.insert(neightbor);
                opens.push(neightbor)
            }
        }
    }
    /*
    for y in y_from..=y_to {
        let mut inside = false;
        let mut is_on_limit = false;
        for x in x_from..=x_to {
            let cell = digs.get(&Node { p: Point(x, y), color: (0,0,0) });
            if !inside && cell != None {
                inside = true;
                is_on_limit = true;
            }
            if inside && !is_on_limit && cell != None {
                inside = false;
            }
            if cell != None {
                all_digs.insert(*cell.unwrap());
            }
            else if inside {
                all_digs.insert(Node { p: Point(x, y), color: (0,0,0) });
            }
            if cell == None {
                is_on_limit = false;
            }
        }
    }
    */
    all_digs
}

pub fn count_dig(input: &str) -> i64 {
    let mut digs = HashSet::<Node>::new();
    let mut current = Node { p: Point(0i64, 0i64), color: (0,0,0) };
    for line in input.lines() {
        let mut splits = line.split_whitespace();
        let direction = splits.next().unwrap();
        let distance = splits.next().unwrap().parse::<i64>().unwrap();
        let (_, colors): (&str, [&str; 3]) = COLORRE.captures(splits.next().unwrap()).unwrap().extract();
        let rgb: Vec<_> = colors.iter().map(|c| u8::from_str_radix(c, 16).unwrap()).collect();

        let sx = current.p.0;
        let sy = current.p.1;
        match direction {
            "U" => {
                for y in ((sy - distance)..sy).rev() {
                    current = Node { p: Point(sx, y), color: (rgb[0], rgb[1], rgb[2]) };
                    digs.insert(current);
                }
            },
            "D" => {
                for y in sy..(sy + distance) {
                    current = Node { p: Point(sx, y + 1), color: (rgb[0], rgb[1], rgb[2]) };
                    digs.insert(current);
                }
            },
            "L" => {
                for x in ((sx - distance)..sx).rev() {
                    current = Node { p: Point(x, sy), color: (rgb[0], rgb[1], rgb[2]) };
                    digs.insert(current);
                }
            },
            "R" => {
                for x in sx..(sx + distance) {
                    current = Node { p: Point(x + 1, sy), color: (rgb[0], rgb[1], rgb[2]) };
                    digs.insert(current);
                }
            },
            _ => {
                panic!("Un recognized direction");
            }
        };
    }
    print_digs(&digs);
    digs = fill_digs(&digs);
    print_digs(&digs);
    digs.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(count_dig(inputs::EXAMPLE), 62);
        assert_eq!(count_dig(inputs::INPUT), 35401);
    }
}
