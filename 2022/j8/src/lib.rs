mod inputs;

use std::cmp;

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for lines in input.lines() {
        result.push(lines.chars().map(|c| String::from(c).parse::<usize>().unwrap()).collect());
    }
    result
}

fn is_visible_left(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    (0..c).map(|i| forest[r][i])
          .all(|cell| cell < forest[r][c])
}

fn is_visible_right(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    (c+1..forest[r].len()).map(|i| forest[r][i])
                          .all(|cell| cell < forest[r][c])
}

fn is_visible_up(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    (0..r).map(|i| forest[i][c])
          .all(|cell| cell < forest[r][c])
}

fn is_visible_down(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> bool {
    (r+1..forest.len()).map(|i| forest[i][c])
                       .all(|cell| cell < forest[r][c])
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

fn scope_left(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> usize {
    let mut scope = 0;
    for i in (0..c).rev() {
        scope += 1;
        if forest[r][i] >= forest[r][c]{
            break;
        }
    }
    scope
}

fn scope_up(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> usize {
    let mut scope = 0;
    for i in (0..r).rev() {
        scope += 1;
        if forest[i][c] >= forest[r][c]{
            break;
        }
    }
    scope
}

fn scope_down(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> usize {
    let mut scope = 0;
    for i in r+1..forest.len() {
        scope += 1;
        if forest[i][c] >= forest[r][c]{
            break;
        }
    }
    scope
}

fn scope_right(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> usize {
    let mut scope = 0;
    for i in c+1..forest[r].len() {
        scope += 1;
        if forest[r][i] >= forest[r][c]{
            break;
        }
    }
    scope
}

fn scope(forest: &Vec<Vec<usize>>, r: usize, c: usize) -> usize {
    scope_up(forest, r, c) * scope_left(forest, r, c) * scope_down(forest, r, c) * scope_right(forest, r, c)
}

fn higest_scope(forest: Vec<Vec<usize>>) -> usize {
    let mut higest: usize = 0;
    let size: usize = forest.len() - 1;
    for r in 1..size {
        for c in 1..size  {
            higest = cmp::max(higest, scope(&forest, r, c));
        }
    }
    higest
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
    
    #[test]
    fn it_find_highest_scenic_score() {
        assert_eq!(higest_scope(parse_input(inputs::EXAMPLE)), 8);
        assert_eq!(higest_scope(parse_input(inputs::INPUT)), 180000);
    }


}
