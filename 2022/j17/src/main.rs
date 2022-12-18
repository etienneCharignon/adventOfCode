mod inputs;
use std::collections::HashSet;
use j17::fall_to;

fn main() {
    let jets = inputs::INPUT.chars().collect();
    let mut world: Vec<HashSet<i32>> = vec![];
    assert_eq!(fall_to(1000000000000, &mut world, jets), 1567723342929);
}