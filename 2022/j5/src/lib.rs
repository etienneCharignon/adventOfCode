use regex::Regex;

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
                let numbers = Regex::new(r"\d+").unwrap()
                                                .captures_iter(line)
                                                .map(|cap| cap.get(0).unwrap().parse::<usize>().unwrap())
                                                .collect();
                let nth = numbers[0];
                let from = numbers[1];
                let to = numbers[2];
                (0..nth).map(|_i| {
                            crates[from].pop().unwrap()
                        })
                        .collect::<Vec<char>>()
                        .iter()
                        .rev()
                        .for_each(|&c| {
                            crates[to].push(c);
                        });
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
        assert_eq!(moves(read_crates(inputs::EXAMPLE_CRATES, 3), inputs::EXAMPLE_MOVES),
                   vec!(vec!('M'), vec!('C'), vec!('P', 'Z', 'N', 'D')));
    }

    #[test]
    fn it_solve() {
        assert_eq!(solve(moves(read_crates(inputs::CRATES, 9), inputs::MOVES)), "BNTZFPMMW");
    }
}
