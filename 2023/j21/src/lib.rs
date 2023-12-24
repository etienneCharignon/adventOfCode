mod inputs;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Point(pub i64, pub i64);

pub fn read(input: &str) -> (Point, HashSet<Point>, Point) {
    let mut rocks = HashSet::<Point>::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut start = Point(0, 0);
    let size = Point(lines[0].chars().count() as i64, lines.len() as i64);
    for r in 0..lines.len() {
        let chars: Vec<char> = lines[r].chars().collect();
        for c in 0..chars.len() {
            if chars[c] == '#' {
                rocks.insert(Point(c as i64, r as i64));
            }
            else if chars[c] == 'S' {
                start = Point(c as i64, r as i64);
            }
        }
    }
    (start, rocks, size)
}

pub fn find_neighbors(plot: Point, rocks: &HashSet<Point>, size: &Point) -> Vec<Point> {
    let mut neighbors = Vec::<Point>::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == dy || dx == -dy {
                continue;
            }
            let x = plot.0 + dx;
            let y = plot.1 + dy;
            if x >= 0 && y >= 0 && x < size.0 && y < size.1 {
                let position = Point(x, y);
                if !rocks.contains(&position) {
                    neighbors.push(position);
                }
            }
        }
    }
    neighbors
}

pub fn print_plots(plots: &HashSet<Point>, rocks: &HashSet<Point>, size: &Point) {
    for r in 0..size.1 {
        let mut row = String::new();
        for c in 0..size.0 {
            let p = Point(c, r);
            if plots.contains(&p) {
                row.push('O');
            }
            else if rocks.contains(&p) {
                row.push('#');
            }
            else {
                row.push('.');
            }
        }
        println!("{row}");
    }
    println!("");
}

pub fn count_garden_plot(steps: usize, map: (Point, HashSet<Point>, Point)) -> usize {
    let mut garden_plots = HashSet::<Point>::new();
    garden_plots.insert(map.0);
    for _step in 0..steps {
        // print_plots(&garden_plots, &map.1, &map.2);
        let mut step_garden_plots = HashSet::<Point>::new();
        for plot in &garden_plots {
            let neighbors = find_neighbors(*plot, &map.1, &map.2);
            //println!("{plot:?}, {neighbors:?}");
            for neighbor in neighbors {
                step_garden_plots.insert(neighbor);
            }
        } 
        garden_plots = step_garden_plots;
    }
    // print_plots(&garden_plots, &map.1, &map.2);
    garden_plots.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_read_world() {
        let map = read(&inputs::EXAMPLE);
        print_plots(&HashSet::<Point>::new(), &map.1, &map.2);
        assert_eq!(0, 1);
    }

    #[test]
    fn it_count() {
        assert_eq!(count_garden_plot(6, read(inputs::EXAMPLE)), 16);
        assert_eq!(count_garden_plot(64, read(inputs::INPUT)), 3733);
    }
}
