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
fn count_faces(start: Cube, cubes: Vec<Cube>) -> usize
{
    let directions= vec![
        Cube(1, 0, 0),
        Cube(-1, 0, 0),
        Cube(0, 1, 0),
        Cube(0, -1, 0),
        Cube(0, 0, 1),
        Cube(0, 0, -1),
    ];
    let mut visited = HashSet::from([start]);
    let mut opens = Vec::from([start]);
    let mut surfaces_count = 0;
    while !opens.is_empty() {
        let current = opens.pop().unwrap();
        for d in directions.clone() {
            let neighbour = add(current, d);
            if vec![neighbour.0, neighbour.1, neighbour.2].iter().any(|n| *n < -1 || *n > 20) || visited.contains(&neighbour) {
                continue;
            }
            if cubes.contains(&neighbour) {
                surfaces_count += 1;
            }
            else {
                opens.push(neighbour);
                visited.insert(neighbour);
            }
        }
    }
    surfaces_count
}  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_count_faces() {
        assert_eq!(count_faces(Cube(0, 0, 0), read_input(inputs::EXAMPLE)), 58);
        // 2041 is too low
        assert_eq!(count_faces(Cube(0, 0, 0), read_input(inputs::INPUT)), 2048);
    }
}
