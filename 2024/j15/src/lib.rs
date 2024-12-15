mod inputs;


#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub struct Pos {
   x: i32,
   y: i32
}

pub fn add(a: Pos, b: Pos) -> Pos {
    Pos {x: a.x + b.x, y: a.y + b.y }
}

pub fn read(input_field: &str) -> (Vec<Vec<char>>, Pos) {
    let mut robot = Pos {x:0, y:0};
    let mut field: Vec<Vec<char>> = vec![];
    for (r, line) in input_field.split('\n').enumerate() {
        let mut row = vec![];
        for (c, cell) in line.chars().enumerate() {
            match cell {
                '.'|'@' => {
                    row.push('.');
                    row.push('.');
                },
                'O' => {
                    row.push('[');
                    row.push(']');
                },
                '#' => {
                    row.push('#');
                    row.push('#');
                },
                _ => { panic!("unknown cell {}", cell) }
            }
            if cell == '@' {
                robot = Pos {x: c as i32 * 2, y: r as i32};
            }
        }
        field.push(row);
    }
    (field, robot)
}

pub fn print(field: &Vec<Vec<char>>, pos: Pos) -> String {
    let mut field_and_robot = field.clone();
    field_and_robot[pos.y as usize][pos.x as usize] = '@';
    let mut output = String::new();
    for line in field_and_robot {
        let str: String = line.iter().collect();
        output.push_str(&str);
        output.push_str("\n");
    }
    println!("{}", output);
    output
}

pub fn move_cell(field: &mut Vec<Vec<char>>, pos: Pos, direction: char, cell: char) -> Option<Pos> {
    let directions = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
    let d = match direction {
        '<' => { directions[2] },
        '>' => { directions[0] },
        '^' => { directions[3] },
        'v' => { directions[1] }
        _ => { panic!("unkown direction") }
    };
    let new_pos = add(pos, d);
    let dest_cell = field[new_pos.y as usize][new_pos.x as usize];
    match cell {
        '.' => { return Some(new_pos); }
        '@' => {
            match dest_cell {
                '#' => { return None; },
                '.' => { return Some(new_pos); }
                '['|']' => {
                    match move_cell(field, new_pos, direction, dest_cell) {
                        Some(_) => { return Some(new_pos); },
                        _ => { return None; }
                    }
                }
                _ => { panic!("unexpected dest_cell {dest_cell}"); }
            }
        },
        '['|']' => {
            match direction {
                '^'|'v' => {
                    let pair_direction = if cell == '[' { directions[0] } else { directions[2] };
                    let pair_new_pos = add(new_pos, pair_direction);
                    let mut pair_dest_cell = field[pair_new_pos.y as usize][pair_new_pos.x as usize];
                    if dest_cell == '#' || pair_dest_cell == '#' {
                        return None;
                    } else if dest_cell == '.' && pair_dest_cell == '.' {
                        let pair_pos = add(pos, pair_direction);
                        field[new_pos.y as usize][new_pos.x as usize] =  field[pos.y as usize][pos.x as usize];
                        field[pair_new_pos.y as usize][pair_new_pos.x as usize] = field[pair_pos.y as usize][pair_pos.x as usize];
                        field[pos.y as usize][pos.x as usize] = '.';
                        field[pair_pos.y as usize][pair_pos.x as usize] = '.';
                        return Some(new_pos);
                    } else {
                        match move_cell(&mut field.clone(), new_pos, direction, dest_cell) {
                            None => { return None; },
                            _ => { }
                        }
                        match move_cell(&mut field.clone(), pair_new_pos, direction, pair_dest_cell) {
                            None => { return None; },
                            _ => { }
                        }

                        move_cell(field, new_pos, direction, dest_cell);
                        pair_dest_cell = field[pair_new_pos.y as usize][pair_new_pos.x as usize];
                        move_cell(field, pair_new_pos, direction, pair_dest_cell);

                        let pair_pos = add(pos, pair_direction);
                        field[new_pos.y as usize][new_pos.x as usize] = field[pos.y as usize][pos.x as usize] ;
                        field[pair_new_pos.y as usize][pair_new_pos.x as usize] = field[pair_pos.y as usize][pair_pos.x as usize];
                        field[pos.y as usize][pos.x as usize] = '.';
                        field[pair_pos.y as usize][pair_pos.x as usize] = '.';
                        return Some(new_pos);
                    }
                },
                '>'|'<' => {
                    let pair_new_pos = add(new_pos, d);
                    let pair_dest_cell = field[pair_new_pos.y as usize][pair_new_pos.x as usize];
                    match pair_dest_cell {
                        '#' => { return None; },
                        '.' => {
                            field[pair_new_pos.y as usize][pair_new_pos.x as usize] = field[new_pos.y as usize][new_pos.x as usize];
                            field[new_pos.y as usize][new_pos.x as usize] = field[pos.y as usize][pos.x as usize];
                            field[pos.y as usize][pos.x as usize] = '.';
                            return Some(new_pos);
                        },
                        '['|']' => {
                            match move_cell(field, pair_new_pos, direction, pair_dest_cell) {
                                None => { return None; },
                                _ => {
                                    field[pair_new_pos.y as usize][pair_new_pos.x as usize] = field[new_pos.y as usize][new_pos.x as usize];
                                    field[new_pos.y as usize][new_pos.x as usize] = field[pos.y as usize][pos.x as usize];
                                    field[pos.y as usize][pos.x as usize] = '.';
                                    return Some(new_pos);
                                }
                            }
                        }
                        _ => {
                            print(field, pos);
                            panic!("pair dest cell impossible {pair_dest_cell}")
                        }
                    }
                }
                _ => {
                    print(field, pos);
                    panic!("impossible direction {direction}")
                }
            };
        },
        _ => { panic!("cannot move {cell}")}
    }
}

pub fn score_gps(field: &Vec<Vec<char>>) -> usize {
    field.iter().enumerate()
         .map(|(r, line)| line.iter().enumerate()
             .map(|(c, cell)| if *cell == '[' { 100 * r + c } else { 0 })
             .sum::<usize>())
         .sum()
}

pub fn gps_boxes(field: &mut Vec<Vec<char>>, robot: Pos, moves: &str) -> usize {
    let mut current = robot;
    print(field, current);
    for line in moves.split('\n') {
        for a_move in line.chars() {
            // println!("MOVE : {a_move}");
            match move_cell(field, current, a_move, '@') {
                Some(new_pos) => {
                    current = new_pos
                },
                _ => {}
            }
            // print(field, current);
        }
    }
    print(field, robot);
    score_gps(field)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_move_robot() {
        let (mut field, mut robot) = read("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########");
        assert_eq!(move_cell(&mut field, robot, '<', '@'), None);
        (field, robot) = read("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########");
        assert_eq!(move_cell(&mut field, robot, '^', '@'), Some(Pos {x: 4, y: 1}));
        assert_eq!(field, read("########
#..O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########").0);
        (field, _) = read("########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########");
        assert_eq!(move_cell(&mut field, Pos{x:5, y: 1}, '>', '@'), Some(Pos {x: 6, y: 1}));
        assert_eq!(move_cell(&mut field, Pos{x:6, y: 1}, '>', '@'), Some(Pos {x: 7, y: 1}));
        assert_eq!(field, read("########
#...OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########").0);
        (field, robot) = read("########
#......#
##O@O..#
#...OO.#
#.##..##
#......#
#......#
########");
        let mut new_pos = move_cell(&mut field, robot, '>', '@');
        new_pos = move_cell(&mut field, new_pos.unwrap(), '>', '@');
        new_pos = move_cell(&mut field, new_pos.unwrap(), '^', '@');
        new_pos = move_cell(&mut field, new_pos.unwrap(), '>', '@');
        new_pos = move_cell(&mut field, new_pos.unwrap(), 'v', '@');
        // assert_eq!(new_pos, None);
        // assert_eq!(new_pos, Some(Pos {x: 8, y: 3}));
        assert_eq!(print(&field, new_pos.unwrap()),
"################
##............##
####[]...@....##
##.......[]...##
##..####[][]####
##............##
##............##
################
");
    }

    #[test]
    fn it_score_gps() {
        let (field, _robot) = read("########
#.O....#");
        assert_eq!(score_gps(&field), 104);
    }

    #[test]
    fn it_works() {
        let (mut field, mut robot) = read(inputs::EXAMPLE_FIELD);
        assert_eq!(gps_boxes(&mut field, robot, inputs::EXAMPLE_MOVES), 9021);
        (field, robot) = read(inputs::INPUT_FIELD);
        assert_eq!(gps_boxes(&mut field, robot, inputs::INPUT_MOVES), 1535509); // 1544522 to0 hight
    }
}
