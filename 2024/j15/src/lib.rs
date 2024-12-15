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
    let mut field: Vec<Vec<char>> = input_field.split('\n').map(|line| line.chars().collect()).collect();
    for (r, row) in input_field.split('\n').enumerate() {
        for (c, cell) in row.chars().enumerate() {
            if cell == '@' {
                field[r][c] = '.';
                robot = Pos {x: c as i32, y: r as i32};
                break;
            }
        }
    }
    (field, robot)
}

pub fn move_robot(field: &mut Vec<Vec<char>>, robot: Pos, moves: char) -> Pos {
    let directions = [Pos{x:1, y:0},Pos{x:0, y:1}, Pos{x:-1, y:0}, Pos{x:0, y:-1}];
    let (d, d_reverse) = match moves {
        '<' => { (directions[2], directions[0]) }
        '>' => { (directions[0], directions[2]) }
        '^' => { (directions[3], directions[1]) }
        'v' => { (directions[1], directions[3]) }
        _ => { panic!("unkown direction") }
    };
    let mut cursor = robot;
    let mut boxes = vec![];
    cursor = add(cursor, d);
    while field[cursor.y as usize][cursor.x as usize] == 'O' {
        boxes.push(cursor);
        cursor = add(cursor, d);
    }
    if field[cursor.y as usize][cursor.x as usize] == '#' {
        return robot;
    }
    for _ in boxes {
        field[cursor.y as usize][cursor.x as usize] = 'O';
        cursor = add(cursor, d_reverse);
    }
    field[cursor.y as usize][cursor.x as usize] = '.';
    cursor
}

pub fn score_gps(field: &Vec<Vec<char>>) -> usize {
    field.iter().enumerate()
         .map(|(r, line)| line.iter().enumerate()
             .map(|(c, cell)| if *cell == 'O' { 100 * r + c } else { 0 })
             .sum::<usize>())
         .sum()
}

pub fn gps_boxes(field: &mut Vec<Vec<char>>, robot: Pos, moves: &str) -> usize {
    let mut current = robot;
    for line in moves.split('\n') {
        for a_move in line.chars() {
            current = move_robot(field, current, a_move);
        }
    }
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
        assert_eq!(move_robot(&mut field, robot, '<'), robot);
        (field, robot) = read("########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########");
        assert_eq!(move_robot(&mut field, robot, '^'), Pos {x: 2, y: 1});
        assert_eq!(field, read("########
#..O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########").0);
        (field, robot) = read("########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########");
        assert_eq!(move_robot(&mut field, robot, '>'), Pos {x: 3, y: 1});
        assert_eq!(field, read("########
#...OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########").0);
        (field, robot) = read("########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########");
        assert_eq!(move_robot(&mut field, robot, '>'), Pos {x: 4, y: 1});
        assert_eq!(field, read("########
#....OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########").0);
    }

    #[test]
    fn it_works() {
        let (mut field, mut robot) = read(inputs::EXAMPLE_FIELD);
        assert_eq!(gps_boxes(&mut field, robot, inputs::EXAMPLE_MOVES), 10092);
        (field, robot) = read(inputs::INPUT_FIELD);
        assert_eq!(gps_boxes(&mut field, robot, inputs::INPUT_MOVES), 10092);
    }
}
