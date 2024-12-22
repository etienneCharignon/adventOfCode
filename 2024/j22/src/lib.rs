mod inputs;
use std::collections::HashMap;
use multimap::MultiMap;
use std::cmp::max;

pub fn mix_prune(secret: u64, number: u64) -> u64 {
    let mix = secret ^ number;
    mix % 16777216
}

pub fn evolve_secret(secret: u64) -> u64 {
    let mut new_secret = mix_prune(secret, secret * 64);
    new_secret = mix_prune(new_secret, new_secret / 32);
    mix_prune(new_secret, new_secret * 2048)
}

pub fn generate_secrets(n: u64, initial_number: u64) -> Vec<u64> {
    let mut secret = initial_number;
    let mut secrets = vec![initial_number];
    for _ in 0..n {
        secret = evolve_secret(secret);
        secrets.push(secret);
    }
    secrets
}

pub fn sum_secrets(input: &str) -> u64 {
    input.split("\n").map(|s| s.parse().expect("Failed to parse number"))
        .map(|initial_number| generate_secrets(2000, initial_number)[2000])
        .sum()
}

pub fn read(input:&str) -> Vec<u64> {
    input.split("\n").map(|s| s.parse().expect("Failed to parse number")).collect()
}

pub fn sum_bananas(byers_secrets: Vec<u64>) -> u64 {
    let mut global_delta_banana = MultiMap::new();
    for secret in byers_secrets {
        let mut delta_banana = HashMap::<Vec<i16>, u64>::new();
        let prices: Vec<u64> = generate_secrets(2000, secret).iter().map(|s| (s%10) as u64).collect();
        let deltas: Vec<i16> = prices[1..].iter().enumerate().map(|(i, p)| {
            *p as i16 - prices[i] as i16
        }).collect();
        for i in 0..(deltas.len() - 4) {
            let sequence = deltas[i..(i + 4)].to_vec();
            if ! delta_banana.contains_key(&sequence) {
                delta_banana.insert(sequence, prices[i + 4]);
            }
        }
        for (delta, banana) in delta_banana {
            global_delta_banana.insert(delta, banana);
        }
    }
    let mut max_banana = 0;
    for (sequence, bananas) in global_delta_banana.iter_all() {
        let sum = bananas.iter().sum();
        max_banana = max(max_banana, sum);
    }
    max_banana
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_evolve_a_secret() {
        assert_eq!(evolve_secret(123), 15887950);
    }

    #[test]
    fn it_work_p1() {
        assert_eq!(sum_secrets(inputs::EXAMPLE), 37327623);
        assert_eq!(sum_secrets(inputs::INPUT), 17577894908);
    }

    #[test]
    fn it_work_p2() {
        assert_eq!(sum_bananas(read(inputs::EXAMPLE2)), 23);
        assert_eq!(sum_bananas(read(inputs::INPUT)), 1931);
    }
}
