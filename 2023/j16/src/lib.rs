mod inputs;

use multimap::MultiMap;

pub fn read(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn move_head(head: ((i32, i32),(i32, i32))) -> (i32, i32) {
    (head.0.0 + head.1.0, head.0.1 + head.1.1)
}

pub fn count_energised(start: ((i32, i32),(i32, i32)), input: Vec<Vec<char>>) -> usize {
    let mut heads: Vec<((i32, i32),(i32, i32))> = vec![];
    let mut energised = MultiMap::<(i32, i32), (i32, i32)>::new();
    heads.push(start);
    while heads.len() > 0 {
        let head = heads.pop().unwrap();
        let new_pos = move_head(head);
        println!("{new_pos:?}");
        if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 == input[0].len() as i32 || new_pos.1 == input.len() as i32 {
            continue;
        }

        if energised.contains_key(&new_pos) {
            let directions = energised.get_vec(&new_pos).unwrap();
            if directions.contains(&head.1) {
                continue;
            }
        }
        energised.insert(new_pos, head.1);

        let (x, y) = (new_pos.0 as usize, new_pos.1 as usize);
        if input[y][x] == '.' {
            heads.push((new_pos, head.1));
        }
        else if input[y][x] == '\\' {
            println!("\\");
            let new_direction = match head.1 {
                (1, 0) => (0, 1),
                (-1, 0) => (0, -1),
                (0, 1) => (1, 0),
                (0, -1) => (-1, 0),
                _ => panic!("imposible direction")
            };
            heads.push((new_pos, new_direction));
        }
        else if input[y][x] == '/' {
            let new_direction = match head.1 {
                (1, 0) => (0, -1),
                (-1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (0, -1) => (1, 0),
                _ => panic!("imposible direction")
            };
            heads.push((new_pos, new_direction));
        }
        else if input[y][x] == '|' {
            if head.1 == (0, 1) || head.1 == (0, -1) {
                heads.push((new_pos, head.1));
            }
            else {
                heads.push((new_pos, (0, 1)));
                heads.push((new_pos, (0, -1)));
            }
        }
        else if input[y][x] == '-' {
            if head.1 == (1, 0) || head.1 == (-1, 0) {
                heads.push((new_pos, head.1));
            }
            else {
                heads.push((new_pos, (1, 0)));
                heads.push((new_pos, (-1, 0)));
            }
        }
    }
    energised.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve() {
        assert_eq!(count_energised(((-1, 0), (1, 0)), read(inputs::EXAMPLE)), 46);
        assert_eq!(count_energised(((-1, 0), (1, 0)), read(inputs::INPUT)), 7608);
    }
}
