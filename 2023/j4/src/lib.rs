mod inputs;

pub fn extract_numbers(numbers: &str) -> Vec<u32> {
    numbers.split_whitespace().map(|str| {
        str.parse::<u32>().unwrap()
    }).collect()
}

pub fn score_card(card: &str) -> u32 {
    let numbers: Vec<Vec<u32>> = card.split(" | ").map(|numbers| extract_numbers(numbers)).collect();
    let number_of_match = numbers[1].iter().filter(|n| numbers[0].contains(*n)).count() as u32;
    match number_of_match {
        0 => 0,
        _ => 2u32.pow(number_of_match - 1)
    }
}

pub fn compute_point(cards: &str) -> u32 {
    cards.lines().map(|c| score_card(c.split(": ").last().unwrap())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve_example() {
        assert_eq!(compute_point(inputs::EXAMPLE), 13);
    }

    #[test]
    fn it_solve_input() {
        assert_eq!(compute_point(inputs::INPUT), 21138);
    }
}
