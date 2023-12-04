mod inputs;

pub fn extract_numbers(numbers: &str) -> Vec<usize> {
    numbers.split_whitespace().map(|str| {
        str.parse::<usize>().unwrap()
    }).collect()
}

pub fn score_card(card: &str) -> usize {
    let number_of_match = winings(card) as u32;
    match number_of_match {
        0 => 0,
        _ => 2usize.pow(number_of_match - 1)
    }
}

pub fn winings(card: &str) -> usize {
    let numbers: Vec<Vec<usize>> = card.split(" | ").map(|numbers| extract_numbers(numbers)).collect();
    numbers[1].iter().filter(|n| numbers[0].contains(*n)).count() as usize
}

pub fn compute_point(cards: &str) -> usize {
    cards.lines().map(|c| score_card(c.split(": ").last().unwrap())).sum()
}

pub fn count_cards(index: i32, won_cards: usize, winings: &Vec<usize>) -> usize {
    // println!("{:?}", (index, won_cards));
    match won_cards {
        0 => 0,
        _ => {
            let from_slice = usize::try_from(index + 1).unwrap();
            winings[from_slice..(from_slice + won_cards)]
                .iter()
                .enumerate()
                .map(|(i, w)| 1 + count_cards((i + from_slice) as i32, *w, winings))
                .sum()
        }
    }
}

pub fn solve_p2(cards: &str) -> usize {
    let winings: Vec<usize> = cards.lines().map(|c| winings(c.split(": ").last().unwrap())).collect();
    count_cards(-1, winings.iter().count(), &winings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve_example_p1() {
        assert_eq!(compute_point(inputs::EXAMPLE), 13);
    }

    #[test]
    fn it_solve_count_copy_card() {
        let winings: Vec<usize> = inputs::EXAMPLE.lines().map(|c| winings(c.split(": ").last().unwrap())).collect();
        println!("{:?}", winings);
        assert_eq!(count_cards(3, winings[3], &winings), 1);
        assert_eq!(count_cards(4, winings[4], &winings), 0);
        assert_eq!(count_cards(2, winings[2], &winings), 3);
    }

    #[test]
    fn it_solve_example_p2() {
        assert_eq!(solve_p2(inputs::EXAMPLE), 30);
    }

    #[test]
    fn it_solve_input_p1() {
        assert_eq!(compute_point(inputs::INPUT), 21138);
    }

    #[test]
    fn it_solve_p2() {
        assert_eq!(solve_p2(inputs::INPUT), 7185540);
    }
}
