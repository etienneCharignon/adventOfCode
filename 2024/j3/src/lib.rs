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

pub fn compute_uncorrupted_but_dont(input:&str) ->i32 {
    let mut sum = 0;
    match input.find("don't()") {
        Some(donot_index) => {
            let enabled = &input[..donot_index];
            sum += compute_uncorrupted(enabled);
            let following = &input[donot_index..];
            match following.find("do()") {
                Some(do_index) => {
                    sum += compute_uncorrupted_but_dont(&following[do_index..]);
                },
                _ => {}
            }
        },
        _ => {
            sum += compute_uncorrupted(input);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        assert_eq!(compute_uncorrupted(inputs::EXAMPLE), 161);
        assert_eq!(compute_uncorrupted(inputs::INPUT), 185797128);
    }

    #[test]
    fn it_works_part2() {
        assert_eq!(compute_uncorrupted_but_dont(inputs::EXAMPLE), 48);
        assert_eq!(compute_uncorrupted_but_dont(inputs::INPUT), 89798695);
    }
}
