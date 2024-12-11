mod inputs;

use std::collections::HashMap;

fn count_digits(n: usize) -> u32 {
    if n == 0 {
        return 1;
    }
    (n as f64).log10().floor() as u32 + 1
}

fn split_first(n: usize, pos: u32) -> usize {
    n / 10_usize.pow(pos)
}

fn split_last(n: usize, pos: u32) -> usize {
    n % 10_usize.pow(pos)
}

pub fn  blink_one(stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones = vec![];
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        }
        else if count_digits(stone) % 2 == 0 {
            let digit = count_digits(stone);
            new_stones.push(split_first(stone, digit/2));
            new_stones.push(split_last(stone, digit/2));
        }
        else {
            new_stones.push(stone * 2024);
        }
    }
    new_stones
}

pub fn blink_n(stone: usize, n: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    match cache.get(&(stone, n)) {
        Some(&n) => { n },
        _ => {
            let mut count = 1;
            if n > 0 {
                if stone == 0 {
                    count = blink_n(1, n - 1, cache)
                }
                else if count_digits(stone) % 2 == 0 {
                    let digit = count_digits(stone);
                    count = blink_n(split_first(stone, digit/2), n -1, cache) + 
                        blink_n(split_last(stone, digit/2), n -1, cache)
                }
                else {
                    count = blink_n(stone * 2024, n-1, cache)
                }
            }
            cache.insert((stone, n), count);
            count
        }
    }
}

pub fn blink(input: Vec<&str>, n: usize) -> usize {
    let stones: Vec<usize> = input.iter().map(|&s| s.parse::<usize>().expect("error")).collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let mut count = 0;
    for stone in stones {
        count += blink_n(stone, n, &mut cache)
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(blink("125 17".split(' ').collect(), 25), 55312);
        assert_eq!(blink(inputs::INPUT.split(' ').collect(), 75), 261952051690787);
    }
}
