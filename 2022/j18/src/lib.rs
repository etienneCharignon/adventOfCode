mod inputs;
use std::collections::HashSet;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cube(i32, i32, i32);

#[allow(dead_code)]
fn read_input(input: &str) -> Vec<Cube> {
    input.lines().map(|l| { 
        let coordinates: Vec<i32> = l.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        Cube(coordinates[0], coordinates[1], coordinates[2])
    }).collect()
}


fn add(a: Cube, b: Cube) -> Cube {
    Cube(a.0+b.0, a.1+b.1, a.2 + b.2)
}

#[allow(dead_code)]
fn count_faces(cubes: Vec<Cube>) -> i32 {
    let directions= vec![
        Cube(1, 0, 0),
        Cube(-1, 0, 0),
        Cube(0, 1, 0),
        Cube(0, -1, 0),
        Cube(0, 0, 1),
        Cube(0, 0, -1),
    ];
    let mut air_bubbles: HashSet<Cube> = HashSet::new();
    for c in cubes.clone() {
        for d in directions.clone() {
            let neighbour = add(c, d);
            if !cubes.contains(&neighbour) && directions.clone().iter().all(|d| cubes.contains(&add(neighbour, *d))) { 
                air_bubbles.insert(neighbour);
            }
        }
    }
    println!("{:?}", air_bubbles);
    let faces: i32 = cubes.clone().iter().map(|c| {
        directions.iter()
                  .map(|d| if cubes.contains(&add(*c, *d)) { 0 } else { 1 })
                  .sum::<i32>()
    }).sum::<i32>();
    faces - (air_bubbles.len() as i32) * 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_faces() {
        assert_eq!(count_faces(read_input(inputs::EXAMPLE)), 58);
        assert_eq!(count_faces(read_input(inputs::INPUT)), 3500);
    }
}
