mod inputs;
use std::collections::HashSet;

pub fn read(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn find_start(world: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for row in 0..world.iter().count() {
        for col in 0..world[row].iter().count() {
            if world[row][col] == 'S' {
                return Some((col, row));
            }
        }
    }
    None
}

pub fn find_neighbour(p: (usize, usize), tile: char) -> ((usize, usize), (usize, usize)) {
    match tile {
        '|' => ((p.0, p.1 - 1),(p.0, p.1 + 1)),
        '-' => ((p.0 - 1, p.1),(p.0 + 1, p.1)),
        'L' => ((p.0, p.1 - 1),(p.0 + 1, p.1)),
        'J' => ((p.0, p.1 - 1),(p.0 - 1, p.1)),
        '7' => ((p.0 - 1, p.1),(p.0, p.1 + 1)),
        'F' => ((p.0 + 1, p.1),(p.0, p.1 + 1)),
        _ => panic!("Tu ne devrais pas être sur un tuyeau inconnu")
    }
}

pub fn print_path(path: &HashSet<(usize, usize)>, width: i64, height: i64) {
    println!("================PATH==================");
    for r in 0..height as usize {
        let mut row = Vec::<char>::new();
        for c in 0..width as usize {
            if path.contains(&(c, r)) {
                row.push('*');
            }
            else {
                row.push('.');
            }
        }
        println!("{}", String::from_iter(row));
    }
}

pub fn find_path(start_tile: char, world: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let height: i64 = world.iter().count().try_into().unwrap();
    let width: i64 = world.iter().next().unwrap().len().try_into().unwrap();
    let start = find_start(world).unwrap();
    let mut path = HashSet::<(usize, usize)>::new();
    let mut open_set = HashSet::<(usize,usize)>::new();
    let mut neighbours = find_neighbour(start, start_tile);
    path.insert(start);
    open_set.insert(neighbours.0);
    open_set.insert(neighbours.1);
    loop {
        let current = match open_set.iter().next() {
            Some(point) => *point,
            None => break
        };
        open_set.remove(&current);
        path.insert(current);
        neighbours = find_neighbour(current, world[current.1][current.0]);
        if !path.contains(&neighbours.0) { open_set.insert(neighbours.0); }
        if !path.contains(&neighbours.1) { open_set.insert(neighbours.1); }
    }
    print_path(&path, width, height);
    path
}

pub fn solve_p1(start_tile: char, world: &Vec<Vec<char>>) -> usize {
    find_path(start_tile, world).iter().count() / 2
}

pub fn solve_p2(start_tile: char, world: &Vec<Vec<char>>) -> i64 {
    let height: i64 = world.iter().count().try_into().unwrap();
    let width: i64 = world.iter().next().unwrap().len().try_into().unwrap();
    let start = find_start(world).unwrap();

    let path = find_path(start_tile, world);
    let mut count_inside = 0;

    let mut is_inside = false;
    let mut is_on_path = false;

    let mut last_turn:char = ' ';

    for r in 0..height as usize {
        for c in 0..width as usize {
            let current_tile = if (c,r) == start { start_tile } else { world[r][c] };
            if path.contains(&(c, r)) {
                if current_tile == '|' {
                    is_inside = ! is_inside;
                }
                else if current_tile == '-' {

                }
                else {
                    if !is_on_path {
                        last_turn = current_tile;
                        is_on_path = true;
                    }
                    else {
                        if (last_turn == 'L' && current_tile == '7') || 
                            (last_turn == 'F' && current_tile == 'J')
                         {
                            is_inside = ! is_inside; 
                        }
                        is_on_path = false;
                    }
                }
            }
            else {
                is_on_path = false;
                if is_inside {
                    println!("{:?}", (c,r));
                    count_inside += 1;
                }
            }
        }
    }
    count_inside
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve_example() {
        assert_eq!(solve_p1('F', &read(inputs::EXAMPLE1)), 4);
    }

    #[test]
    fn it_solve_example_p2() {
        assert_eq!(solve_p2('F', &read(inputs::EXAMPLE2)), 4);
    }

    #[test]
    fn it_solve_example3_p2() {
        assert_eq!(solve_p2('F', &read(inputs::EXAMPLE3)), 8);
    }

    #[test]
    fn it_solve_example4_p2() {
        assert_eq!(solve_p2('7', &read(inputs::EXAMPLE4)), 10);
    }

    #[test]
    #[ignore]
    fn it_solve_input() {
        assert_eq!(solve_p1('|', &read(inputs::INPUT)), 7086);
    }

    #[test]
    fn it_solve_p2_input() {
        assert_eq!(solve_p2('|', &read(inputs::INPUT)), 317); //1645 to high
    }
}
