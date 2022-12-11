#[allow(dead_code)]
pub struct Monkey
{
    pub test: u64,
    pub dest: (usize, usize)
}

#[allow(dead_code)]
pub fn example() -> Vec<Monkey> {
    vec![
        Monkey { test: 23, dest: (2, 3) },
        Monkey { test: 19, dest: (2, 0) },
        Monkey { test: 13, dest: (1, 3) },
        Monkey { test: 17, dest: (0, 1) },
    ]
}

#[allow(dead_code)]
pub fn input() -> Vec<Monkey> {
    vec![
        Monkey { test: 2,  dest: (5, 2) },
        Monkey { test: 13, dest: (4, 3) },
        Monkey { test: 5,  dest: (5, 1) },
        Monkey { test: 3,  dest: (6, 7) },
        Monkey { test: 11, dest: (7, 3) },
        Monkey { test: 17, dest: (4, 1) },
        Monkey { test: 7,  dest: (0, 2) },
        Monkey { test: 19, dest: (6, 0) },
    ]
}

#[allow(dead_code)]
pub fn operate_example (monkey_number: u64, item: u64) -> u64 {
    match monkey_number {
        0 => { item * 19 },
        1 => { item + 6 },
        2 => { item * item },
        3 => { item + 3 },
        _ => panic!("unknown monkey number")
    }
}

#[allow(dead_code)]
pub fn operate_input (monkey_number: u64, item: u64) -> u64 {
    match monkey_number {
        0 => { item * 13 },
        1 => { item + 7 },
        2 => { item + 2 },
        3 => { item * 2 },
        4 => { item * item },
        5 => { item + 6 },
        6 => { item + 1 },
        7 => { item + 8 },
        _ => panic!("unknown monkey number")
    }
}