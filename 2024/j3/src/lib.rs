mod inputs;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
}

pub fn compute_uncorrupted(input: &str) -> i32 {
    let mut sum = 0;
    for mult in RE.captures_iter(input) {
        let numbers: Vec<i32> = mult[1].split(',').map(|s| s.parse().expect("Failed to parse number")).collect();
        println!("{:?}", numbers);
        sum += numbers[0] * numbers[1];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(compute_uncorrupted(inputs::EXAMPLE), 161);
        assert_eq!(compute_uncorrupted(inputs::INPUT), 185797128);
    }
}
