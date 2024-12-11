mod inputs;

pub fn  blink_one(stones: Vec<String>) -> Vec<String> {
    let mut new_stones = vec![];
    for stone in stones {
        if stone == "0" {
            new_stones.push(String::from("1"));
        }
        else if stone.len() % 2 == 0 {
            new_stones.push(stone[..stone.len() / 2].parse::<usize>().expect("error").to_string());
            new_stones.push(stone[stone.len() / 2..].parse::<usize>().expect("error").to_string());
        }
        else {
            let number: usize = stone.parse().expect("Not a number ?");
            new_stones.push((number * 2024).to_string());
        }
    }
    new_stones
}

pub fn blink(input: Vec<&str>, n: usize) -> usize {
    let mut stones = input.iter().map(|&s| String::from(s)).collect();
    for _ in 0..n {
        stones = blink_one(stones);
        println!("{:?}", stones)
    }
    stones.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(blink("125 17".split(' ').collect(), 25), 55312);
        assert_eq!(blink(inputs::INPUT.split(' ').collect(), 25), 220722);
    }
}
