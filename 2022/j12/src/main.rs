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
    let d = |a: Point, b: Point| -> f32 {
        let cost = ((map[a.1 as usize][a.0 as usize] - map[b.1 as usize][b.0 as usize]).abs() + 1) as f32;
        if cost <= 2_f32 { 1_f32 } else { f32::MAX }
    };
    find_path(start, end, h, d).iter().count();
}