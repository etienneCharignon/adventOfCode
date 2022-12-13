mod inputs;
use j12::read_map;
use j12::find_path;
use j12::length;
use j12::inputs::Point;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let data = read_map(inputs::INPUT);
    let map = data.0;
    let start = data.1;
    let end = data.2;
    let h = |p: Point| -> f32 {
        length(p, end)
    };
    let d = |c: Point, n: Point| -> f32 {
        let cost = ((map[n.1 as usize][n.0 as usize] - map[c.1 as usize][c.0 as usize]) + 1) as f32;
        if cost <= 2_f32 { 1_f32 } else { f32::MAX }
    };
    find_path(start, end, h, d).iter().count();
}