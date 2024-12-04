mod inputs;

pub fn read_input(input: &str) -> Vec<Vec<char>> {
    let mut read = vec![];
    let lines: Vec<&str> = input.split('\n').collect();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        read.push(chars);
    }
    read
}

pub fn get(input: &Vec<Vec<char>>, x: usize, y: usize, h: usize, w: usize, direction: (i32, i32), distance: i32) -> Option<char> {
    let new_x = x as i32 + (direction.0 * distance);
    let new_y = y as i32 + (direction.1 * distance);
    if new_x < 0 || new_x >= h as i32 { return None; }
    if new_y < 0 || new_y >= w as i32 { return None; }
    Some(input[new_x as usize][new_y as usize])
}

pub fn count_xmas(input: &Vec<Vec<char>>, x: usize, y: usize, w:usize, h:usize) -> usize {
    if input[x][y] == 'A' {
        for pattern in [('M', 'M', 'S', 'S'), ('M', 'S', 'S', 'M'), ('S', 'S', 'M', 'M'), ('S', 'M', 'M', 'S')] {
            if get(input, x, y, h, w, (-1, -1), 1) == Some(pattern.0) &&
               get(input, x, y, h, w, (-1, 1), 1) == Some(pattern.1) &&
               get(input, x, y, h, w, (1, 1), 1) == Some(pattern.2) &&
               get(input, x, y, h, w, (1, -1), 1) == Some(pattern.3) {
                   return 1;
            }
        }
    }
    0
}

pub fn search(input: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let height = input.len();
    let width = input[0].len();
    for row in 0..height {
        for col in 0..width {
            count += count_xmas(&input, col, row, width, height);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(search(read_input(inputs::EXAMPLE)), 9);
        assert_eq!(search(read_input(inputs::INPUT)), 1877);
    }
}
