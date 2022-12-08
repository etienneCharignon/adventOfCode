mod inputs;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for lines in input.lines() {
        result.push(lines.chars().map(|c| String::from(c).parse::<usize>().unwrap()).collect());
    }
    result
}

fn is_visible_left(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    let mut visible = true;
    for i in 0..c {
        visible = visible && forest[r][i] < forest[r][c];
    }
    visible
}

fn is_visible_right(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    let mut visible = true;
    for i in c+1..forest[r].len() {
        visible = visible && forest[r][i] < forest[r][c];
    }
    visible
}

fn is_visible_up(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    let mut visible = true;
    for i in 0..r {
        visible = visible && forest[i][c] < forest[r][c]
    }
    visible
}

fn is_visible_down(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    let mut visible = true;
    for i in r+1..forest.len() {
        visible = visible && forest[i][c] < forest[r][c]
    }
    visible
}

fn is_visible(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    is_visible_left(forest, r, c) ||
    is_visible_right(forest, r, c) ||
    is_visible_up(forest, r, c) ||
    is_visible_down(forest, r, c)
}

fn count_visible(forest: Vec<Vec<usize>>) -> usize {
    let mut visible: usize = 0;
    let size: usize = forest.len() - 1;
    visible += size * 4;
    for r in 1..size {
        for c in 1..size  {
            if is_visible(&forest, r, c)  {
                visible += 1;
            }
        }
    }
    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parse_input() {
        assert_eq!(parse_input(inputs::EXAMPLE), vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
    }

    #[test]
    fn it_check_if_visible_tree() {
        assert!(is_visible(&parse_input(inputs::EXAMPLE), 1, 1));
        assert!(is_visible(&vec![
            vec![0, 1, 0],
            vec![0, 1, 1],
            vec![0, 1, 0],
        ], 1, 1));
        assert!(is_visible(&vec![
            vec![0, 0, 0],
            vec![1, 1, 1],
            vec![0, 1, 0],
        ], 1, 1));
        assert!(is_visible(&vec![
            vec![0, 1, 0],
            vec![1, 1, 0],
            vec![0, 1, 0],
        ], 1, 1));
        assert!(is_visible(&vec![
            vec![0, 1, 0],
            vec![1, 1, 1],
            vec![0, 0, 0],
        ], 1, 1));
        assert!(!is_visible(&vec![
            vec![0, 0, 1, 0],
            vec![1, 0, 1, 1],
            vec![0, 0, 1, 0],
        ], 1, 2));
        assert!(!is_visible(&vec![
            vec![0, 1, 1, 0],
            vec![1, 1, 0, 1],
            vec![0, 1, 1, 0],
        ], 1, 1));
    }

    #[test]
    fn it_count_visible_tree() {
        assert_eq!(count_visible(parse_input(inputs::EXAMPLE)), 21);
        assert_eq!(count_visible(parse_input(inputs::INPUT)), 1843);
    }

}
