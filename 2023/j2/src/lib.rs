mod inputs;

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Game (\d*): (.*)").unwrap();
    static ref RED: Regex = Regex::new(r"(\d*) red").unwrap();
    static ref GREEN: Regex = Regex::new(r"(\d*) green").unwrap();
    static ref BLUE: Regex = Regex::new(r"(\d*) blue").unwrap();
    static ref COULEURS: Regex = Regex::new(r"(\d*) (blue|green|red)").unwrap();
    static ref MAX_COULEURS: HashMap<&'static str, u32> = HashMap::from([ 
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
}

pub fn read_color(re: &Regex, set: &str) -> u32 {
    if re.is_match(set) {
       let cap: regex::Captures = re.captures(set).unwrap();
        cap[1].parse().unwrap()
    }
    else {
        0
    }
}

pub fn read_set(set: &str) -> (u32, u32, u32) {
    (read_color(&RED, set), read_color(&GREEN, set), read_color(&BLUE, set))
}

pub fn score_game(game: &str) -> u32 {
    let game_cap = RE.captures(game).unwrap();
    let game_id: u32 = game_cap[1].parse().unwrap();
    if COULEURS.captures_iter(&game_cap[2]).all(|c| &c[1].parse::<u32>().unwrap() <= MAX_COULEURS.get(&c[2]).unwrap()) { game_id } else { 0 }
}

pub fn power(sets: Vec<(u32, u32, u32)>) -> u32 {
    let (mr, mv, mb) = sets.iter()
       .fold((0, 0, 0), |(mr, mv, mb), (r, v, b)| (cmp::max(mr, *r), cmp::max(mv, *v), cmp::max(mb, *b)));
    mr * mv * mb
}

pub fn power_game(game: &str) -> u32 {
    let game_cap = RE.captures(game).unwrap();
    let sets: Vec<(u32, u32, u32)> =
        game_cap[2]
        .split("; ")
        .map(|set| read_set(set))
        .collect();
    power(sets)
}

pub fn count_games(input: &str) -> u32 {
    input.lines().map(|game| score_game(game)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solve_example() {
        assert_eq!(count_games(inputs::EXAMPLE), 8);
    }

    #[test]
    fn it_solve() {
        assert_eq!(count_games(inputs::INPUT), 2105);
    }
}
