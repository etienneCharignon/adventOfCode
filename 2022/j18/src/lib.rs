mod inputs;
#[derive(Debug, Copy, Clone, PartialEq)]
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
fn count_faces(cubes: Vec<Cube>) -> usize {
    let directions= vec![
        Cube(1, 0, 0),
        Cube(-1, 0, 0),
        Cube(0, 1, 0),
        Cube(0, -1, 0),
        Cube(0, 0, 1),
        Cube(0, 0, -1),
    ];
    cubes.clone().iter().map(|c| {
        directions.iter()
                  .map(|d| if cubes.contains(&add(*c, *d)) { 0 } else { 1 })
                  .sum::<usize>()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_faces() {
        assert_eq!(count_faces(read_input(inputs::EXAMPLE)), 64);
        assert_eq!(count_faces(read_input(inputs::INPUT)), 3500);
    }
}
