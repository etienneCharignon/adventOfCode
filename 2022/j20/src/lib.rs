mod inputs;

#[allow(unused_imports)]
use std::collections::HashSet;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Item(usize, i64);

#[allow(dead_code)]
fn read_input(input: &str) -> Vec<Item> {
    input.lines().enumerate().map(|(i, l)| Item(i, l.parse::<i64>().unwrap())).collect()
}

#[allow(dead_code)]
fn decrypt(input: Vec<Item>) -> Vec<Item> {
    input.iter().map(|i| Item(i.0, i.1 * 811589153)).collect()
}

fn move_one(file: &mut Vec<Item>, n: usize, file_length: i64) {
    let position = file.iter().position(|item| item.0 == n).unwrap();
    let item = file[position];
    if item.1 == 0 {
        return;
    }
    let mut new_pos = (position as i64 + item.1) % (file_length - 1);
    if new_pos < 0 {
        new_pos += file_length - 1;
    }
    let item = file.remove(position);
    file.insert(new_pos.try_into().unwrap(), item);
}

#[allow(dead_code)]
fn mix(file: &mut Vec<Item>) -> Vec<Item> {
    let file_length = file.len();
    for n in 0..file_length {
       // println!("{:?}", file.iter().map(|i| i.1).collect::<Vec<i64>>());
        move_one(file, n, file_length as i64);
    }
    //println!("{:?}", file.iter().map(|i| i.1).collect::<Vec<i64>>());
    file.to_vec()
}

#[allow(dead_code)]
fn mix10(file: &mut Vec<Item>) -> Vec<Item> {
    for _ in 0..10 {
        mix(file);
    }
    file.to_vec()
}

#[allow(dead_code)]
fn grove(file: &Vec<Item>) -> (usize, i64, i64, i64) {
    let zero_pos = file.iter().position(|item| item.1 == 0).unwrap();
    let file_length = file.len();
    (
        zero_pos,
        file[(zero_pos + 1000) % file_length].1,
        file[(zero_pos + 2000) % file_length].1,
        file[(zero_pos + 3000) % file_length].1
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_input() {
        assert_eq!(read_input(inputs::EXAMPLE), vec![
            Item(0, 1),
            Item(1, 2),
            Item(2, -3), 
            Item(3, 3),
            Item(4, -2),
            Item(5, 0),
            Item(6, 4),
            ]);
    }

    #[test]
    #[ignore]
    fn check_input_has_no_dup() {
        let hash: HashSet<Item> = HashSet::from_iter(read_input(inputs::INPUT));
        assert_eq!(inputs::INPUT.lines().count(), hash.iter().count());
    }

    #[test]
    fn it_move_one() {
        let mut input = read_input(inputs::EXAMPLE);
        move_one(&mut input, 0, 7);
        assert_eq!(input, vec![
            Item(1, 2),
            Item(0, 1),
            Item(2, -3), 
            Item(3, 3),
            Item(4, -2),
            Item(5, 0),
            Item(6, 4),
            ]);
    }

    #[test]
    fn it_move_negative() {
        let mut input = read_input(inputs::EXAMPLE);
        move_one(&mut input, 2, 7);
        assert_eq!(input, vec![
            Item(0, 1),
            Item(1, 2),
            Item(3, 3),
            Item(4, -2),
            Item(5, 0),
            Item(2, -3), 
            Item(6, 4),
            ]);
    }

    #[test]
    fn it_move_last_to_first() {
        let mut input = read_input("0\n1");
        move_one(&mut input, 1, 2);
        assert_eq!(input, vec![
            Item(1, 1),
            Item(0, 0),
            ]);
    }

    #[test]
    fn it_move_last_to_first_2() {
        let mut input = read_input("0\n1\n2\n3");
        move_one(&mut input, 2, 4);
        assert_eq!(input, vec![
            Item(0, 0),
            Item(2, 2),
            Item(1, 1),
            Item(3, 3),
            ]);
    }

    #[test]
    fn it_mix() {
        assert_eq!(mix(&mut read_input(inputs::EXAMPLE)), vec![
            Item(4, -2),
            Item(0, 1),
            Item(1, 2),
            Item(2, -3), 
            Item(6, 4),
            Item(5, 0),
            Item(3, 3),
            ]);
    }

    #[test]
    fn it_solve() {
        assert_eq!(grove(&mix10(&mut decrypt(read_input(inputs::EXAMPLE)))), (6, 811589153, 2434767459, -1623178306));
        assert_eq!(grove(&mix10(&mut decrypt(read_input(inputs::INPUT)))), (2166, -764516982126, 3812845840794, 5284256975183));
    }
}
