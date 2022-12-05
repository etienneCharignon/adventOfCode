mod inputs;

#[cfg(test)]
mod tests {
    use super::*;

    fn read_crates(crates: &str, n: usize) -> Vec<Vec<char>> {
        let mut result = Vec::new();
        (0..n).for_each(|_column| {result.push(Vec::new())});
        crates.lines()
              .rev()
              .for_each(|line| {
                (0..n).for_each(|column| {
                    let c = line.chars().nth(column * 4 + 1).unwrap();
                    if c != ' ' {
                        result[column].push(c)
                    }
                })
              });
        result
    }

    fn moves(mut crates: Vec<Vec<char>>, moves: &str) -> Vec<Vec<char>>  {
        moves.lines()
             .for_each(|line| {
                let words:Vec<&str> = line.split(' ').collect();
                let nth = words[1].parse::<usize>().unwrap();
                let from = words[3].parse::<usize>().unwrap() - 1;
                let to = words[5].parse::<usize>().unwrap() - 1;
                (0..nth).for_each(|_i| {
                    let c = crates[from].pop().unwrap();
                    crates[to].push(c);
                })
             });
        crates
    }

    fn solve(crates: Vec<Vec<char>>) -> String {
        crates.iter()
              .map(|column| { column.last().unwrap() })
              .collect()
    }

    #[test]
    fn it_read_crates() {
        assert_eq!(read_crates(inputs::EXAMPLE_CRATES, 3), vec!(vec!('Z','N'), vec!('M','C','D'), vec!('P')));
    }

    #[test]
    fn it_moves() {
        assert_eq!(moves(read_crates(inputs::EXAMPLE_CRATES, 3), inputs::EXAMPLE_MOVES), vec!(vec!('C'), vec!('M'), vec!('P', 'D', 'N', 'Z')));
    }

    #[test]
    fn it_solve() {
        assert_eq!(solve(moves(read_crates(inputs::CRATES, 9), inputs::MOVES)), "PSNRGBTFT");
    }
}
