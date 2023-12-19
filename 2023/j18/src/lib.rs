mod inputs;

use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use multimap::MultiMap;

lazy_static! {
    static ref COLORRE: Regex = Regex::new(r"\(#(.{5})(.)\)").unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Point(pub i64, pub i64);

pub fn print_digs(digs: &HashSet<Point>) {
    let x_from = digs.iter().map(|d| d.0).min().unwrap();
    let x_to = digs.iter().map(|d| d.0).max().unwrap();
    let y_from = digs.iter().map(|d| d.1).min().unwrap();
    let y_to = digs.iter().map(|d| d.1).max().unwrap();
    for y in y_from..=y_to {
        let mut row = String::new();
        for x in x_from..=x_to {
            let cell = digs.get(&Point(x, y));
            match cell {
                Some(_node) => {
                    row = row + &"#";
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

pub fn find_neighbors(node: Point) -> Vec<Point> {
    let mut neightbors = Vec::<Point>::new();
    neightbors.push( Point(node.0 - 1, node.1));
    neightbors.push( Point(node.0 + 1, node.1));
    neightbors.push( Point(node.0, node.1 - 1));
    neightbors.push( Point(node.0, node.1 + 1));

    neightbors.push( Point(node.0 + 1, node.1 + 1));
    neightbors.push( Point(node.0 - 1, node.1 + 1));
    neightbors.push( Point(node.0 + 1, node.1 - 1));
    neightbors.push( Point(node.0 - 1, node.1 - 1));
    neightbors
}

pub fn fill_digs(digs: &HashSet<Point>) -> HashSet<Point> {
    let mut all_digs: HashSet<Point> = HashSet::new();
    for node in digs.iter() {
        all_digs.insert(*node);
    }

    let x_from = digs.iter().map(|d| d.0).min().unwrap();
    let x_to = digs.iter().map(|d| d.0).max().unwrap();
    let y_from = digs.iter().map(|d| d.1).min().unwrap();
    let _y_to = digs.iter().map(|d| d.1).max().unwrap();
    let mut start_fill = Point(0,0);
    for x in x_from..=x_to {
        if digs.contains(&Point(x, y_from)) {
            start_fill = Point( x + 1, y_from + 1);
            break;
        }
    }
    // println!("{start_fill:?}");
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
    all_digs
}

pub fn count_inside(digs: &HashSet<Point>) -> i64 {
    let mut mmap = MultiMap::<i64, i64>::new();
    for dig in digs {
        mmap.insert(dig.1, dig.0);
    }
    let mut rows = Vec::from_iter(mmap.keys());
    rows.sort();
    let mut count = 0;
    let mut live_cells = 0;
    let mut last_row = 0;
    let mut last_cols = Vec::<i64>::new();
    // println!("{mmap:?} {rows:?}");
    for r in rows {
        count += live_cells * (r-last_row).abs();
        // println!("{live_cells} * ({r} - {last_row}) = {}", live_cells * (r-last_row).abs());

        let mut corners = mmap.get_vec(r).unwrap().clone();
        corners.sort();
        let existing_cols = last_cols.clone();
        let closing_corners: Vec<i64>  = corners.iter().filter(|c| last_cols.contains(c)).map(|c| *c).collect();
        for corner in corners.clone() {
            if ! last_cols.contains(&corner) {
                last_cols.push(corner);
            }
        }
        last_cols.sort();

        let mut new_columns = Vec::<i64>::new();
        for col in last_cols {
            if ! closing_corners.contains(&col) {
                new_columns.push(col);
            }
        }
        last_cols = new_columns;

        let cr = count_reduction(&existing_cols, &corners, &last_cols);
        count += cr;

        /*
        if live_cells>0 {
            // println!("{},{}", live_cells, cr);
            for i in 1..((r-last_row).abs()) {
                println!("{live_cells},0");
            }
        }
        */

        live_cells = 0;
        for chunk in last_cols.chunks(2) {
            live_cells += chunk[1] - chunk[0] + 1;
        }
        // println!("{last_cols:?} {live_cells}");
        last_row = *r;
    }
    count += live_cells;
    count
}

pub fn count_reduction(existing_cols: &Vec<i64>, corners: &Vec<i64>, all_cols: &Vec<i64>) -> i64 {
    let mut count = 0;
    for pair in corners.chunks(2) {
        if existing_cols.contains(&pair[0]) {
            let index = existing_cols.iter().position(|c| *c == pair[0]).unwrap();
            if existing_cols.contains(&pair[1]) {
                if index % 2 == 0 {
                    count += pair[1] - pair[0] + 1;
                }
            }
            else if index % 2 == 0 {
                count += pair[1] - pair[0];
            }
        }
        else if existing_cols.contains(&pair[1]) {
            let index = existing_cols.iter().position(|c| *c == pair[1]).unwrap();
            if index % 2 != 0 {
                count += pair[1] - pair[0];
            }
        }
        else {
            let index = all_cols.iter().position(|c| *c == pair[0]).unwrap();
            if index % 2 != 0 {
                count += pair[1] - pair[0] - 1;
            }
        }
    } 
    // println!("count_reduction existing {existing_cols:?}  corners {corners:?} cr: {count}");
    count
}

pub fn count_dig(input: &str) -> i64 {
    let mut digs = HashSet::<Point>::new();
    let mut current = Point(0i64, 0i64);
    for line in input.lines() {
        let mut splits = line.split_whitespace();
        let olddirection = splits.next().unwrap();
        let olddistance = splits.next().unwrap().parse::<i64>().unwrap();
        let (_, colors): (&str, [&str; 2]) = COLORRE.captures(splits.next().unwrap()).unwrap().extract();
        // println!("{colors:?}");
        let distance = i64::from_str_radix(colors[0], 16).unwrap();
        let direction = match colors[1].parse::<i64>().unwrap() {
            0 => "R",
            1 => "D",
            2 => "L",
            _ => "U",
        };

        let sx = current.0;
        let sy = current.1;
        match direction {
            "U" => { // "U"
                current = Point(sx, sy - distance);
                digs.insert(current);
            },
            "D" => { // "D"
                current = Point(sx, sy + distance);
                digs.insert(current);
            },
            "L" => { // "L"
                current = Point(sx - distance, sy);
                digs.insert(current);
            },
            "R" => { // "R"
                current = Point(sx + distance, sy);
                digs.insert(current);
            },
            _ => {
                panic!("Un recognized direction");
            }
        };
    }
    //print_digs(&digs);
    count_inside(&digs)
    //digs = fill_digs(&digs);
    //print_digs(&digs);
    //digs.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_reductions() {
        assert_eq!(count_reduction(&vec![], &vec![1, 6], &vec![0, 1, 6]), 4);
        assert_eq!(count_reduction(&vec![], &vec![0, 6], &vec![0, 6]), 0);
        assert_eq!(count_reduction(&vec![0, 6], &vec![0, 2], &vec![]), 2);
        assert_eq!(count_reduction(&vec![0, 6, 9, 10], &vec![6, 7], &vec![]), 0);
        assert_eq!(count_reduction(&vec![0, 6, 9, 10], &vec![8, 9], &vec![]), 0);
        assert_eq!(count_reduction(&vec![0, 6, 9, 10], &vec![6, 9], &vec![]), 0);
        assert_eq!(count_reduction(&vec![0, 6, 9, 10], &vec![0, 6], &vec![]), 7);
        assert_eq!(count_reduction(&vec![0, 6, 9, 10], &vec![0, 6, 9, 10], &vec![]), 9);
        assert_eq!(count_reduction(&vec![0, 6], &vec![4, 6], &vec![]), 2);
        assert_eq!(count_reduction(&vec![0, 6], &vec![0, 6], &vec![]), 7);
    }

    #[test]
    fn it_works() {
        assert_eq!(count_dig(inputs::EXAMPLE), 952408144115); // 952408144115);
        assert_eq!(count_dig(inputs::INPUT), 48020869073824);
    }
}
