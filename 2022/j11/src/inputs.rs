struct Monkey<'a>
{
    items: &'a mut Vec<usize>,
    operation: &'a dyn Fn(usize) -> usize,
    test: usize,
    dest: (u8, u8)
}

impl<'a> Monkey<'a>
{
    fn example() -> &'a mut Vec<Monkey<'a>> {
        &mut vec![
            Monkey {
                items: &mut vec![79, 98],
                operation: &|old| { old * 19 },
                test: 23,
                dest: (2, 3)
            },
            Monkey {
                items: &mut vec![54, 65, 75, 74],
                operation: &|old| { old + 6 },
                test: 19,
                dest: (2, 0)
            },
            Monkey {
                items: &mut vec![79, 60, 97],
                operation: &|old| { old * old },
                test: 13,
                dest: (1, 3)
            },
            Monkey {
                items: &mut vec![74],
                operation: &|old| { old + 3 },
                test: 17,
                dest: (0, 1)
            },
        ]
    }
}