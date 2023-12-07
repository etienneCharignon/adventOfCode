mod inputs;

use std::collections::HashMap;
use std::cmp::Ordering;

pub fn card(c: char) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => (c.to_string()).parse::<i32>().unwrap()
    }
}

pub fn type_hand(hand: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut number_of_j =  0;
    for c in hand {
        if *c == 1 {
            number_of_j += 1;
            continue;
        }
        if map.contains_key(c) {
            map.insert(*c, map[c] + 1);
        }
        else {
            map.insert(*c, 1);
        }
    }
    println!("{:?}", map);
    let mut duplicates: Vec<&i32> = map.values().collect();
    duplicates.sort_by(|a, b| b.cmp(a));
    println!("{:?}", duplicates);
    let first = if duplicates.len() > 0 { *duplicates[0] + number_of_j } else { number_of_j };
    println!("{:?}", first);
    if first == 5 {
        6
    }
    else if first == 4 {
        5
    }
    else if first == 3 && *duplicates[1] == 2 {
        4
    }
    else if first == 3 {
        3
    }
    else if first == 2 && *duplicates[1] == 2 {
        2
    }
    else if first == 2 {
        1
    }
    else { 0 }
}

pub fn compare(h1: &Vec<i32>, h2:&Vec<i32>) -> Ordering {
    let type_order = type_hand(h1).cmp(&type_hand(h2));
    if type_order != Ordering::Equal {
        return type_order;
    }
    for i in 0..5 {
        let order = h1[i].cmp(&h2[i]);
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

pub fn parse_hand(hand: &str) -> Vec<i32> {
    hand.chars().map(|c| card(c)).collect::<Vec<i32>>()
} 

pub fn winings(input: &str) -> usize {
    let mut world: Vec<_> = input.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .map(|pair| (parse_hand(pair[0]), pair[1].parse::<usize>().unwrap()))
        .collect();
    world.sort_by(|p1, p2| compare(&p1.0, &p2.0));
    println!("{:?}", world);
    world.iter().enumerate().map(|(i, hand)| (i + 1) * hand.1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_read_type_hand() {
        assert_eq!(type_hand(&parse_hand("T55J5")), 5);
        assert_eq!(type_hand(&parse_hand("22222")), 6);
        assert_eq!(type_hand(&parse_hand("22227")), 5);
        assert_eq!(type_hand(&parse_hand("22277")), 4);
        assert_eq!(type_hand(&parse_hand("22278")), 3);
        assert_eq!(type_hand(&parse_hand("22677")), 2);
        assert_eq!(type_hand(&parse_hand("22677")), 2);
        assert_eq!(type_hand(&parse_hand("32T3K")), 1);
        assert_eq!(type_hand(&parse_hand("23456")), 0);
    }

    #[test]
    fn it_compare_hands() {
        assert_eq!(compare(&parse_hand("32T3K"), &parse_hand("22677")), Ordering::Less);
        assert_eq!(compare(&vec![3,3,3,3,2], &vec![2, 14, 14, 14, 14]), Ordering::Greater);
        assert_eq!(compare(&vec![7,7,8,8,8], &vec![7,7,7,8,8]), Ordering::Greater);
    }

    #[test]
    fn it_solve_example() {
        assert_eq!(winings(inputs::EXAMPLE), 5905);
    }

    #[test]
    fn it_solve_input() {
        assert_eq!(winings(inputs::INPUT), 248652697);
    }
}
